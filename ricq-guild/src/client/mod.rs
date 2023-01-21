use dynamic_protobuf::dynamic_message;
use std::collections::HashMap;
use std::ops::Deref;
use std::sync::Arc;

use tokio::sync::{broadcast, RwLockReadGuard};
use tokio::task::JoinHandle;

use ricq::structs::ImageInfo;
use ricq_core::highway::BdhInput;

use ricq_core::msg::MessageChain;
use ricq_core::protocol::packet::Packet;
use ricq_core::{RQError, RQResult};

use crate::client::decoder::Decoder;
use crate::protocol::protobuf::FirstViewMsg;
use crate::protocol::{
    protobuf, FirstView, FirstViewMessage, GuildImage, GuildImageStoreResp, GuildSelfProfile,
};

pub mod builder;
pub mod decoder;
pub mod processor;

#[allow(dead_code)]
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

            let r = rx.recv().await.unwrap();
            let mut first_view: FirstViewMsg = Decoder.decode_first_view_msg(r.body)?;

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

        let first_view_msg = first_view.await.unwrap()?;
        let first_view_rsp = Decoder.decode_guild_first_view_response(rsp.body)?;

        let opt = match (first_view_msg, first_view_rsp) {
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

    pub async fn upload_channel_image(
        &self,
        guild_id: u64,
        channel_id: u64,
        image: &[u8],
    ) -> RQResult<GuildImage> {
        let info = ImageInfo::try_new(&image)?;

        let image_store = self
            .get_guild_image_store(guild_id, channel_id, &image)
            .await?;

        let fid;
        let dn_index;
        let server;
        match image_store {
            GuildImageStoreResp::Exist {
                file_id,
                mut addrs,
                download_index,
            } => {
                fid = file_id;
                dn_index = download_index;
                server = addrs.pop().ok_or(RQError::EmptyField("Address"))?;
            }
            GuildImageStoreResp::NotExist {
                file_id,
                upload_key,
                mut upload_addrs,
                download_index,
            } => {
                let addr = match self.rq_client.highway_addrs.read().await.first() {
                    Some(addr) => addr.clone(),
                    None => upload_addrs
                        .pop()
                        .ok_or(RQError::EmptyField("upload_addrs"))?,
                };

                self.rq_client
                    .highway_upload_bdh(
                        addr.clone().into(),
                        BdhInput {
                            command_id: 83,
                            ticket: upload_key,
                            ext: dynamic_message! {
                                11 => guild_id,
                                12 => channel_id,
                            }
                            .encode()
                            .to_vec(),
                            encrypt: false,
                            chunk_size: 256 * 1024,
                            send_echo: true,
                        },
                        image,
                    )
                    .await?;

                fid = file_id;
                dn_index = download_index;
                server = addr;
            }
        };

        let guild_image = GuildImage {
            file_id: fid,
            file_name: info.filename,
            size: info.size,
            width: info.width,
            height: info.height,
            image_type: info.image_type,
            download_index: dn_index,
            md5: info.md5,
            server_ip: server.0,
            server_port: server.1,
        };

        Ok(guild_image)
    }

    pub async fn get_guild_image_store(
        &self,
        guild_id: u64,
        channel_id: u64,
        data: &[u8],
    ) -> RQResult<GuildImageStoreResp> {
        let image_info = ImageInfo::try_new(data)?;
        let req = self.engine().await.build_guild_image_store_packet(
            channel_id as _,
            guild_id,
            image_info.filename,
            image_info.md5,
            image_info.size as u64,
            image_info.width,
            image_info.height,
            image_info.image_type as u32,
        );
        let resp = self.rq_client.send_and_wait(req).await?;
        Decoder.decode_guild_image_store_response(resp.body)
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
