use bytes::Bytes;
use ricq_core::{RQError, RQResult};

use crate::{
    ricq_core::command::common::PbToBytes,
    ricq_core::pb::{self},
};

use crate::protocol::protobuf::FirstViewMsg;
use crate::protocol::{protobuf, protobuf::GuildUserProfile as GuildUserProf, FirstViewResponse};

pub struct Decoder;

impl Decoder {
    pub fn decode_guild_first_view_response(
        &self,
        payload: Bytes,
    ) -> RQResult<Option<FirstViewResponse>> {
        let rep = protobuf::FirstViewRsp::from_bytes(&payload)
            .map_err(|e| RQError::Decode(format!("FirstViewRsp: {}", e)))?;

        match rep {
            protobuf::FirstViewRsp {
                result: Some(r),
                err_msg: Some(err),
                ..
            } => Err(RQError::Decode(format!(
                "FirstViewRsp decode error: {}, {}",
                r,
                String::from_utf8_lossy(&err)
            ))),
            protobuf::FirstViewRsp {
                guild_count: Some(guild_count),
                self_tinyid: Some(self_tinyid),
                direct_message_switch: Some(direct_message_switch),
                direct_message_guild_count: Some(direct_message_guild_count),
                ..
            } => Ok(Some(FirstViewResponse {
                guild_count,
                self_tinyid,
                direct_message_switch,
                direct_message_guild_count,
            })),
            _ => Ok(None),
        }
    }

    pub fn decode_first_view_msg(&self, payload: Bytes) -> RQResult<FirstViewMsg> {
        let msg = FirstViewMsg::from_bytes(&payload)
            .map_err(|e| RQError::Decode(format!("FirstViewMsg: {}", e)))?;

        Ok(msg)
    }

    pub fn decode_guild_user_profile(&self, payload: Bytes) -> RQResult<Option<GuildUserProf>> {
        let pkg = pb::oidb::OidbssoPkg::from_bytes(&payload)
            .map_err(|e| RQError::Decode(format!("OidbssoPkg: {}", e)))?;

        let oidb = protobuf::ChannelOidb0xfc9Rsp::from_bytes(&pkg.bodybuffer)
            .map_err(|e| RQError::Decode(format!("ChannelOidb0xfc9Rsp: {}", e)))?;

        Ok(oidb.profile)
    }
}
