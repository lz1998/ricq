use std::collections::HashMap;
use std::io::Write;

use flate2::write::GzEncoder;
use flate2::Compression;

use crate::command::common::PbToBytes;
use crate::command::multi_msg::{ForwardMessage, PackedMessage};
use crate::msg::elem::RichMsg;
use crate::msg::MessageChain;
use crate::pb;
use crate::protocol::device::random_string;
use crate::protocol::packet::Packet;

impl super::super::super::Engine {
    pub fn build_multi_msg_apply_down_req(&self, res_id: String) -> Packet {
        let req = pb::multimsg::MultiReqBody {
            subcmd: 2,
            term_type: 5,
            platform_type: 9,
            net_type: 3,
            build_ver: self.transport.version.build_ver.into(),
            multimsg_applydown_req: vec![pb::multimsg::MultiMsgApplyDownReq {
                msg_resid: res_id.into_bytes(),
                msg_type: 3,
                ..Default::default()
            }],
            bu_type: 2,
            req_channel_type: 2,
            ..Default::default()
        };
        self.uni_packet("MultiMsg.ApplyDown", req.to_bytes())
    }

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
        messages: Vec<super::ForwardMessage>,
        group_code: i64,
    ) -> Vec<u8> {
        let PackedMessage {
            mut buffer,
            filename,
        } = self.pack_forward_msg(messages, group_code);
        let msgs = buffer.remove(&filename).expect("msgs not found");
        let mut pb_item_list = vec![pb::msg::PbMultiMsgItem {
            file_name: Some("MultiMsg".into()),
            buffer: Some(pb::msg::PbMultiMsgNew { msg: msgs.clone() }),
        }];
        pb_item_list.extend(
            buffer
                .into_iter()
                .map(|(filename, msg)| pb::msg::PbMultiMsgItem {
                    file_name: Some(filename),
                    buffer: Some(pb::msg::PbMultiMsgNew { msg }),
                })
                .collect::<Vec<pb::msg::PbMultiMsgItem>>(),
        );
        let trans = pb::msg::PbMultiMsgTransmit {
            msg: msgs.clone(),
            pb_item_list,
        };
        let mut encoder = GzEncoder::new(vec![], Compression::default());
        encoder.write_all(&trans.to_bytes()).ok();
        encoder.finish().unwrap_or_default()
    }

    fn pack_forward_msg(
        &self,
        messages: Vec<super::ForwardMessage>,
        group_code: i64,
    ) -> PackedMessage {
        let mut packed_buffers = HashMap::default();
        let msgs: Vec<pb::msg::Message> = messages
            .into_iter()
            .map(|m| match m {
                ForwardMessage::Message(message) => {
                    self.pack_msg(message, group_code)
                }
                ForwardMessage::Forward(forward) => {
                    let t_sum = forward.nodes.len();
                    let preview = super::gen_forward_preview(&forward.nodes);
                    let packed_message = self.pack_forward_msg(forward.nodes, group_code);
                    packed_buffers.extend(packed_message.buffer);
                    self.pack_msg(
                        super::MessageNode {
                            sender_id: forward.sender_id,
                            time: forward.time,
                            sender_name: forward.sender_name,
                            elements: MessageChain(
                                RichMsg {
                                    template1: format!(
                                        r##"<?xml version='1.0' encoding='UTF-8' standalone='yes' ?><msg serviceID="35" templateID="1" action="viewMultiMsg" brief="[聊天记录]" m_resid="" m_fileName="{}" tSum="{}" sourceMsgId="0" url="" flag="3" adverSign="0" multiMsgFlag="0"><item layout="1" advertiser_id="0" aid="0"><title size="34" maxLines="2" lineSpace="12">群聊的聊天记录</title>{}<hr hidden="false" style="0" /><summary size="26" color="#777777">查看{}条转发消息</summary></item><source name="聊天记录" icon="" action="" appid="-1" /></msg>"##,
                                        packed_message.filename, t_sum, preview, t_sum
                                    ),
                                    service_id: 35,
                                }.into()
                            ),
                        },
                        group_code,
                    )
                }
            })
            .collect();
        let filename = random_string(16);
        packed_buffers.insert(filename.clone(), msgs);
        PackedMessage {
            filename,
            buffer: packed_buffers,
        }
    }

    fn pack_msg(&self, node: super::MessageNode, group_code: i64) -> pb::msg::Message {
        pb::msg::Message {
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
        }
    }
}
