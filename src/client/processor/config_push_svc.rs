use crate::client::income::decoder::config_push_svc::ConfigPushReq;
use crate::client::Client;

impl Client {
    pub async fn process_config_push_req(&self, config_push_req: ConfigPushReq) {
        // send response to server
        let resp = config_push_req.resp;
        self.send(
            self.build_conf_push_resp_packet(resp.t, resp.pkt_seq, resp.jce_buf)
                .await
                .into(),
        )
        .await;
        // TODO process
    }
}
