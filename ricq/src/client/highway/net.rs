use std::net::SocketAddr;
use std::time::Duration;

use bytes::Bytes;
use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio_util::codec::Framed;

use ricq_core::command::common::PbToBytes;
use ricq_core::crypto::qqtea_encrypt;
use ricq_core::highway::BdhInput;
use ricq_core::{pb, RQError, RQResult};

use crate::client::highway::codec::HighwayCodec;
use crate::client::highway::HighwayFrame;
use crate::client::tcp::tcp_connect_timeout;
use crate::Client;

impl Client {
    pub async fn highway_upload_bdh(
        &self,
        addr: SocketAddr,
        mut input: BdhInput,
        data: &[u8],
    ) -> RQResult<Bytes> {
        if input.encrypt {
            let session_key = self.highway_session.read().await.session_key.clone();
            input.ext = qqtea_encrypt(&input.ext, &session_key)
        }
        let stream = tcp_connect_timeout(addr, Duration::from_secs(5))
            .await
            .map_err(RQError::IO)?;
        let mut stream = Framed::new(stream, HighwayCodec);
        // send heartbeat
        let sum = md5::compute(data).to_vec();
        let length = data.len();

        if input.send_echo {
            stream
                .send(HighwayFrame {
                    head: self.highway_session.read().await.build_heartbreak(),
                    body: Bytes::new(),
                })
                .await?;
            let _ = read_response(&mut stream).await?;
        }
        let mut ticket = input.ticket;
        let mut rsp_ext = Bytes::new();
        let data = Bytes::copy_from_slice(data);
        let len = data.len();
        let chunk_size = input.chunk_size;

        for i in (0..len).step_by(chunk_size) {
            let min = std::cmp::min(i + chunk_size, len);
            let chunk = data.slice(i..min);
            let head = pb::ReqDataHighwayHead {
                msg_basehead: Some(self.highway_session.read().await.build_basehead(
                    "PicUp.DataUp".into(),
                    4096,
                    input.command_id,
                    2052,
                )),
                msg_seghead: Some(self.highway_session.read().await.build_seghead(
                    length as i64,
                    i as i64,
                    &chunk,
                    ticket.clone(),
                    sum.clone(),
                )),
                req_extendinfo: input.ext.clone(),
                ..Default::default()
            };
            stream
                .send(HighwayFrame {
                    head: head.to_bytes(),
                    body: chunk,
                })
                .await?;
            let resp = read_response(&mut stream).await?;
            let rsp_head = self
                .highway_session
                .read()
                .await
                .decode_rsp_head(resp.head)?;
            if rsp_head.error_code != 0 {
                return Err(RQError::Other(format!(
                    "error_code = {}",
                    rsp_head.error_code
                )));
            }
            if !rsp_head.rsp_extendinfo.is_empty() {
                rsp_ext = Bytes::from(rsp_head.rsp_extendinfo)
            }
            if let Some(h) = rsp_head.msg_seghead {
                if !h.serviceticket.is_empty() {
                    ticket = h.serviceticket
                }
            }
        }

        Ok(rsp_ext)
    }
}

async fn read_response(stream: &mut Framed<TcpStream, HighwayCodec>) -> RQResult<HighwayFrame> {
    loop {
        if let Some(resp) = stream.next().await {
            return resp;
        }
    }
}
