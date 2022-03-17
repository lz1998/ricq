use crate::command::common::PbToBytes;
use crate::pb;
use crate::protocol::packet::Packet;

impl super::super::super::Engine {
    // PbMessageSvc.PbMsgReadedReport
    pub fn build_group_msg_readed_packet(&self, group_code: i64, msg_seq: i32) -> Packet {
        let req = pb::msg::PbMsgReadedReportReq {
            grp_read_report: vec![pb::msg::PbGroupReadedReportReq {
                group_code: Some(group_code as u64),
                last_read_seq: Some(msg_seq as u64),
            }],
            ..Default::default()
        };
        self.uni_packet("PbMessageSvc.PbMsgReadedReport", req.to_bytes())
    }

    // PbMessageSvc.PbMsgReadedReport
    pub fn build_friend_msg_readed_packet(&self, uin: i64, time: i64) -> Packet {
        let transport = &self.transport;
        let req = pb::msg::PbMsgReadedReportReq {
            c2_c_read_report: Some(pb::msg::PbC2cReadedReportReq {
                pair_info: vec![pb::msg::UinPairReadInfo {
                    peer_uin: Some(uin as u64),
                    last_read_time: Some(time as u32),
                    ..Default::default()
                }],
                sync_cookie: Some(transport.sig.sync_cookie.to_vec()),
            }),
            ..Default::default()
        };
        self.uni_packet("PbMessageSvc.PbMsgReadedReport", req.to_bytes())
    }
}
