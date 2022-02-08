use bytes::{Buf, Bytes};

use crate::{
    protocol::{
        device::Device, oicq, packet::Packet, sig::Sig, transport::Transport, version::Version,
    },
    RQResult,
};

pub trait TransportEncoder {
    fn encoding_packet(&self, pkt: Packet) -> Bytes;
    fn encoding_oidb_packet(&self, cmd: i32, service_type: i32, body: Bytes) -> Bytes;
}

pub trait TransportDecoder {
    fn decode_packet<B: Buf>(&self, buff: B) -> RQResult<Packet>;
}

pub trait TransportInner {
    fn get_sig(&self) -> &Sig;
    fn get_device(&self) -> &Device;
    fn get_version(&self) -> &Version;
    fn get_oicq_codec(&self) -> &oicq::Codec;
}

pub trait TransportTrait: TransportDecoder + TransportEncoder + TransportInner {}

// impl for Inner Transport

impl TransportEncoder for Transport {
    fn encoding_packet(&self, pkt: Packet) -> Bytes {
        self.encode_packet(pkt)
    }

    fn encoding_oidb_packet(&self, cmd: i32, service_type: i32, body: Bytes) -> Bytes {
        self.encode_oidb_packet(cmd, service_type, body)
    }
}

impl TransportDecoder for Transport {
    fn decode_packet<B: Buf>(&self, buff: B) -> RQResult<Packet> {
        self.decode_packet(buff)
    }
}

impl TransportInner for Transport {
    fn get_sig(&self) -> &Sig {
        &self.sig
    }

    fn get_device(&self) -> &Device {
        &self.device
    }

    fn get_version(&self) -> &Version {
        &self.version
    }

    fn get_oicq_codec(&self) -> &oicq::Codec {
        &self.oicq_codec
    }
}

impl TransportTrait for Transport {}
