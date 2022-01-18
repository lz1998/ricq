use bytes::{Buf, Bytes};

use crate::command::common::PbToBytes;
use crate::pb::msg::GetMessageResponse;
use crate::{jce, RQError, RQResult};

impl crate::Engine {
    // untested
    pub fn decode_svc_notify(&self, mut payload: Bytes) -> RQResult<Option<i32>> {
        payload.advance(4);
        let mut req: jce::RequestPacket = jcers::from_buf(&mut payload)?;
        let mut data: jce::RequestDataVersion2 = jcers::from_buf(&mut req.s_buffer)?;
        if data.map.is_empty() {
            return Ok(None);
        }
        let notify_data = data
            .map
            .get_mut("req_PushNotify")
            .unwrap()
            .get_mut("PushNotifyPack.RequestPushNotify")
            .unwrap();
        notify_data.advance(1);
        let notify: jce::RequestPushNotify = jcers::from_buf(notify_data)?;
        Ok(Some(notify.msg_type))
    }

    pub fn decode_message_svc_packet(
        &self,
        payload: Bytes,
    ) -> RQResult<super::MessageSyncResponse> {
        let resp = GetMessageResponse::from_bytes(&payload)
            .map_err(|_| RQError::Decode("GetMessageResponse".to_string()))?;
        Ok(super::MessageSyncResponse {
            sync_flag: resp.sync_flag.unwrap(),
            sync_cookie: Bytes::copy_from_slice(resp.sync_cookie()),
            pub_account_cookie: Bytes::copy_from_slice(resp.pub_account_cookie()),
            msgs: resp
                .uin_pair_msgs
                .into_iter()
                .flat_map(|x| x.messages)
                .collect(),
        })
    }
}
