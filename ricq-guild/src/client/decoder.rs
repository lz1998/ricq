use bytes::Bytes;
use ricq_core::{RQError, RQResult};

use crate::protocol::protobuf::FirstViewMsg;
use crate::protocol::{
    protobuf, protobuf::GuildUserProfile as GuildUserProf, FirstViewResponse, GuildImageStoreResp,
};
use crate::{
    ricq_core::command::common::PbToBytes,
    ricq_core::pb::{self},
};
use prost::Message;
use ricq_core::common::RQAddr;

pub struct Decoder;

impl Decoder {
    pub fn decode_guild_first_view_response(
        &self,
        payload: Bytes,
    ) -> RQResult<Option<FirstViewResponse>> {
        let rep = protobuf::FirstViewRsp::from_bytes(&payload)?;

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
        let msg = FirstViewMsg::from_bytes(&payload)?;

        Ok(msg)
    }

    pub fn decode_guild_user_profile(&self, payload: Bytes) -> RQResult<Option<GuildUserProf>> {
        let pkg = pb::oidb::OidbssoPkg::from_bytes(&payload)?;

        let oidb = protobuf::ChannelOidb0xfc9Rsp::from_bytes(&pkg.bodybuffer)?;

        Ok(oidb.profile)
    }

    pub fn decode_guild_image_store_response(
        &self,
        payload: Bytes,
    ) -> RQResult<GuildImageStoreResp> {
        let mut rsp = pb::cmd0x388::D388RspBody::decode(&*payload)?;
        let rsp = rsp
            .tryup_img_rsp
            .pop()
            .ok_or(RQError::EmptyField("tryup_img_rsp"))?;
        if rsp.result() != 0 {
            return Err(RQError::Other(
                String::from_utf8_lossy(&rsp.fail_msg.unwrap_or_default()).into_owned(),
            ));
        }

        let download_index = rsp
            .download_index
            .ok_or(RQError::EmptyField("download_index"))?;
        Ok(if rsp.file_exit.unwrap_or_default() {
            GuildImageStoreResp::Exist {
                file_id: rsp.fileid.unwrap_or_default(),
                addrs: rsp
                    .up_ip
                    .into_iter()
                    .zip(rsp.up_port)
                    .map(|(ip, port)| RQAddr(ip, port as u16))
                    .collect(),
                download_index,
            }
        } else {
            GuildImageStoreResp::NotExist {
                file_id: rsp.fileid.unwrap_or_default(),
                upload_key: rsp.up_ukey.unwrap_or_default(),
                upload_addrs: rsp
                    .up_ip
                    .into_iter()
                    .zip(rsp.up_port)
                    .map(|(ip, port)| RQAddr(ip, port as u16))
                    .collect(),
                download_index,
            }
        })
    }
}
