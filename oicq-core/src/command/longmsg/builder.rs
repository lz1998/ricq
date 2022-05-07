use crate::pb;
use prost::Message;

impl super::super::super::Engine {
    pub fn build_long_req(&self, dst_uin: i64, msg_content: Vec<u8>, msg_ukey: Vec<u8>) -> Vec<u8> {
        pb::longmsg::LongReqBody {
            subcmd: 1,
            term_type: 5,
            platform_type: 9,
            msg_up_req: vec![pb::longmsg::LongMsgUpReq {
                msg_type: 3, // group
                dst_uin,
                msg_id: 0,
                msg_content,
                store_type: 2,
                msg_ukey: msg_ukey,
                need_cache: 0,
            }],
            ..Default::default()
        }
        .encode_to_vec()
    }
}
