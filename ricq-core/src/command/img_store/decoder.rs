use bytes::Bytes;
use prost::Message;

use crate::command::img_store::GroupImageStoreResp;
use crate::common::RQAddr;
use crate::RQError::EmptyField;
use crate::{pb, RQError, RQResult};

impl super::super::super::Engine {
    pub fn decode_group_image_store_response(
        &self,
        payload: Bytes,
    ) -> RQResult<GroupImageStoreResp> {
        let mut rsp = pb::cmd0x388::D388RspBody::decode(&*payload)?;
        let rsp = rsp.tryup_img_rsp.pop().ok_or(EmptyField("tryup_img_rsp"))?;
        if rsp.result() != 0 {
            return Err(RQError::Other(
                String::from_utf8_lossy(&rsp.fail_msg.unwrap_or_default()).into_owned(),
            ));
        }
        Ok(if rsp.file_exit() {
            GroupImageStoreResp::Exist {
                file_id: rsp.fileid.unwrap_or_default(),
                addrs: rsp
                    .up_ip
                    .into_iter()
                    .zip(rsp.up_port)
                    .map(|(ip, port)| RQAddr(ip, port as u16))
                    .collect(),
            }
        } else {
            GroupImageStoreResp::NotExist {
                file_id: rsp.fileid.unwrap_or_default(),
                upload_key: rsp.up_ukey.unwrap_or_default(),
                upload_addrs: rsp
                    .up_ip
                    .into_iter()
                    .zip(rsp.up_port)
                    .map(|(ip, port)| RQAddr(ip, port as u16))
                    .collect(),
            }
        })
    }
}
