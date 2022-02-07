use std::sync::Arc;

use bytes::Bytes;

use crate::engine::protocol::packet::Packet;

pub mod c2c;
pub mod config_push_svc;
pub mod message_svc;
pub mod online_push;
pub mod reg_prxy_svc;
pub mod wtlogin;

macro_rules! log_error {
    ($process: expr, $info: expr) => {
        if let Err(e) = $process {
            tracing::error!(target: "rs_qq", $info, e);
        }
    };
}

impl super::Client {
    pub async fn process_income_packet(self: &Arc<Self>, pkt: Packet) {
        tracing::trace!(target: "rs_qq", "received pkt: {}", &pkt.command_name);
        // response
        {
            if let Some(sender) = self.packet_promises.write().await.remove(&pkt.seq_id) {
                sender.send(pkt).unwrap(); //todo response
                return;
            }
        }
        tracing::trace!(target: "rs_qq", "pkt: {} passed packet_promises", &pkt.command_name);
        {
            if let Some(tx) = self.packet_waiters.write().await.remove(&pkt.command_name) {
                tx.send(pkt).unwrap();
                return;
            }
        }
        tracing::trace!(target: "rs_qq", "pkt: {} passed packet_waiters", &pkt.command_name);

        let cli = self.clone();
        tokio::spawn(async move {
            match pkt.command_name.as_ref() {
                "OnlinePush.PbPushGroupMsg" => {
                    let p = cli
                        .engine
                        .read()
                        .await
                        .decode_group_message_packet(pkt.body)
                        .unwrap();
                    log_error!(
                        cli.process_group_message_part(p).await,
                        "process group message part error: {:?}"
                    )
                }
                "ConfigPushSvc.PushReq" => {
                    let req = cli
                        .engine
                        .read()
                        .await
                        .decode_push_req_packet(pkt.body)
                        .unwrap();
                    log_error!(
                        cli.process_config_push_req(req).await,
                        "process config push req error: {:?}"
                    )
                }
                "RegPrxySvc.PushParam" => {
                    let other_clients = cli
                        .engine
                        .read()
                        .await
                        .decode_push_param_packet(&pkt.body)
                        .unwrap();
                    log_error!(
                        cli.process_push_param(other_clients).await,
                        "process push param error: {:?}"
                    )
                }
                "MessageSvc.PushNotify" => {
                    // c2c流程：
                    // 1. Server 发送 PushNotify 到 Client, 表示有通知需要 Client 拉取 (不带具体内容)
                    // 2. Client 根据 msg_type 发送请求拉取具体通知内容
                    // 类型：好友申请、群申请、私聊消息、其他?
                    let notify = cli
                        .engine
                        .read()
                        .await
                        .decode_svc_notify(pkt.body)
                        .unwrap_or_default();
                    if let Some(notify) = notify {
                        match notify.msg_type {
                            35 | 36 | 37 | 45 | 46 | 84 | 85 | 86 | 87 => {
                                // pull group system msg(group request), then process
                                match cli.get_all_group_system_messages().await {
                                    Ok(msgs) => {
                                        cli.process_group_system_messages(msgs).await;
                                    }
                                    Err(err) => {
                                        tracing::warn!(
                                            "failed to get group system message {}",
                                            err
                                        );
                                    }
                                }
                            }
                            187 | 188 | 189 | 190 | 191 => {
                                // pull friend system msg(friend request), then process
                                match cli.get_friend_system_messages().await {
                                    Ok(msgs) => {
                                        cli.process_friend_system_messages(msgs).await;
                                    }
                                    Err(err) => {
                                        tracing::warn!(
                                            "failed to get friend system message {}",
                                            err
                                        );
                                    }
                                }
                            }
                            _ => {
                                // TODO tracing.warn!()
                            }
                        }
                    }
                    // pull private msg and other, then process
                    match cli.sync_all_message().await {
                        Ok(msgs) => {
                            log_error!(
                                cli.process_message_sync(msgs).await,
                                "process message sync error: {:?}"
                            )
                        }
                        Err(err) => {
                            tracing::warn!("failed to sync message {}", err);
                        }
                    }
                }
                "OnlinePush.ReqPush" => {
                    let engine = cli.engine.read().await;
                    let resp = engine.decode_online_push_req_packet(pkt.body).unwrap();
                    let _ = cli
                        .send(engine.build_delete_online_push_packet(
                            resp.resp.uin,
                            0,
                            Bytes::new(),
                            pkt.seq_id as u16,
                            resp.resp.msg_infos,
                        ))
                        .await;
                    tracing::warn!(target: "rs_qq", "unhandled OnlinePush.ReqPush");
                    //todo
                }
                _ => {
                    tracing::warn!(target: "rs_qq", "unhandled pkt: {}", &pkt.command_name);
                }
            }
        });
    }
}
