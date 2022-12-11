use std::sync::Arc;

use bytes::Bytes;

use ricq_core::protocol::packet::Packet;

pub mod c2c;
pub mod config_push_svc;
pub mod message_svc;
pub mod online_push;
pub mod reg_prxy_svc;
pub mod stat_svc;
pub mod wtlogin;

macro_rules! log_error {
    ($process: expr, $info: expr) => {
        if let Err(e) = $process {
            tracing::error!($info, e);
        }
    };
}

impl super::Client {
    /// 接收到的 Packet 统一分发
    pub async fn process_income_packet(self: &Arc<Self>, pkt: Packet) {
        tracing::trace!("received pkt: {}", &pkt.command_name);
        // response, send_and_wait 的包将会在此被截流
        {
            if let Some(sender) = self.packet_promises.write().await.remove(&pkt.seq_id) {
                sender.send(pkt).unwrap();
                return;
            }
        }

        tracing::trace!("pkt: {} passed packet_promises", &pkt.command_name);

        {
            if let Some(handler) = self.packet_handler.read().await.get(&pkt.command_name) {
                let _ = handler.send(pkt.clone());
            }
        }

        let cli = self.clone();
        tokio::spawn(async move {
            match pkt.command_name.as_ref() {
                "OnlinePush.PbPushGroupMsg" => {
                    let p = cli
                        .engine
                        .read()
                        .await
                        .decode_group_message_packet(pkt.body);
                    match p {
                        Ok(part) => {
                            log_error!(
                                cli.process_group_message_part(part).await,
                                "process_group_message_part error: {:?}"
                            )
                        }
                        Err(err) => {
                            tracing::warn!("failed to decode [OnlinePush.PbPushGroupMsg]: {}", err);
                        }
                    }
                }
                "ConfigPushSvc.PushReq" => {
                    let req = cli.engine.read().await.decode_push_req_packet(pkt.body);
                    match req {
                        Ok(req) => {
                            log_error!(
                                cli.process_config_push_req(req).await,
                                "process_config_push_req error: {:?}"
                            )
                        }
                        Err(err) => {
                            tracing::warn!("failed to decode [ConfigPushSvc.PushReq]: {}", err);
                        }
                    }
                }
                "RegPrxySvc.PushParam" => {
                    let other_clients = cli.engine.read().await.decode_push_param_packet(&pkt.body);
                    match other_clients {
                        Ok(other_clients) => {
                            log_error!(
                                cli.process_push_param(other_clients).await,
                                "process_push_param error: {:?}"
                            )
                        }
                        Err(err) => {
                            tracing::warn!("failed to decode [RegPrxySvc.PushParam]: {}", err);
                        }
                    }
                }
                #[cfg(not(feature = "no-svc-notify"))]
                "MessageSvc.PushNotify" => {
                    // c2c流程：
                    // 1. Server 发送 PushNotify 到 Client, 表示有通知需要 Client 拉取 (不带具体内容)
                    // 2. Client 根据 msg_type 发送请求拉取具体通知内容
                    // 类型：好友申请、群申请、私聊消息、其他?
                    let resp = cli.engine.read().await.decode_svc_notify(pkt.body);
                    match resp {
                        Ok(notify) => {
                            cli.process_push_notify(notify).await;
                        }
                        Err(err) => {
                            tracing::warn!("failed to decode [MessageSvc.PushNotify]: {}", err);
                        }
                    }
                }
                "OnlinePush.ReqPush" => {
                    let resp = cli
                        .engine
                        .read()
                        .await
                        .decode_online_push_req_packet(pkt.body);
                    match resp {
                        Ok(resp) => {
                            log_error!(
                                cli.delete_online_push(
                                    resp.uin,
                                    0,
                                    Bytes::new(),
                                    pkt.seq_id as u16,
                                    resp.msg_infos.clone(),
                                )
                                .await,
                                "delete_online_push error: {:?}"
                            );
                            cli.process_push_req(resp.msg_infos).await;
                        }
                        Err(err) => {
                            tracing::warn!("failed to decode [OnlinePush.ReqPush]: {}", err);
                        }
                    }
                }
                "OnlinePush.PbPushTransMsg" => {
                    let online_push_trans = cli
                        .engine
                        .read()
                        .await
                        .decode_online_push_trans_packet(pkt.body);
                    match online_push_trans {
                        Ok(online_push_trans) => {
                            cli.process_push_trans(online_push_trans).await;
                        }
                        Err(err) => {
                            tracing::warn!("failed to decode [OnlinePush.PbPushTransMsg]: {}", err);
                        }
                    }
                }
                "MessageSvc.PushForceOffline" => {
                    let offline = cli.engine.read().await.decode_force_offline(pkt.body);
                    match offline {
                        Ok(offline) => {
                            cli.process_push_force_offline(offline).await;
                        }
                        Err(err) => {
                            tracing::warn!(
                                "failed to decode [MessageSvc.PushForceOffline]: {}",
                                err
                            );
                        }
                    }
                }
                "StatSvc.ReqMSFOffline" => {
                    let offline = cli.engine.read().await.decode_msf_force_offline(pkt.body);
                    match offline {
                        Ok(offline) => {
                            cli.process_msf_force_offline(offline).await;
                        }
                        Err(err) => {
                            tracing::warn!("failed to decode [StatSvc.ReqMSFOffline]: {}", err);
                        }
                    }
                }
                #[cfg(not(feature = "no-c2c-sync"))]
                "OnlinePush.PbC2CMsgSync" => {
                    // 其他设备发送消息，同步
                    let push = cli.engine.read().await.decode_c2c_sync_packet(pkt.body);
                    match push {
                        Ok(push) => {
                            log_error!(
                                cli.process_c2c_sync(pkt.seq_id, push).await,
                                "process_c2c_sync error: {:?}"
                            )
                        }
                        Err(err) => {
                            tracing::warn!("failed to decode [OnlinePush.PbC2CMsgSync]: {}", err);
                        }
                    }
                }
                "OnlinePush.SidTicketExpired" => {
                    log_error!(
                        cli.process_sid_ticket_expired(pkt.seq_id).await,
                        "process_sid_ticket_expired error: {:?}"
                    )
                }
                "RegPrxySvc.GetMsgV2"
                | "RegPrxySvc.PbGetMsg"
                | "RegPrxySvc.NoticeEnd"
                | "MessageSvc.PushReaded" => {
                    tracing::trace!("ignore pkt: {}", &pkt.command_name);
                }
                _ => {
                    tracing::debug!("unhandled pkt: {}", &pkt.command_name);
                }
            }
        });
    }
}
