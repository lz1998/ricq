use bytes::Bytes;

use crate::{
    protocol::{packet::Packet, transport::Transport},
    Engine,
};

use super::{transport_traits::TransportTrait, PacketBuilder, BuilderArgs};

/// Engine 特征约束
/// 无内部方法，仅用于标记
pub trait EngineTrait: GetQQId + NextSeq + GetTransport + EnginePacket {}

/// Get Uin 获取当前在线的QQ号  
///
/// *`EngineTrait` 标记辅助trait*  
///
/// <del>如果我理解没错的话</del>
pub trait GetQQId {
    fn get_id(&self) -> i64;
}

/// 原来的接口没说，我也不知道干啥的（
pub trait NextSeq {
    fn next_seq(&self) -> u16;
    fn next_packet_seq(&self) -> i32;
    fn next_group_seq(&self) -> i32;
    fn next_friend_seq(&self) -> i32;
    fn next_group_data_trans_seq(&self) -> i32;
    fn next_highway_apply_seq(&self) -> i32;
}

/// 获取 Transport 用于辅助编码/解码数据
pub trait GetTransport {
    type Transport: TransportTrait;
    fn get_transport(&self) -> &Self::Transport;
}

/// 以uin为self.uin 方式打包
pub trait EnginePacket {
    fn oicq_request(&self, uin: i64, command_id: u16, body: &[u8]) -> Bytes;
    fn uin_packet(&self, command: &str, body: Bytes) -> Packet;
}

// impl for Inner Engine

impl GetQQId for Engine {
    fn get_id(&self) -> i64 {
        self.uin()
    }
}

impl NextSeq for Engine {
    fn next_seq(&self) -> u16 {
        self.next_seq()
    }

    fn next_packet_seq(&self) -> i32 {
        self.next_packet_seq()
    }

    fn next_group_seq(&self) -> i32 {
        self.next_group_seq()
    }

    fn next_friend_seq(&self) -> i32 {
        self.next_friend_seq()
    }

    fn next_group_data_trans_seq(&self) -> i32 {
        self.next_group_data_trans_seq()
    }

    fn next_highway_apply_seq(&self) -> i32 {
        self.next_highway_apply_seq()
    }
}

impl GetTransport for Engine {
    type Transport = Transport;
    fn get_transport(&self) -> &Self::Transport {
        &self.transport
    }
}

impl EnginePacket for Engine {
    fn uin_packet(&self, command: &str, body: Bytes) -> Packet {
        self.uni_packet(command.as_ref(), body)
    }

    fn oicq_request(&self, uin: i64, command_id: u16, body: &[u8]) -> Bytes {
        self.build_oicq_request_packet(uin, command_id, body)
    }
}

impl EngineTrait for Engine {}

/// packet 打包的engine端特征
///
/// *这个trait通常自动实现，基本不需要手动实现*
pub trait PacketBuildEngine: EngineTrait + Sized {
    /// 接受指定 Packet Builder 的 Args 并根据 Args 与 engine 自身的信息构造Packet
    fn build_packet<B, Arg>(&self, args: Arg) -> Packet
    where
        B: PacketBuilder<Args = Arg>,
        Arg: BuilderArgs<Builder = B>,
    {
        let body = B::build(self, args);
        B::packet(self, body)
    }
}

impl<E> PacketBuildEngine for E where E: EngineTrait + Sized {}