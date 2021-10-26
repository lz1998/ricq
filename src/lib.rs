pub mod packet;
pub mod tlv;
pub mod binary_writer;
pub mod version;
pub mod encrypt;
pub mod hex;
pub mod tea;
pub mod client;
pub mod device;
pub mod client_packet;

pub mod protocol {
    pub fn test() -> i32 {
        1
    }
}


#[cfg(test)]
mod tests {
    use std::io::{Read, Write};
    use std::net::TcpStream;
    use std::str::from_utf8;
    use crate::client::Client;
    use crate::client_packet::ClientPacket;

    #[test]
    fn test_connect() {
        let mut cli = Client::new();
        cli.device_info.guid = vec![0; 16];
        cli.ecdh.initial_share_key = vec![10, 189, 97, 102, 56, 46, 27, 0, 80, 148, 249, 244, 112, 201, 120, 26];
        cli.ecdh.public_key = vec![4, 199, 20, 9, 159, 245, 55, 112, 54, 175, 138, 225, 66, 235, 169, 126, 24, 21, 228, 12, 24, 114, 154, 176, 39, 105, 193, 149, 235, 115, 15, 196, 81, 154, 224, 93, 248, 143, 30, 132, 197, 5, 62, 198, 41, 215, 235, 20, 54, 97, 18, 13, 28, 252, 5, 30, 22, 180, 189, 139, 223, 151, 62, 29, 243];
        cli.random_key = vec![0; 16];
        cli.device_info.imei = "468356291846738".to_string();
        cli.ksid = format!("|{}|A8.2.7.27f6ea96", cli.device_info.imei).into_bytes(); // TODO before connect
        let (seq, pkt) = cli.build_qrcode_fetch_request_packet();
        println!("{:?}", pkt);
        match TcpStream::connect("42.81.172.81:80") {
            Ok(mut stream) => {
                println!("Successfully connected to server in port 3333");

                let msg = b"Hello!";

                stream.write(&pkt).unwrap();
                println!("Sent Hello, awaiting reply...");

                let mut data = [0 as u8; 10240]; // using 6 byte buffer
                stream.read(&mut data);
                println!("resp: {:?}", data);
                // match stream.read_exact(&mut data) {
                //     Ok(_) => {
                //         if &data == msg {
                //             println!("Reply is ok!");
                //         } else {
                //             let text = from_utf8(&data).unwrap();
                //             println!("Unexpected reply: {}", text);
                //         }
                //     },
                //     Err(e) => {
                //         println!("Failed to receive data: {}", e);
                //     }
                // }
            }
            Err(e) => {
                println!("Failed to connect: {}", e);
            }
        }
    }
}