use bytes::Bytes;

use crate::command::common::PbToBytes;
use crate::hex::encode_hex;
use crate::pb;
use crate::protocol::packet::Packet;

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
                build_ver: Some(self.transport.version.build_ver.into()),
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

    pub fn build_friend_try_up_ptt_req(
        &self,
        target: i64,
        file_md5: Vec<u8>,
        file_size: i64,
        voice_length: i32,
    ) -> Bytes {
        let req = pb::cmd0x346::C346ReqBody {
            cmd: 500,
            seq: self.next_seq() as i32,
            business_id: 17,
            client_type: 104,
            apply_upload_req: Some(pb::cmd0x346::ApplyUploadReq {
                sender_uin: self.uin(),
                recver_uin: target,
                file_type: 2,
                file_size,
                file_name: encode_hex(&file_md5),
                bytes_10m_md5: file_md5,
                ..Default::default()
            }),
            extension_req: Some(pb::cmd0x346::ExtensionReq {
                id: 3,
                ptt_format: 1,
                net_type: 3,
                voice_type: 2,
                ptt_time: voice_length,
                ..Default::default()
            }),
            ..Default::default()
        };
        req.to_bytes()
    }

    pub fn build_group_ptt_down_req(&self, group_code: i64, file_md5: Vec<u8>) -> Packet {
        let req = pb::cmd0x388::D388ReqBody {
            net_type: Some(3),
            subcmd: Some(4),
            getptt_url_req: vec![pb::cmd0x388::GetPttUrlReq {
                group_code: Some(group_code as u64),
                dst_uin: Some(self.uin() as u64),
                fileid: None,
                file_md5: Some(file_md5),
                req_term: Some(5),
                req_platform_type: Some(9),
                inner_ip: Some(0),
                bu_type: Some(4), // 3?
                build_ver: Some(self.transport.version.build_ver.into()),
                codec: Some(0),
                // 11=file_key, 14=2, 15=1 ?
                ..Default::default()
            }],
            ..Default::default()
        };
        self.uni_packet("PttStore.GroupPttDown", req.to_bytes())
    }

    pub fn build_c2c_ptt_down_req(&self, sender_uin: i64, file_uuid: Vec<u8>) -> Packet {
        let req = pb::cmd0x346::C346ReqBody {
            client_type: 104,
            cmd: 1200,
            business_id: 17, // 3?
            apply_download_req: Some(pb::cmd0x346::ApplyDownloadReq {
                uin: sender_uin,
                uuid: file_uuid,
                need_https_url: 1,
                ..Default::default()
            }),
            ..Default::default()
        };
        self.uni_packet(
            "PttCenterSvr.pb_pttCenter_CMD_REQ_APPLY_DOWNLOAD-1200",
            req.to_bytes(),
        )
    }
}
