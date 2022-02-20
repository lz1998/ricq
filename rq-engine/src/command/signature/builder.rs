use crate::command::common::PbToBytes;
use crate::pb::sig_act;
use crate::protocol::packet::Packet;

impl super::super::super::Engine {
    pub fn build_update_signature_packet(&self, signature: String) -> Packet {
        let req = sig_act::ReqBody {
            cmd: Some(2),
            seq: Some(chrono::Utc::now().timestamp_millis() as u64),
            plf: Some(sig_act::Platform {
                platform: Some(109),
                osver: Some(self.transport.device.version.release.to_owned()),
                mqqver: Some(self.transport.version.sort_version_name.into()),
            }),
            auth_req: Some(sig_act::SigauthReq {
                uin_disable: Some(self.uin() as u64),
                itemid: Some(0),
                len: Some(signature.len() as i32 + 27),
                data: Some({
                    let mut buf = vec![0x3, signature.as_bytes().len() as u8 + 1, 0x20];
                    buf.extend(signature.into_bytes());
                    buf.extend([
                        0x91, 0x04, 0x00, 0x00, 0x00, 0x00, 0x92, 0x04, 0x00, 0x00, 0x00, 0x00,
                        0xA2, 0x04, 0x00, 0x00, 0x00, 0x00, 0xA3, 0x04, 0x00, 0x00, 0x00, 0x00,
                    ]);
                    buf
                }),
                fontid: Some(0),
            }),
            source: Some(1),
            ..Default::default()
        };
        self.uni_packet("Signature.auth", req.to_bytes())
    }
}
