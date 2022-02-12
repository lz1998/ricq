use std::net::{Ipv4Addr, SocketAddr};

use bytes::Bytes;

use crate::command::common::PbToBytes;
use crate::command::img_store::GroupImageStoreResp;
use crate::common::RQIP;
use crate::{pb, RQError, RQResult};

impl super::super::super::Engine {
    pub fn decode_group_image_store_response(
        &self,
        payload: Bytes,
    ) -> RQResult<GroupImageStoreResp> {
        let mut rsp = pb::cmd0x388::D388RspBody::from_bytes(&payload)
            .map_err(|_| RQError::Decode("D388RspBody".into()))?;
        let rsp = rsp
            .tryup_img_rsp
            .pop()
            .ok_or_else(|| RQError::Other("tryup_img_rsp.len = 0".into()))?;
        if rsp.result() != 0 {
            return Err(RQError::Other(
                String::from_utf8_lossy(&rsp.fail_msg.unwrap_or_default()).to_string(),
            ));
        }
        Ok(if rsp.file_exit() {
            let img_info = rsp.img_info.unwrap_or_default();
            GroupImageStoreResp::Exist {
                file_id: rsp.file_id.unwrap_or_default(),
                height: img_info.file_height.unwrap_or_default(),
                width: img_info.file_width.unwrap_or_default(),
            }
        } else {
            GroupImageStoreResp::NotExist {
                file_id: rsp.fileid.unwrap_or_default(),
                upload_key: rsp.up_ukey.unwrap_or_default(),
                upload_addrs: rsp
                    .up_ip
                    .into_iter()
                    .zip(rsp.up_port)
                    .map(|(ip, port)| SocketAddr::new(Ipv4Addr::from(RQIP(ip)).into(), port as u16))
                    .collect(),
            }
        })
    }
}
