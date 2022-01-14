use bytes::{BufMut, Bytes, BytesMut};

pub fn build_code2d_request_packet(seq: u32, j: u64, cmd: u16, body: &[u8]) -> Bytes {
    let mut w = BytesMut::new();
    w.put_u8(2);
    w.put_u16((43 + body.len() + 1) as u16);
    w.put_u16(cmd);
    w.put_slice(&vec![0; 21]);
    w.put_u8(3);
    w.put_u16(0);
    w.put_u16(50);
    w.put_u32(seq);
    w.put_u64(j);
    w.put_slice(body);
    w.put_u8(3);
    w.into()
}
