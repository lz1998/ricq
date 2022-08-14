use crate::protocol::protobuf;
use dynamic_protobuf::{dynamic_message, DynamicMessage};
use rand::Rng;
use ricq_core::command::common::PbToBytes;
use ricq_core::protocol::packet::Packet;
use std::sync::atomic::Ordering;

impl<'a> super::Engine<'a> {
    pub fn build_sync_channel_first_view_packet(&self) -> Packet {
        let req = protobuf::FirstViewReq {
            last_msg_time: Some(0),
            udc_flag: None,
            seq: Some(0),
            direct_message_flag: Some(1),
        };

        let b = req.to_bytes();
        self.uni_packet("trpc.group_pro.synclogic.SyncLogic.SyncFirstView", b)
    }

    pub fn build_get_user_profile_packet(&self, tiny_id: u64) -> Packet {
        let mut flags = DynamicMessage::new();

        for i in 3..=29 {
            flags.set(i, 1u32)
        }
        flags.set(99, 1u32);
        flags.set(100, 1u32);

        let payload = {
            let msg = dynamic_message! {
                1 => flags,
                3 => tiny_id,
                4 => 0u32,
            };

            self.transport.encode_oidb_packet(0xf88, 1, msg.encode())
        };

        self.uni_packet("OidbSvcTrpcTcp.0xfc9_1", payload)
    }

    pub fn build_send_channel_message_packet(
        &self,
        elems: Vec<ricq_core::pb::msg::Elem>,
        guild_id: u64,
        channel_id: u64,
    ) -> Packet {
        let routing = protobuf::ChannelRoutingHead {
            guild_id: Some(guild_id),
            channel_id: Some(channel_id),
            from_uin: Some(self.uin.load(Ordering::Relaxed) as _),
            from_tinyid: None,
            guild_code: None,
            from_appid: None,
            direct_message_flag: None,
        };

        let mut rng = rand::thread_rng();
        let random = rng.gen_range(0..i32::MAX);
        let content = protobuf::ChannelContentHead {
            r#type: Some(3840),
            sub_type: None,
            random: Some(random as _),
            seq: None,
            cnt_seq: None,
            time: None,
            meta: None,
        };

        let msg_head = protobuf::ChannelMsgHead {
            routing_head: Some(routing),
            content_head: Some(content),
        };

        let body = ricq_core::pb::msg::MessageBody {
            rich_text: Some(ricq_core::pb::msg::RichText {
                attr: None,
                elems,
                not_online_file: None,
                ptt: None,
            }),
            msg_content: None,
            msg_encrypt_content: None,
        };

        let content = protobuf::ChannelMsgContent {
            head: Some(msg_head),
            ctrl_head: None,
            body: Some(body),
            ext_info: None,
        };

        self.uni_packet(
            "MsgProxy.SendMsg",
            dynamic_message! {
                1 => content.to_bytes(),
            }
            .encode(),
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn build_guild_image_store_packet(
        &self,
        channel_id: u64,
        guild_code: u64,
        file_name: String,
        md5: Vec<u8>,
        size: u64,
        width: u32,
        height: u32,
        image_type: u32,
    ) -> Packet {
        let req = ricq_core::pb::cmd0x388::D388ReqBody {
            net_type: Some(3),
            subcmd: Some(1),
            // TODO 支持多张图片？
            tryup_img_req: vec![ricq_core::pb::cmd0x388::TryUpImgReq {
                group_code: Some(channel_id),
                src_uin: Some(self.uin() as u64),
                file_id: Some(0),
                file_md5: Some(md5),
                file_size: Some(size),
                file_name: Some(file_name.into_bytes()),
                src_term: Some(5),
                platform_type: Some(9),
                bu_type: Some(211),
                pic_type: Some(image_type),
                pic_width: Some(width),
                pic_height: Some(height),
                build_ver: Some(self.transport.version.build_ver.as_bytes().to_vec()),
                app_pic_type: Some(1050),
                qqmeet_guild_id: Some(guild_code),
                qqmeet_channel_id: Some(channel_id),
                ..Default::default()
            }],
            command_id: Some(83),
            ..Default::default()
        };
        self.uni_packet("ImgStore.QQMeetPicUp", req.to_bytes())
    }
}
