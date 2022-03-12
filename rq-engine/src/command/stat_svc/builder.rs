use std::collections::HashMap;

use bytes::{BufMut, Bytes, BytesMut};
use jcers::JcePut;

use crate::command::common::pack_uni_request_data;
use crate::command::common::PbToBytes;
use crate::jce;
use crate::protocol::packet::*;
use crate::structs::CustomOnlineStatus;

impl super::super::super::Engine {
    // StatSvc.SetStatusFromClient
    pub fn build_set_online_status_packet(
        &self,
        online_status: i32,
        ext_online_status: i64,
        custom_status: Option<CustomOnlineStatus>,
    ) -> Packet {
        let transport = &self.transport;
        let svc = jce::SvcReqRegister {
            uin: self.uin(),
            bid: 1 | 2 | 4,
            conn_type: 0,
            status: online_status,
            kick_pc: 0,
            kick_weak: 0,
            ios_version: transport.device.version.sdk as i64,
            net_type: 1, // 0-移动网络 1-wifi
            reg_type: 0,
            guid: transport.sig.guid.to_owned(),
            is_set_status: 1,
            locale_id: 2052,
            dev_name: transport.device.model.to_owned(),
            dev_type: transport.device.model.to_owned(),
            os_ver: transport.device.version.release.to_owned(),
            open_push: 1,
            large_seq: 1551,
            vendor_name: transport.device.vendor_name.to_owned(),
            vendor_os_name: transport.device.vendor_os_name.to_owned(),
            ext_online_status,
            timestamp: chrono::Utc::now().timestamp(),
            custom_status: custom_status
                .map(|custom_status| {
                    crate::pb::online_status::CustomStatus {
                        face_index: Some(custom_status.face_index),
                        wording: Some(custom_status.wording),
                        face_type: Some(1),
                    }
                    .to_bytes()
                })
                .unwrap_or_default(),
            ..Default::default()
        };
        let pkt = self.svc_req_register_pkt(svc);
        self.uni_packet("StatSvc.SetStatusFromClient", pkt.freeze())
    }

    // StatSvc.register
    pub fn build_client_register_packet(&self) -> Packet {
        let seq = self.next_seq();
        let transport = &self.transport;

        let svc = jce::SvcReqRegister {
            uin: self.uin(),
            bid: 1 | 2 | 4,
            conn_type: 0,
            status: 11,
            kick_pc: 0,
            kick_weak: 0,
            ios_version: transport.device.version.sdk as i64,
            net_type: 1, // 0-移动网络 1-wifi
            reg_type: 0,
            guid: transport.sig.guid.to_owned(),
            is_set_status: 0,
            locale_id: 2052,
            dev_name: transport.device.model.to_owned(),
            dev_type: transport.device.model.to_owned(),
            os_ver: transport.device.version.release.to_owned(),
            open_push: 1,
            large_seq: 1551,
            old_sso_ip: 0,
            new_sso_ip: 31806887127679168,
            channel_no: "".to_string(),
            cpid: 0,
            vendor_name: transport.device.vendor_name.to_owned(),
            vendor_os_name: transport.device.vendor_os_name.to_owned(),
            b769: Bytes::from_static(&[
                0x0A, 0x04, 0x08, 0x2E, 0x10, 0x00, 0x0A, 0x05, 0x08, 0x9B, 0x02, 0x10, 0x00,
            ]),
            set_mute: 0,
            ..Default::default()
        };
        let pkt = self.svc_req_register_pkt(svc);
        Packet {
            packet_type: PacketType::Login,
            encrypt_type: EncryptType::D2Key,
            seq_id: seq as i32,
            body: pkt.freeze(),
            command_name: "StatSvc.register".into(),
            uin: self.uin(),
            ..Default::default()
        }
    }

    fn svc_req_register_pkt(&self, svc: jce::SvcReqRegister) -> jce::RequestPacket {
        let mut b = BytesMut::new();
        b.put_slice(&[0x0A]);
        b.put_slice(&svc.freeze());
        b.put_slice(&[0x0B]);
        let buf = jce::RequestDataVersion3 {
            map: HashMap::from([("SvcReqRegister".to_string(), b.into())]),
        };
        jce::RequestPacket {
            i_version: 3,
            s_servant_name: "PushService".to_string(),
            s_func_name: "SvcReqRegister".to_string(),
            s_buffer: buf.freeze(),
            context: Default::default(),
            status: Default::default(),
            ..Default::default()
        }
    }

    // StatSvc.GetDevLoginInfo
    pub fn build_device_list_request_packet(&self) -> Packet {
        let transport = &self.transport;
        let req = jce::SvcReqGetDevLoginInfo {
            guid: transport.sig.guid.to_owned(),
            login_type: 1,
            app_name: "com.tencent.mobileqq".into(),
            require_max: 20,
            get_dev_list_type: 20,
            ..Default::default()
        };
        let buf = jce::RequestDataVersion3 {
            map: HashMap::from([(
                "SvcReqGetDevLoginInfo".to_string(),
                pack_uni_request_data(&req.freeze()),
            )]),
        };
        let pkt = jce::RequestPacket {
            i_version: 3,
            s_servant_name: "StatSvc".to_string(),
            s_func_name: "SvcReqGetDevLoginInfo".to_string(),
            s_buffer: buf.freeze(),
            ..Default::default()
        };
        self.uni_packet("StatSvc.GetDevLoginInfo", pkt.freeze())
    }

    // StatSvc.RspMSFForceOffline
    pub fn build_msf_force_offline_rsp(&self, uin: i64, seq_no: i64) -> Packet {
        let rsp = jce::RspMSFForceOffline {
            uin,
            seq_no,
            const_zero: 0,
        };
        let buf = jce::RequestDataVersion3 {
            map: HashMap::from([(
                "RspMSFForceOffline".to_string(),
                pack_uni_request_data(&rsp.freeze()),
            )]),
        };
        let pkt = jce::RequestPacket {
            i_version: 3,
            s_servant_name: "StatSvc".to_string(),
            s_func_name: "RspMSFForceOffline".to_string(),
            s_buffer: buf.freeze(),
            ..Default::default()
        };
        self.uni_packet("StatSvc.RspMSFForceOffline", pkt.freeze())
    }
}
