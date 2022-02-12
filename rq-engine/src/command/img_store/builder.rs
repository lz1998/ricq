use crate::command::common::PbToBytes;
use crate::pb;
use crate::protocol::packet::Packet;

impl super::super::super::Engine {
    pub fn build_group_image_store_packet(
        &self,
        group_code: i64,
        file_name: String,
        md5: Vec<u8>,
        size: i32,
    ) -> Packet {
        let req = pb::cmd0x388::D388ReqBody {
            net_type: Some(3),
            subcmd: Some(1),
            // TODO 支持多张图片？
            tryup_img_req: vec![pb::cmd0x388::TryUpImgReq {
                group_code: Some(group_code as u64),
                src_uin: Some(self.uin() as u64),
                file_md5: Some(md5),
                file_size: Some(size as u64),
                file_name: Some(format!("{}.gif", file_name).as_bytes().to_vec()),
                src_term: Some(5),
                platform_type: Some(9),
                bu_type: Some(1),
                pic_type: Some(1000),
                build_ver: Some(self.transport.version.build_ver.as_bytes().to_vec()),
                app_pic_type: Some(1006),
                file_index: Some(vec![]),
                transfer_url: Some(vec![]),
                ..Default::default()
            }],
            extension: Some(vec![]),
            ..Default::default()
        };
        self.uni_packet("ImgStore.GroupPicUp", req.to_bytes())
    }
}
