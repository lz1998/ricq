use crate::engine::command::common::PbToBytes;
use crate::engine::protocol::packet::Packet;
use crate::pb;

impl super::super::super::Engine {
    // ProfileService.Pb.ReqSystemMsgNew.Group
    pub fn build_system_msg_new_group_packet(&self, suspicious: bool) -> Packet {
        let req = pb::structmsg::ReqSystemMsgNew {
            msg_num: 100,
            version: 1000,
            checktype: 3,
            flag: Some(pb::structmsg::FlagInfo {
                grp_msg_kick_admin: 1,
                grp_msg_hidden_grp: 1,
                grp_msg_wording_down: 1,
                grp_msg_get_official_account: 1,
                grp_msg_get_pay_in_group: 1,
                frd_msg_discuss2_many_chat: 1,
                grp_msg_not_allow_join_grp_invite_not_frd: 1,
                frd_msg_need_waiting_msg: 1,
                frd_msg_uint32_need_all_unread_msg: 1,
                grp_msg_need_auto_admin_wording: 1,
                grp_msg_get_transfer_group_msg_flag: 1,
                grp_msg_get_quit_pay_group_msg_flag: 1,
                grp_msg_support_invite_auto_join: 1,
                grp_msg_mask_invite_auto_join: 1,
                grp_msg_get_disbanded_by_admin: 1,
                grp_msg_get_c2c_invite_join_group: 1,
                ..Default::default()
            }),
            friend_msg_type_flag: 1,
            req_msg_type: if suspicious { 2 } else { 1 },
            ..Default::default()
        };
        let payload = req.to_bytes();
        self.uni_packet("ProfileService.Pb.ReqSystemMsgNew.Group", payload)
    }
}
