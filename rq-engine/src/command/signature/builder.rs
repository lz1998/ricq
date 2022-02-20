use crate::command::common::PbToBytes;
use crate::pb::sig_act;
use crate::protocol::packet::Packet;

impl super::super::super::Engine {
    pub fn build_set_signature_packet(&self, _signature: String) -> Packet {
        let req = sig_act::ReqBody {
            cmd: Some(2),
            seq: Some(chrono::Utc::now().timestamp_millis() as u64),
            plf: Some(sig_act::Platform {
                platform: Some(109),
                osver: None,
                mqqver: Some(self.transport.version.sort_version_name.into()),
            }),
            auth_req: Some(sig_act::SigauthReq {
                uin_disable: Some(self.uin() as u64),
                itemid: Some(0),
                len: None,
                data: None,
                fontid: Some(0),
            }),
            source: Some(1),
            ..Default::default()
        };
        self.uni_packet("Signature.auth", req.to_bytes())
    }
}
