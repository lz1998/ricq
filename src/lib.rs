pub mod binary;
pub mod crypto;
pub mod hex;
pub mod client;
pub mod jce;

pub mod pb {
    include!(concat!(env!("OUT_DIR"), "/pb.rs"));

    pub mod structmsg { include!(concat!(env!("OUT_DIR"), "/structmsg.rs")); }

    pub mod msg { include!(concat!(env!("OUT_DIR"), "/msg.rs")); }

    pub mod cmd0x6ff { include!(concat!(env!("OUT_DIR"), "/cmd0x6ff.rs")); }
}

pub mod protocol {
    pub fn test() -> i32 {
        1
    }
}


#[cfg(test)]
mod tests {
    use std::net::TcpStream;
    use std::str::from_utf8;
    use std::time::Duration;

    use byteorder::{BigEndian, ReadBytesExt};


    #[test]
    fn test_connect() {
        // cli.device_info.guid = vec![0; 16];
        // cli.ecdh.initial_share_key = vec![10, 189, 97, 102, 56, 46, 27, 0, 80, 148, 249, 244, 112, 201, 120, 26];
        // cli.ecdh.public_key = vec![4, 199, 20, 9, 159, 245, 55, 112, 54, 175, 138, 225, 66, 235, 169, 126, 24, 21, 228, 12, 24, 114, 154, 176, 39, 105, 193, 149, 235, 115, 15, 196, 81, 154, 224, 93, 248, 143, 30, 132, 197, 5, 62, 198, 41, 215, 235, 20, 54, 97, 18, 13, 28, 252, 5, 30, 22, 180, 189, 139, 223, 151, 62, 29, 243];
        // cli.random_key = vec![0; 16];
        // cli.device_info.imei = "468356291846738".to_string();
        // let mut cli = Client::new(0,Password::String("".to_string()));
        // cli.device_info.gen_guid();
        // cli.device_info.gen_tgtgt_key();
        // TODO before connect
        // match TcpStream::connect("42.81.172.81:80") {
        //     Ok(mut stream) => {
        //         println!("Successfully connected to server");
        //
        //         let (seq, pkt) = cli.build_qrcode_fetch_request_packet(); // 获取登录二维码
        //         stream.write(&pkt).unwrap();
        //         println!("Sent fetch qrcode request");
        //         let l = stream.read_i32::<BigEndian>().unwrap();
        //         println!("Receive packet length: {}", l);
        //         let mut data = vec![0 as u8; l as usize - 4];
        //         stream.read(&mut data);
        //         println!("Receive packet data: {:?}", data);
        //         let pkt = cli.parse_incoming_packet(&mut data);
        //         println!("Incoming packet: {:?}", pkt);
        //         let resp = decode_trans_emp_response(&mut cli, &mut pkt.unwrap().payload).unwrap();
        //         println!("FetchQRCodeResp: {:?}", resp);
        //         std::fs::write("qrcode.png", resp.image_data);
        //         println!("qrcode.png is saved.");
        //
        //         loop {
        //             std::thread::sleep(Duration::from_secs(5));// 5秒查询一次登录二维码状态
        //             let (seq, pkt) = cli.build_qrcode_result_query_request_packet(&resp.sig);
        //             stream.write(&pkt).unwrap();
        //             let l = stream.read_i32::<BigEndian>().unwrap();
        //             println!("Receive packet length: {}", l);
        //             let mut data = vec![0 as u8; l as usize - 4];
        //             stream.read(&mut data);
        //             println!("Receive packet data: {:?}", data);
        //             let pkt = cli.parse_incoming_packet(&mut data);
        //             println!("Incoming packet: {:?}", pkt);
        //             let resp = decode_trans_emp_response(&mut cli, &mut pkt.unwrap().payload).unwrap();
        //             println!("QueryQRCodeResp: {:?}", resp);
        //             match resp.state {
        //                 LoginState::QRCodeImageFetch => {}
        //                 LoginState::QRCodeWaitingForScan => {
        //                 }
        //                 LoginState::QRCodeWaitingForConfirm => {}
        //                 LoginState::QRCodeTimeout => {}
        //                 LoginState::QRCodeConfirmed => {
        //                     println!("QRCode confirmed");
        //                     let (seq, pkt) = cli.build_qrcode_login_packet(&resp.login_info.tmp_pwd, &resp.login_info.tmp_no_pic_sig, &resp.login_info.tgt_qr);
        //                     stream.write(&pkt).unwrap();
        //                     let l = stream.read_i32::<BigEndian>().unwrap();
        //                     println!("QRCode confirmed - Receive packet length: {}", l);
        //                     let mut data = vec![0 as u8; l as usize - 4];
        //                     stream.read(&mut data);
        //                     println!("QRCode confirmed - Receive packet data: {:?}", data);
        //                     let pkt = cli.parse_incoming_packet(&mut data);
        //                     println!("QRCode confirmed - Incoming packet: {:?}", pkt);
        //                     let resp = decode_login_response(&mut cli, &mut pkt.unwrap().payload);
        //                     println!("QRCode confirmed - Decode Login Resp: {:?}", resp);
        //                     break
        //                 }
        //                 LoginState::QRCodeCanceled => {}
        //             }
        //         }
        //     }
        //     Err(e) => {
        //         println!("Failed to connect: {}", e);
        //     }
        // }
    }
}