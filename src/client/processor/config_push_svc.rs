use crate::client::engine::command::config_push_svc::ConfigPushReq;
use crate::client::Client;
use crate::RQError;

impl Client {
    pub async fn process_config_push_req(
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
        // TODO process
        Ok(())
    }
}
