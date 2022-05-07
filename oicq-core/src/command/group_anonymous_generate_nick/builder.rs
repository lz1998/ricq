use crate::command::common::PbToBytes;
use crate::pb;
use crate::protocol::packet::Packet;

impl super::super::super::Engine {
    // group_anonymous_generate_nick.group
    pub fn build_get_anony_info_request(&self, group_code: i64) -> Packet {
        let req = pb::cmd0x3bb::AnonyMsg {
            cmd: Some(1),
            anony_req: Some(pb::cmd0x3bb::C3bbReqBody {
                uin: Some(self.uin() as u64),
                group_code: Some(group_code as u64),
            }),
            ..Default::default()
        };
        self.uni_packet("group_anonymous_generate_nick.group", req.to_bytes())
    }
}
