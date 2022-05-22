use crate::command::common::PbToBytes;
use crate::hex::encode_hex;
use crate::pb;
use crate::protocol::packet::Packet;

impl super::super::super::Engine {
    pub fn build_group_video_store_packet(
        &self,
        short_video_up_req: pb::short_video::ShortVideoUploadReq,
    ) -> Packet {
        let seq = self.next_seq();
        let req = pb::short_video::ShortVideoReqBody {
            seq: seq as i32,
            cmd: 300,
            ptt_short_video_upload_req: Some(short_video_up_req),
            extension_req: vec![pb::short_video::ShortVideoExtensionReq {
                sub_busi_type: 0,
                user_cnt: 1,
            }],
            ..Default::default()
        };
        self.uni_packet_with_seq(
            seq as i32,
            "PttCenterSvr.GroupShortVideoUpReq",
            req.to_bytes(),
        )
    }

    pub fn build_short_video_up_req(
        &self,
        to_uin: i64,
        file_md5: Vec<u8>,
        thumb_file_md5: Vec<u8>,
        file_size: i64,
        thumb_file_size: i64,
    ) -> pb::short_video::ShortVideoUploadReq {
        pb::short_video::ShortVideoUploadReq {
            from_uin: self.uin(),
            to_uin,
            chat_type: 1,
            client_type: 2,
            info: Some(pb::short_video::ShortVideoFileInfo {
                file_name: format!("{}.mp4", encode_hex(&file_md5)),
                file_md5,
                thumb_file_md5,
                file_size,
                file_res_length: 1280,
                file_res_width: 720,
                file_format: 3,
                file_time: 120,
                thumb_file_size,
            }),
            group_code: to_uin,
            support_large_size: 1,
            ..Default::default()
        }
    }

    // PttCenterSvr.pb_pttCenter_CMD_REQ_APPLY_DOWNLOAD-1200
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
