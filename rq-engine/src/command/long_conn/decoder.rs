use super::OffPicUpResp;
use crate::command::common::PbToBytes;
use crate::common::RQIP;
use crate::{pb, RQError, RQResult};
use bytes::Bytes;

impl crate::Engine {
    // LongConn.OffPicUp
    pub fn decode_off_pic_up_response(&self, payload: Bytes) -> RQResult<OffPicUpResp> {
        let mut resp = pb::cmd0x352::RspBody::from_bytes(&payload)
            .map_err(|_| RQError::Decode("RspBody".into()))?;
        if let Some(err) = resp.fail_msg {
            Ok(OffPicUpResp::Fail(
                String::from_utf8(err).map_err(RQError::from)?,
            ))
        } else if resp.subcmd() != 1 {
            Ok(OffPicUpResp::SubComErr(resp.subcmd()))
        } else if resp.tryup_img_rsp.is_empty() {
            Ok(OffPicUpResp::EmptyImgVec)
        } else {
            let img = resp.tryup_img_rsp.swap_remove(0);
            if img.result() != 0 {
                Ok(OffPicUpResp::Fail(String::from_utf8(
                    img.fail_msg().to_vec(),
                )?))
            } else if img.file_exit() {
                Ok(OffPicUpResp::Exit(String::from_utf8(
                    img.up_resid().to_vec(),
                )?))
            } else {
                Ok(OffPicUpResp::UploadRequired {
                    res_id: String::from_utf8(img.up_resid().to_vec())?,
                    upload_key: img.up_ukey().to_vec(),
                    upload_addrs: img
                        .up_ip
                        .into_iter()
                        .zip(img.up_port)
                        .map(|(ip, port)| {
                            std::net::SocketAddr::new(
                                std::net::Ipv4Addr::from(RQIP(ip)).into(),
                                port as u16,
                            )
                        })
                        .collect(),
                })
            }
        }
    }
}
