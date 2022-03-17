use crate::command::common::PbToBytes;
use crate::pb;
use crate::protocol::packet::Packet;

impl super::super::super::Engine {
    pub fn build_multi_msg_apply_up_req(
        &self,
        msg_size: i64,
        msg_md5: Vec<u8>,
        bu_type: i32,
        group_uin: i64,
    ) -> Packet {
        let req = pb::multimsg::MultiReqBody {
            subcmd: 1,
            term_type: 5,
            platform_type: 9,
            net_type: 3,
            build_ver: self.transport.version.build_ver.into(),
            multimsg_applyup_req: vec![pb::multimsg::MultiMsgApplyUpReq {
                dst_uin: group_uin,
                msg_size,
                msg_md5,
                msg_type: 3, // group
                ..Default::default()
            }],
            bu_type,
            ..Default::default()
        };
        self.uni_packet("MultiMsg.ApplyUp", req.to_bytes())
    }
}
