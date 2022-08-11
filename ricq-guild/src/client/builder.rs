use dynamic_protobuf::{dynamic_message, DynamicMessage};
use ricq_core::command::common::PbToBytes;
use ricq_core::protocol::packet::Packet;
use crate::protocol::protobuf;

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
}
