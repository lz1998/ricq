use crate::client::income::decoder::config_push_svc::decode_push_req_packet;
use crate::client::income::decoder::online_push::decode_group_message_packet;
use crate::client::income::IncomePacket;

pub mod online_push;
pub mod config_push_svc;

impl super::Client{
    pub async fn process_income_packet(&self, pkt: IncomePacket) {
        // response
        if let Some(sender) = self.packet_promises.write().await.remove(&pkt.seq_id) {
            sender.send(pkt); // response
            return;
        }

        match pkt.command_name.as_ref() {
            "OnlinePush.PbPushGroupMsg" => {
                let p = decode_group_message_packet(&pkt.payload).unwrap();
                self.process_group_message_part(p).await;
            }
            "ConfigPushSvc.PushReq"=>{
                let req=decode_push_req_packet(&pkt.payload).unwrap();
                self.process_config_push_req(req).await;
            }
            _ => {
                println!("unhandled pkt: {}", &pkt.command_name);
            }
        }
    }
}