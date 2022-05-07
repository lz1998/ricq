use crate::protocol::packet::*;

impl super::super::super::Engine {
    // Heartbeat.Alive
    pub fn build_heartbeat_packet(&self) -> Packet {
        let seq = self.next_seq();
        Packet {
            packet_type: PacketType::Login,
            encrypt_type: EncryptType::NoEncrypt,
            seq_id: seq as i32,
            command_name: "Heartbeat.Alive".into(),
            uin: self.uin(),
            ..Default::default()
        }
    }
}
