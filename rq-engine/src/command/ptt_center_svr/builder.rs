use crate::command::common::PbToBytes;
use crate::pb;
use crate::protocol::packet::Packet;

impl super::super::super::Engine {
    pub fn build_group_video_store_packet(
        &self,
        group_code: i64,
        file_name: String,
        file_md5: Vec<u8>,
        thumb_file_md5: Vec<u8>,
        file_size: i64,
        thumb_file_size: i64,
    ) -> Packet {
        let req = pb::short_video::ShortVideoReqBody {
            cmd: 300,
            ptt_short_video_upload_req: Some(pb::short_video::ShortVideoUploadReq {
                from_uin: self.uin(),
                to_uin: group_code,
                chat_type: 20,
                client_type: 1,
                info: Some(pb::short_video::ShortVideoFileInfo {
                    file_name,
                    file_md5,
                    thumb_file_md5,
                    file_size,
                    file_res_length: 1280,
                    file_res_width: 720,
                    file_format: 3,
                    file_time: 120,
                    thumb_file_size,
                }),
                group_code,
                support_large_size: 1,
                ..Default::default()
            }),
            extension_req: vec![pb::short_video::ShortVideoExtensionReq {
                sub_busi_type: 0,
                user_cnt: 1,
            }],
            ..Default::default()
        };
        self.uni_packet("PttCenterSvr.GroupShortVideoUpReq", req.to_bytes())
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
