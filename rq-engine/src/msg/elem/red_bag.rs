use crate::pb::msg;
use crate::pb::msg::QqWalletMsg;

#[derive(Default, Debug, Clone)]
pub struct RedBag {
    pub msg_type: i32,
    pub title: String,
}

impl From<msg::QqWalletMsg> for RedBag {
    fn from(e: QqWalletMsg) -> Self {
        let aio_body = e.aio_body.unwrap_or_default();
        Self {
            msg_type: aio_body.msg_type(),
            title: aio_body
                .receiver
                .unwrap_or_default()
                .title
                .unwrap_or_default(),
        }
    }
}

// TODO display msg_type
