use crate::command::common::PbToBytes;
use crate::pb;
use crate::protocol::packet::*;

impl super::super::super::Engine {
    // group_member_card.get_group_member_card_info
    pub fn build_group_member_info_request_packet(&self, group_code: i64, uin: i64) -> Packet {
        let payload = pb::GroupMemberReqBody {
            group_code,
            uin,
            new_client: true,
            client_type: 1,
            rich_card_name_ver: 1,
        };
        self.uni_packet(
            "group_member_card.get_group_member_card_info",
            payload.to_bytes(),
        )
    }
}
