use bytes::Bytes;

use crate::command::common::PbToBytes;
use crate::common::RQAddr;
use crate::{pb, RQError, RQResult};

use super::OffPicUpResp;

impl crate::Engine {
    // LongConn.OffPicUp
    pub fn decode_off_pic_up_response(&self, payload: Bytes) -> RQResult<OffPicUpResp> {
        let mut resp = pb::cmd0x352::RspBody::from_bytes(&payload)
            .map_err(|_| RQError::Decode("RspBody".into()))?;
        if let Some(err) = resp.fail_msg {
            return Err(RQError::Other(String::from_utf8_lossy(&err).to_string()));
        }
        if resp.subcmd() != 1 {
            return Err(RQError::Other(format!(
                "subcmd is not 1: {}",
                resp.subcmd()
            )));
        }
        let img = resp
            .tryup_img_rsp
            .pop()
            .ok_or_else(|| RQError::Other("EmptyImgVec".into()))?;

        if img.result() != 0 {
            return Err(RQError::Other(
                String::from_utf8_lossy(&img.fail_msg.unwrap_or_default()).to_string(),
            ));
        }
        if img.file_exit() {
            Ok(OffPicUpResp::Exist {
                uuid: String::from_utf8_lossy(img.up_uuid()).to_string(),
                res_id: img.up_resid.unwrap_or_default(),
            })
        } else {
            Ok(OffPicUpResp::UploadRequired {
                uuid: String::from_utf8_lossy(img.up_uuid()).to_string(),
                res_id: img.up_resid.unwrap_or_default(),
                upload_key: img.up_ukey.unwrap_or_default(),
                upload_addrs: img
                    .up_ip
                    .into_iter()
                    .zip(img.up_port)
                    .map(|(ip, port)| RQAddr(ip, port as u16))
                    .collect(),
            })
        }
    }
}
