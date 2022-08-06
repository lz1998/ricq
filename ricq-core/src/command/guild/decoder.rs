use bytes::Bytes;

use crate::{
    command::common::PbToBytes,
    pb::{
        self,
        guild::{FirstViewRsp, GuildUserProfile as GuildUserProf},
    },
    RQError, RQResult,
};

use super::{FirstViewResponse, GuildUserProfile};

impl super::super::super::Engine {
    pub fn decode_first_view_response(
        &self,
        payload: Bytes,
    ) -> RQResult<Option<FirstViewResponse>> {
        let rep = pb::guild::FirstViewRsp::from_bytes(&payload)
            .map_err(|e| RQError::Decode(format!("FirstViewRsp: {}", e)))?;

        match rep {
            FirstViewRsp {
                result: Some(r),
                err_msg: Some(err),
                ..
            } => Err(RQError::Decode(format!(
                "FirstViewRsp decode error: {}, {}",
                r,
                String::from_utf8_lossy(&err)
            ))),
            FirstViewRsp {
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

    pub fn decode_user_profile(&self, payload: Bytes) -> RQResult<Option<GuildUserProfile>> {
        let pkg = pb::oidb::OidbssoPkg::from_bytes(&payload)
            .map_err(|e| RQError::Decode(format!("OidbssoPkg: {}", e)))?;

        let oidb = pb::guild::ChannelOidb0xfc9Rsp::from_bytes(&pkg.bodybuffer)
            .map_err(|e| RQError::Decode(format!("ChannelOidb0xfc9Rsp: {}", e)))?;

        match oidb.profile {
            Some(GuildUserProf {
                tiny_id: Some(tiny_id),
                nickname: Some(nickname),
                avatar_url: Some(avatar_url),
                join_time,
            }) => Ok(Some(GuildUserProfile {
                tiny_id,
                nickname,
                avatar_url,
                join_time,
            })),

            _ => Ok(None),
        }
    }
}
