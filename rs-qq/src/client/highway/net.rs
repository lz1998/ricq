use std::net::SocketAddr;

use bytes::Bytes;
use futures::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio_util::codec::Framed;

use crate::client::highway::codec::HighwayCodec;
use crate::client::highway::HighwayFrame;
use crate::engine::command::common::PbToBytes;
use crate::engine::highway::BdhInput;
use crate::engine::{pb, RQError, RQResult};
use crate::Client;

impl Client {
    pub async fn highway_upload_bdh(&self, addr: SocketAddr, input: BdhInput) -> RQResult<Bytes> {
        let stream = TcpStream::connect(&addr).await.map_err(RQError::IO)?;
        let mut stream = Framed::new(stream, HighwayCodec);
        // send heartbeat
        let sum = md5::compute(&input.body).to_vec();
        let length = input.body.len();

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
        for (i, chunk) in input.body.chunks(input.chunk_size).enumerate() {
            let chunk = chunk.to_vec();
            let head = pb::ReqDataHighwayHead {
                msg_basehead: Some(self.highway_session.read().await.build_basehead(
                    "PicUp.DataUp".into(),
                    4096,
                    input.command_id,
                    2052,
                )),
                msg_seghead: Some(self.highway_session.read().await.build_seghead(
                    length as i64,
                    (i * input.chunk_size) as i64,
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
                    body: Bytes::from(chunk.clone()),
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
