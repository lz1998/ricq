use bytes::Bytes;

use crate::{RQError, RQResult};

#[derive(PartialEq, derivative::Derivative, Eq)]
#[derivative(Default, Debug)]
pub enum PacketType {
    #[derivative(Default)]
    Simple,
    Login,
}

impl PacketType {
    pub fn value(&self) -> u32 {
        match self {
            PacketType::Login => 0x0A,
            PacketType::Simple => 0x0B,
        }
    }

    pub fn from_i32(v: i32) -> RQResult<Self> {
        match v {
            0x0A => Ok(Self::Login),
            0x0B => Ok(Self::Simple),
            _ => Err(RQError::InvalidPacketType),
        }
    }
}

#[derive(PartialEq, derivative::Derivative, Eq)]
#[derivative(Default, Debug)]
pub enum EncryptType {
    #[derivative(Default)]
    NoEncrypt,
    D2Key,
    EmptyKey,
}

impl EncryptType {
    pub fn value(&self) -> u32 {
        match self {
            EncryptType::NoEncrypt => 0x00,
            EncryptType::D2Key => 0x01,
            EncryptType::EmptyKey => 0x02,
        }
    }
    pub fn from_u8(v: u8) -> RQResult<Self> {
        match v {
            0x00 => Ok(Self::NoEncrypt),
            0x01 => Ok(Self::D2Key),
            0x02 => Ok(Self::EmptyKey),
            _ => Err(RQError::InvalidEncryptType),
        }
    }
}

#[derive(Default, Debug)]
pub struct Packet {
    pub packet_type: PacketType,
    pub encrypt_type: EncryptType,
    pub seq_id: i32,
    pub body: Bytes,
    pub command_name: String,
    pub uin: i64,
    pub message: String,
}

impl Packet {
    pub fn check_command_name(self, command_name: &str) -> RQResult<Self> {
        if self.command_name != command_name {
            Err(RQError::CommandNameMismatch(
                command_name.to_owned(),
                self.command_name,
            ))
        } else {
            Ok(self)
        }
    }
}
