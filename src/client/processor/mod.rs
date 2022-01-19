use std::sync::Arc;

use crate::engine::protocol::packet::Packet;

pub mod config_push_svc;
pub mod message_svc;
pub mod online_push;
pub mod reg_prxy_svc;
pub mod wtlogin;

impl super::Client {
    pub async fn process_income_packet(self: Arc<Self>, pkt: Packet) {
        tracing::trace!("received pkt: {}", &pkt.command_name);
        // response
        {
            if let Some(sender) = self.packet_promises.write().await.remove(&pkt.seq_id) {
                sender.send(pkt).unwrap(); //todo response
                return;
            }
        }
        tracing::trace!("pkt: {} passed packet_promises", &pkt.command_name);
        {
            if let Some(tx) = self.packet_waiters.write().await.remove(&pkt.command_name) {
                tx.send(pkt).unwrap();
                return;
            }
        }
        tracing::trace!("pkt: {} passed packet_waiters", &pkt.command_name);

        tokio::spawn(async move {
            match pkt.command_name.as_ref() {
                "OnlinePush.PbPushGroupMsg" => {
                    let p = self
                        .engine
                        .read()
                        .await
                        .decode_group_message_packet(pkt.body)
                        .unwrap();
                    if let Err(e) = self.process_group_message_part(p).await {
                        tracing::error!("process group message part error: {:?}", e);
                    }
                }
                "ConfigPushSvc.PushReq" => {
                    let req = self
                        .engine
                        .read()
                        .await
                        .decode_push_req_packet(pkt.body)
                        .unwrap();
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
                "MessageSvc.PushNotify" => {
                    let _notify_msg_type = self
                        .engine
                        .read()
                        .await
                        .decode_svc_notify(pkt.body)
                        .unwrap();
                    let _ = self
                        .send_and_wait(self.engine.read().await.build_get_message_request_packet(0))
                        .await;
                }
                "MessageSvc.PbGetMsg" => {
                    let resp = self
                        .engine
                        .read()
                        .await
                        .decode_message_svc_packet(pkt.body)
                        .unwrap();
                    self.process_message_sync(resp).await.unwrap();
                }
                _ => {
                    println!("unhandled pkt: {}", &pkt.command_name);
                }
            }
        });
    }
}
