use crate::command::common::PbToBytes;
use crate::command::guild::dynamic_msg::DynamicMessage;
use crate::pb;
use crate::protocol::packet::*;

impl super::super::super::Engine {
    pub fn build_sync_channel_first_view_packet(&self) -> Packet {
        let req = pb::guild::FirstViewReq {
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
            let mut msg = DynamicMessage::new();
            msg.set(1, flags);
            msg.set(3, tiny_id);
            msg.set(4, 0u32);

            self.transport.encode_oidb_packet(0xf88, 1, msg.encode())
        };

        self.uni_packet("OidbSvcTrpcTcp.0xfc9_1", payload)
    }
}
