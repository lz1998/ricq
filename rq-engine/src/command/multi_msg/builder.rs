use std::io::Write;

use flate2::write::GzEncoder;
use flate2::Compression;

use crate::command::common::PbToBytes;
use crate::pb;
use crate::protocol::packet::Packet;

impl super::super::super::Engine {
    pub fn build_multi_msg_apply_up_req(
        &self,
        msg_size: i64,
        msg_md5: Vec<u8>,
        bu_type: i32,
        dst_uin: i64,
    ) -> Packet {
        let req = pb::multimsg::MultiReqBody {
            subcmd: 1,
            term_type: 5,
            platform_type: 9,
            net_type: 3,
            build_ver: self.transport.version.build_ver.into(),
            req_channel_type: 0,
            multimsg_applyup_req: vec![pb::multimsg::MultiMsgApplyUpReq {
                dst_uin,
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

    pub fn calculate_validation_data(
        &self,
        msgs: Vec<super::MessageNode>,
        group_code: i64,
    ) -> Vec<u8> {
        let msgs: Vec<pb::msg::Message> = msgs
            .into_iter()
            .map(|node| pb::msg::Message {
                head: Some(pb::msg::MessageHead {
                    from_uin: Some(node.sender_id),
                    msg_type: Some(82), // troop
                    msg_seq: Some(self.next_group_seq()),
                    msg_time: Some(node.time),
                    msg_uid: Some(0x01000000000000000 | rand::random::<u16>() as i64), // TODO ?
                    group_info: Some(pb::msg::GroupInfo {
                        group_code: Some(group_code),
                        group_card: Some(node.sender_name),
                        ..Default::default()
                    }),
                    mutiltrans_head: Some(pb::msg::MutilTransHead {
                        status: Some(0),
                        msg_id: Some(1),
                    }),
                    ..Default::default()
                }),
                body: Some(pb::msg::MessageBody {
                    rich_text: Some(pb::msg::RichText {
                        elems: node.elements.into(),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
                ..Default::default()
            })
            .collect();
        let trans = pb::msg::PbMultiMsgTransmit {
            msg: msgs.clone(),
            pb_item_list: vec![pb::msg::PbMultiMsgItem {
                file_name: Some("MultiMsg".into()),
                buffer: Some(pb::msg::PbMultiMsgNew { msg: msgs }),
            }],
        };
        let mut encoder = GzEncoder::new(vec![], Compression::default());
        encoder.write_all(&trans.to_bytes()).ok();
        encoder.finish().unwrap_or_default()
    }
}
