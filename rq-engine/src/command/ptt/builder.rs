use bytes::Bytes;

use crate::command::common::PbToBytes;
use crate::pb;

impl super::super::super::Engine {
    pub fn build_group_try_up_ptt_req(
        &self,
        group_code: i64,
        file_md5: Vec<u8>,
        file_size: u64,
        codec: u32,
        voice_length: u32,
    ) -> Bytes {
        let req = pb::cmd0x388::D388ReqBody {
            net_type: Some(3),
            subcmd: Some(3),
            tryup_ptt_req: vec![pb::cmd0x388::TryUpPttReq {
                group_code: Some(group_code as u64),
                src_uin: Some(self.uin() as u64),
                file_md5: Some(file_md5.clone()),
                file_size: Some(file_size),
                file_name: Some(file_md5),
                src_term: Some(5),
                platform_type: Some(9),
                bu_type: Some(4),
                build_ver: Some("6.5.5.663".into()),
                inner_ip: Some(0),
                // TODO ?
                voice_length: Some(voice_length),
                new_up_chan: Some(true),
                codec: Some(codec),
                // 2021/1/26 因为 #577 修改为 resource.voiceCodec
                voice_type: Some(1),
                ..Default::default()
            }],
            ..Default::default()
        };
        req.to_bytes()
    }
}
