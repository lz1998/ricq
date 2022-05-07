use crate::command::common::PbToBytes;
use crate::protocol::packet::Packet;

impl crate::Engine {
    // LongConn.OffPicUp
    pub fn build_off_pic_up_packet(
        &self,
        target: i64,
        file_name: String,
        md5: Vec<u8>,
        size: u64,
        width: u32,
        height: u32,
        image_type: u32,
    ) -> Packet {
        let req = crate::pb::cmd0x352::ReqBody {
            subcmd: Some(1),
            tryup_img_req: vec![crate::pb::cmd0x352::D352TryUpImgReq {
                src_uin: Some(self.uin() as u64),
                dst_uin: Some(target as u64),
                file_name: Some(file_name.as_bytes().to_vec()), //todo
                file_md5: Some(md5),
                file_size: Some(size),
                pic_width: Some(width),
                pic_height: Some(height),
                pic_type: Some(image_type),
                pic_original: Some(true),
                build_ver: Some(self.transport.version.build_ver.as_bytes().to_vec()),
                bu_type: Some(1),
                src_term: Some(5),
                platform_type: Some(9),
                ..Default::default()
            }],
            ..Default::default()
        };
        self.uni_packet("LongConn.OffPicUp", req.to_bytes())
    }
}
