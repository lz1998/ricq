use bytes::Bytes;
use prost::Message;

use crate::msg::elem::Anonymous;
use crate::{pb, RQError, RQResult};

impl super::super::super::Engine {
    // group_member_card.get_group_member_card_info
    pub fn decode_get_anony_info_response(&self, payload: Bytes) -> RQResult<Option<Anonymous>> {
        let resp = pb::cmd0x3bb::AnonyMsg::decode(&*payload)?;
        let rsp = resp.anony_rsp.ok_or(RQError::EmptyField("anony_rsp"))?;
        let enable_anony = rsp
            .anony_status
            .map(|s| s.forbid_talking.unwrap_or(1) == 0)
            .unwrap_or_default();
        if !enable_anony {
            return Ok(None);
        }
        Ok(Some(Anonymous {
            anon_id: vec![],
            nick: String::from_utf8_lossy(&rsp.anony_name.unwrap_or_default()).into_owned(),
            portrait_index: rsp.portrait_index.unwrap_or_default() as i32,
            bubble_index: rsp.bubble_index.unwrap_or_default() as i32,
            expire_time: rsp.expired_time.unwrap_or_default() as i32,
            color: rsp.color.unwrap_or_default(),
        }))
    }
}
