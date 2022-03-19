use bytes::Bytes;
use rq_engine::common::RQAddr;

use crate::engine::command::config_push_svc::ConfigPushBody;

use crate::client::Client;
use crate::engine::command::config_push_svc::ConfigPushReq;
use crate::RQError;

impl Client {
    pub(crate) async fn process_config_push_req(
        &self,
        config_push_req: ConfigPushReq,
    ) -> Result<(), RQError> {
        // send response to server
        let resp = config_push_req.resp;
        let response = self.engine.read().await.build_conf_push_resp_packet(
            resp.t,
            resp.pkt_seq,
            resp.jce_buf,
        );
        self.send(response).await?;
        match config_push_req.body {
            ConfigPushBody::Unknown => {}
            ConfigPushBody::SsoServers { .. } => {}
            ConfigPushBody::FileStorageInfo { info: _, rsp_body } => {
                let mut session = self.highway_session.write().await;
                if let Some(rsp_body) = rsp_body {
                    session.sig_session = Bytes::from(rsp_body.sig_session.unwrap_or_default());
                    session.session_key = Bytes::from(rsp_body.session_key.unwrap_or_default());
                    session.uin = self.uin().await;
                    session.app_id = self.engine.read().await.transport.version.app_id as i32;
                    let mut highway_addrs = self.highway_addrs.write().await;
                    rsp_body.addrs.into_iter().for_each(|addr| {
                        let service_type = addr.service_type.unwrap_or_default();
                        if service_type == 10 {
                            let addrs: Vec<RQAddr> = addr
                                .addrs
                                .into_iter()
                                .map(|addr| {
                                    RQAddr(
                                        addr.ip.unwrap_or_default(),
                                        addr.port.unwrap_or_default() as u16,
                                    )
                                })
                                .collect();
                            highway_addrs.extend(addrs);
                        } else if service_type == 11 {
                            // TODO
                        }
                    })
                }
            }
        }
        // TODO process
        Ok(())
    }
}
