use crate::command::common::PbToBytes;
use crate::protocol::packet::Packet;

impl crate::Engine {
    // LongConn.OffPicUp
    pub fn build_off_pic_up_packet(&self, target: i64, md5: Vec<u8>, size: i32) -> Packet {
        let req = crate::pb::cmd0x352::ReqBody {
            subcmd: Some(1),
            tryup_img_req: vec![crate::pb::cmd0x352::D352TryUpImgReq {
                src_uin: Some(self.uin() as u64),
                dst_uin: Some(target as u64),
                file_name: Some(
                    format!("{}.{}", crate::hex::encode_hex(&md5), ".jpg")
                        .as_bytes()
                        .to_vec(),
                ), //todo
                file_md5: Some(md5),
                file_size: Some(size as u64),
                src_term: Some(5),
                platform_type: Some(9),
                bu_type: Some(1),
                pic_original: Some(true),
                pic_type: Some(1000), //todo
                build_ver: Some(b"8.2.7.4410".to_vec()),
                file_index: Some(vec![]),
                srv_upload: Some(1),
                transfer_url: Some(vec![]),
                ..Default::default()
            }],
            ..Default::default()
        };
        self.uni_packet("LongConn.OffPicUp", req.to_bytes())
    }
}
