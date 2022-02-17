use std::sync::Arc;

use rq_engine::common::group_uin2code;
use rq_engine::structs::NewMember;
use rq_engine::{pb, RQError, RQResult};

use crate::client::event::NewMemberEvent;
use crate::handler::QEvent;
use crate::Client;

impl Client {
    pub(crate) async fn process_join_group(
        self: &Arc<Self>,
        msg: pb::msg::Message,
    ) -> RQResult<()> {
        let head = msg
            .head
            .ok_or_else(|| RQError::Other("missing head".into()))?;
        let group_code = group_uin2code(head.from_uin());
        let member_uin = head.auth_uin();

        let group = self
            .find_group(group_code, true)
            .await
            .ok_or_else(|| RQError::Other("group not found".into()))?;

        if member_uin == self.uin().await {
            // find_group 的时候已经 reload group info 了
            self.handler
                .handle(QEvent::NewMember(NewMemberEvent {
                    client: self.clone(),
                    new_member: NewMember {
                        group_code,
                        member_uin,
                    },
                }))
                .await;
        }

        let mut members = group.members.write().await;
        if members.iter().find(|m| m.uin == member_uin).is_none() {
            let member_info = self.get_group_member_info(group_code, member_uin).await?;
            members.push(member_info);
            self.handler
                .handle(QEvent::NewMember(NewMemberEvent {
                    client: self.clone(),
                    new_member: NewMember {
                        group_code,
                        member_uin,
                    },
                }))
                .await;
        }

        Ok(())
    }
}
