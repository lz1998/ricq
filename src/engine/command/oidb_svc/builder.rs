use crate::engine::command::common::PbToBytes;
use crate::engine::protocol::packet::Packet;
use crate::pb;

impl super::super::super::Engine {
    // OidbSvc.0x88d_0
    pub fn build_group_info_request_packet(&self, group_code: i64) -> Packet {
        let transport = &self.transport;
        let body = pb::oidb::D88dReqBody {
            app_id: Some(transport.version.app_id),
            req_group_info: vec![pb::oidb::ReqGroupInfo {
                group_code: Some(group_code as u64),
                stgroupinfo: Some(pb::oidb::D88dGroupInfo {
                    group_owner: Some(0),
                    group_uin: Some(0),
                    group_create_time: Some(0),
                    group_flag: Some(0),
                    group_member_max_num: Some(0),
                    group_member_num: Some(0),
                    group_option: Some(0),
                    group_level: Some(0),
                    group_face: Some(0),
                    group_name: Some(vec![]),
                    group_memo: Some(vec![]),
                    group_finger_memo: Some(vec![]),
                    group_last_msg_time: Some(0),
                    group_cur_msg_seq: Some(0),
                    group_question: Some(vec![]),
                    group_answer: Some(vec![]),
                    group_grade: Some(0),
                    active_member_num: Some(0),
                    head_portrait_seq: Some(0),
                    msg_head_portrait: Some(pb::oidb::D88dGroupHeadPortrait::default()),
                    st_group_ex_info: Some(pb::oidb::D88dGroupExInfoOnly::default()),
                    group_sec_level: Some(0),
                    cmduin_privilege: Some(0),
                    no_finger_open_flag: Some(0),
                    no_code_finger_open_flag: Some(0),
                    ..Default::default()
                }),
                ..Default::default()
            }],
            pc_client_version: Some(0),
        };
        let payload = pb::oidb::OidbssoPkg {
            command: 2189,
            bodybuffer: body.to_bytes().to_vec(),
            ..Default::default()
        };
        self.uni_packet("OidbSvc.0x88d_0", payload.to_bytes())
    }
}
