use std::sync::Arc;

use ricq_core::common::group_uin2code;
use ricq_core::structs::NewMember;
use ricq_core::{pb, RQError, RQResult};

use crate::client::event::NewMemberEvent;
use crate::handler::QEvent;
use crate::Client;

impl Client {
    pub(crate) async fn process_join_group(
        self: &Arc<Self>,
        msg: pb::msg::Message,
    ) -> RQResult<()> {
        let head = msg.head.ok_or(RQError::EmptyField("msg.head"))?;
        let group_code = group_uin2code(head.from_uin());
        let member_uin = head.auth_uin();

        self.handler
            .handle(QEvent::NewMember(NewMemberEvent {
                client: self.clone(),
                inner: NewMember {
                    group_code,
                    member_uin,
                },
            }))
            .await;

        Ok(())
    }
}
