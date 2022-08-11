use crate::client::decoder::Decoder;
use crate::protocol::protobuf::FirstViewMsg;
use crate::protocol::{protobuf, FirstView, FirstViewMessage, GuildSelfProfile, GuildUserProfile};
use ricq_core::msg::MessageChain;
use ricq_core::protocol::packet::Packet;
use ricq_core::RQResult;
use std::collections::HashMap;
use std::ops::Deref;
use std::sync::Arc;
use tokio::sync::{broadcast, RwLockReadGuard};
use tokio::task::JoinHandle;

pub mod builder;
pub mod decoder;
pub mod processor;

pub struct GuildClient {
    rq_client: Arc<ricq::Client>,
    listeners: HashMap<&'static str, broadcast::Receiver<Packet>>,
}

impl GuildClient {
    pub async fn new(rq_client: &Arc<ricq::Client>) -> Self {
        let rq_client = rq_client.clone();

        let listeners = HashMap::new();

        Self {
            rq_client,
            listeners,
        }
    }

    pub async fn engine(&self) -> Engine<'_> {
        Engine::from_rq(self.rq_client.engine.read().await)
    }

    pub async fn fetch_guild_first_view(&self) -> RQResult<Option<FirstView>> {
        let pkt = self.engine().await.build_sync_channel_first_view_packet();

        let cli = self.rq_client.clone();
        let first_view: JoinHandle<RQResult<FirstViewMsg>> = tokio::spawn(async move {
            static COMMAND: &str = "trpc.group_pro.synclogic.SyncLogic.PushFirstView";

            let mut rx = cli.listen_command(COMMAND).await;

            let mut first_view: FirstViewMsg;
            let r = rx.recv().await.unwrap();
            first_view = Decoder.decode_first_view_msg(r.body)?;

            for _ in 0..2 {
                let r = rx.recv().await.unwrap();
                let msg = Decoder.decode_first_view_msg(r.body)?;

                match msg {
                    FirstViewMsg {
                        push_flag,
                        channel_msgs,
                        get_msg_time,
                        ..
                    } if !channel_msgs.is_empty() => {
                        first_view.push_flag = push_flag;
                        first_view.channel_msgs = channel_msgs;
                        first_view.get_msg_time = get_msg_time;
                    }
                    FirstViewMsg {
                        direct_message_guild_nodes,
                        ..
                    } if !direct_message_guild_nodes.is_empty() => {
                        first_view.direct_message_guild_nodes = direct_message_guild_nodes;
                    }
                    _ => {}
                }
            }
            Ok(first_view)
        });

        let rsp = self.rq_client.send_and_wait(pkt).await?;

        let first_view_rsp = Decoder.decode_guild_first_view_response(rsp.body)?;

        let opt = match (first_view.await.unwrap()?, first_view_rsp) {
            (
                FirstViewMsg {
                    push_flag: Some(push_flag),
                    guild_nodes,
                    channel_msgs,
                    get_msg_time: Some(get_msg_time),
                    direct_message_guild_nodes,
                    ..
                },
                Some(response),
            ) => {
                let message = FirstViewMessage {
                    push_flag,
                    guild_nodes,
                    channel_msgs,
                    get_msg_time,
                    direct_message_guild_nodes,
                };

                Some(FirstView { response, message })
            }
            _ => None,
        };

        Ok(opt)
    }

    pub async fn fetch_guild_self_profile(
        &self,
        tiny_id: u64,
    ) -> RQResult<Option<GuildSelfProfile>> {
        let pkt = self.engine().await.build_get_user_profile_packet(tiny_id);
        let rsp = self.rq_client.send_and_wait(pkt).await?;
        let usr = Decoder.decode_guild_user_profile(rsp.body)?;

        let prof = match usr {
            Some(protobuf::GuildUserProfile {
                tiny_id: Some(tiny_id),
                nickname: Some(nickname),
                avatar_url: Some(avatar_url),
                ..
            }) => Some(GuildSelfProfile {
                tiny_id,
                nickname,
                avatar_url,
            }),
            _ => None,
        };

        Ok(prof)
    }

    pub async fn send_channel_message(
        &self,
        elems: MessageChain,
        guild_id: u64,
        channel_id: u64,
    ) -> RQResult<Packet> {
        let pkt = self.engine().await.build_send_channel_message_packet(
            elems.into(),
            guild_id,
            channel_id,
        );

        let ret = self.rq_client.send_and_wait(pkt).await?;

        Ok(ret) // todo: decode receipt
    }
}

pub struct Engine<'a>(RwLockReadGuard<'a, ricq_core::Engine>);

impl<'a> Engine<'a> {
    fn from_rq(engine: RwLockReadGuard<'a, ricq_core::Engine>) -> Self {
        Self(engine)
    }
}

impl<'a> Deref for Engine<'a> {
    type Target = ricq_core::Engine;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
