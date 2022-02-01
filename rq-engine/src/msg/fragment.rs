use crate::pb::msg;

use super::MessageChain;

impl MessageChain {
    // TODO test
    // https://github.com/mamoe/mirai/blob/dev/mirai-core/src/commonMain/kotlin/network/protocol/packet/chat/receive/MessageSvc.PbSendMsg.kt#L68
    pub fn fragment(mut self) -> Vec<MessageChain> {
        let mut results = vec![];
        let mut txt_add = false;
        let mut last = vec![];
        fn flush(
            txt_add: &mut bool,
            last: &mut Vec<msg::elem::Elem>,
            results: &mut Vec<MessageChain>,
        ) {
            *txt_add = false;
            if !last.is_empty() {
                results.push(MessageChain(last.clone()));
                last.clear()
            }
        }
        self.0.iter_mut().for_each(|element| {
            if last.len() >= 4 {
                flush(&mut txt_add, &mut last, &mut results);
            }
            if let msg::elem::Elem::Text(t) = element {
                if txt_add {
                    flush(&mut txt_add, &mut last, &mut results);
                }
                if t.str.clone().unwrap_or_default().len() < 80 {
                    txt_add = true;
                    last.push(element.clone());
                } else {
                    let split = t
                        .str
                        .clone()
                        .unwrap_or_default()
                        .chars()
                        .collect::<Vec<char>>()
                        .chunks(4)
                        .map(|c| c.iter().collect::<String>())
                        .collect::<Vec<String>>();
                    flush(&mut txt_add, &mut last, &mut results);
                    split.into_iter().for_each(|s| {
                        results.push(MessageChain(vec![msg::elem::Elem::Text(msg::Text {
                            str: Some(s),
                            ..Default::default()
                        })]))
                    });
                }
            } else {
                last.push(element.clone());
            }
        });
        flush(&mut txt_add, &mut last, &mut results);
        results
    }
}
