use std::sync::Arc;

use crate::client::engine::decoder::config_push_svc::decode_push_req_packet;
use crate::client::engine::decoder::online_push::decode_group_message_packet;
use crate::client::protocol::packet::Packet;

pub mod config_push_svc;
pub mod online_push;
pub mod reg_prxy_svc;
pub mod wtlogin;

impl super::Client {
    pub async fn process_income_packet(self: Arc<Self>, pkt: Packet) {
        // response
        if let Some(sender) = self.packet_promises.write().await.remove(&pkt.seq_id) {
            sender.send(pkt).unwrap(); //todo response
            return;
        }

        if let Some(tx) = self.packet_waiters.write().await.remove(&pkt.command_name) {
            tx.send(pkt).unwrap();
            return;
        }

        tokio::spawn(async move {
            match pkt.command_name.as_ref() {
                "OnlinePush.PbPushGroupMsg" => {
                    let p = decode_group_message_packet(&pkt.body).unwrap();
                    if let Err(e) = self.process_group_message_part(p).await {
                        tracing::error!("process group message part error: {:?}", e);
                    }
                }
                "ConfigPushSvc.PushReq" => {
                    let req = decode_push_req_packet(&pkt.body).unwrap();
                    if let Err(e) = self.process_config_push_req(req).await {
                        tracing::error!("process config push req error: {:?}", e);
                    }
                }
                "RegPrxySvc.PushParam" => {
                    let other_clients = self
                        .engine
                        .read()
                        .await
                        .decode_push_param_packet(&pkt.body)
                        .unwrap();
                    if let Err(e) = self.process_push_param(other_clients).await {
                        tracing::error!("process push param error: {:?}", e);
                    }
                }
                _ => {
                    println!("unhandled pkt: {}", &pkt.command_name);
                }
            }
        });
    }
}
