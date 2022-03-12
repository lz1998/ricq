use bytes::{Buf, Bytes};

use crate::command::common::PbToBytes;
use crate::pb::msg::GetMessageResponse;
use crate::{jce, RQError, RQResult};

impl crate::Engine {
    // MessageSvc.PushNotify
    pub fn decode_svc_notify(&self, mut payload: Bytes) -> RQResult<jce::RequestPushNotify> {
        payload.advance(4);
        let mut req: jce::RequestPacket = jcers::from_buf(&mut payload)?;
        let mut data: jce::RequestDataVersion2 = jcers::from_buf(&mut req.s_buffer)?;
        let mut notify_data = data
            .map
            .remove("req_PushNotify")
            .ok_or_else(|| RQError::Decode("req_PushNotify".into()))?
            .remove("PushNotifyPack.RequestPushNotify")
            .ok_or_else(|| RQError::Decode("PushNotifyPack.RequestPushNotify".into()))?;
        notify_data.advance(1);
        let notify: jce::RequestPushNotify = jcers::from_buf(&mut notify_data)?;
        Ok(notify)
    }

    // MessageSvc.PushForceOffline
    pub fn decode_force_offline(
        &self,
        mut payload: Bytes,
    ) -> RQResult<jce::RequestPushForceOffline> {
        let mut req: jce::RequestPacket = jcers::from_buf(&mut payload)?;
        let mut data: jce::RequestDataVersion2 = jcers::from_buf(&mut req.s_buffer)?;
        let mut data = data
            .map
            .remove("req_PushForceOffline")
            .ok_or_else(|| RQError::Decode("req_PushForceOffline".into()))?
            .remove("PushNotifyPack.RequestPushForceOffline")
            .ok_or_else(|| RQError::Decode("PushNotifyPack.RequestPushForceOffline".into()))?;
        data.advance(1);
        let offline: jce::RequestPushForceOffline = jcers::from_buf(&mut data)?;
        Ok(offline)
    }

    // MessageSvc.PbGetMsg
    pub fn decode_message_svc_packet(
        &self,
        payload: Bytes,
    ) -> RQResult<super::MessageSyncResponse> {
        let resp = GetMessageResponse::from_bytes(&payload)
            .map_err(|_| RQError::Decode("GetMessageResponse".to_string()))?;
        Ok(super::MessageSyncResponse {
            msg_rsp_type: resp.msg_rsp_type.unwrap_or_default(),
            sync_flag: resp.sync_flag.unwrap(),
            sync_cookie: resp.sync_cookie,
            pub_account_cookie: resp.pub_account_cookie,
            msgs: resp
                .uin_pair_msgs
                .into_iter()
                .flat_map(|x| x.messages)
                .collect(),
        })
    }
}
