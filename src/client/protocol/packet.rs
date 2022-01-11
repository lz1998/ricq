use bytes::Bytes;

#[derive(derivative::Derivative)]
#[derivative(Default)]
pub enum PacketType {
    #[derivative(Default)]
    RequestTypeSimple,
    RequestTypeLogin,
}

impl PacketType {
    pub fn value(&self) -> u32 {
        match self {
            PacketType::RequestTypeLogin => 0x0A,
            PacketType::RequestTypeSimple => 0x0B,
        }
    }
}

#[derive(derivative::Derivative)]
#[derivative(Default)]
pub enum EncryptType {
    #[derivative(Default)]
    EncryptTypeNoEncrypt,
    EncryptTypeD2Key,
    EncryptTypeEmptyKey,
}

impl EncryptType {
    pub fn value(&self) -> u32 {
        match self {
            EncryptType::EncryptTypeNoEncrypt => 0x00,
            EncryptType::EncryptTypeD2Key => 0x01,
            EncryptType::EncryptTypeEmptyKey => 0x02,
        }
    }
}

pub struct Packet {
    pub packet_type: PacketType,
    pub encrypt_type: EncryptType,
    pub seq_id: i32,
    pub body: Bytes,
    pub command_name: String,
    pub uin: i64,
    pub message: String,
}
