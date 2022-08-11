use ricq_core::command::common::PbToBytes;
use ricq_core::protocol::packet::Packet;
use tracing::error;
use ricq_core::RQResult;
use crate::client::decoder::Decoder;
use crate::protocol::protobuf;
use crate::protocol::protobuf::FirstViewMsg;

impl super::GuildClient {
    pub async fn process_packet(&self, pkt: Packet) {

    }

    pub async fn send_sync_first_view_and_wait(&self, pkt: Packet) -> RQResult<FirstViewMsg> {
        static COMMAND: &str = "trpc.group_pro.synclogic.SyncLogic.PushFirstView";

        let mut rx = self
            .rq_client
            .listen_command(COMMAND)
            .await;

        let mut first_view: FirstViewMsg;
        let r = rx.recv().await.unwrap();
        first_view = Decoder.decode_first_view_msg(r.body)?;
        
        for _ in 0..2 {
            let r = rx.recv().await.unwrap();
            let msg = Decoder.decode_first_view_msg(r.body)?;

            match msg {
                FirstViewMsg { guild_nodes, .. } if !guild_nodes.is_empty() => {
                    first_view.guild_nodes = guild_nodes;
                }
                FirstViewMsg { channel_msgs, .. } if !channel_msgs.is_empty() => {
                    first_view.channel_msgs = channel_msgs;
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
    }
}
