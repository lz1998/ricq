use crate::engine::*;
use async_trait::async_trait;

/// 所有需要外发的数据的枚举打包
#[derive(Debug)]
pub enum QEvent {
    /// 登录成功事件
    LoginEvent(i64),
    /// 群消息撤回
    GroupMessageReceipt(GroupMessageReceiptEvent),
    /// 群消息
    GroupMessage(GroupMessageEvent),
    /// 群自身消息
    SelfGroupMessage(GroupMessageEvent),
    // FriendList(decoder::friendlist::FriendListResponse),
    // GroupMemberInfo(structs::GroupMemberInfo),
}

/// 处理外发数据的接口
#[async_trait]
pub trait Handler: Sync {
    async fn handle(&self, msg: QEvent) {
        match msg {
            QEvent::LoginEvent(uin) => self.handle_login_event(uin).await,
            QEvent::GroupMessageReceipt(group_message_receipt_event) => {
                self.handle_group_message_receipt(group_message_receipt_event)
                    .await
            }
            QEvent::GroupMessage(group_message) => self.handle_group_message(group_message).await,
            QEvent::SelfGroupMessage(group_message) => {
                self.handle_self_group_message(group_message).await
            }
        }
    }
    async fn handle_login_event(&self, _uin: i64) {}
    async fn handle_group_message_receipt(
        &self,
        _group_message_receipt_event: GroupMessageReceiptEvent,
    ) {
    }
    async fn handle_group_message(&self, _group_message: GroupMessageEvent) {}
    async fn handle_self_group_message(&self, _group_message: GroupMessageEvent) {}
}

/// 一个默认 Handler，只是把信息打印出来
pub struct DefaultHandler;

#[async_trait]
impl Handler for DefaultHandler {
    async fn handle(&self, msgs: QEvent) {
        println!("{:?}", msgs);
    }
}

use tokio::sync::{
    broadcast::Sender as BroadcastSender,
    mpsc::{Sender as MpscSender, UnboundedSender},
    watch::Sender as WatchSender,
};

#[async_trait]
impl Handler for BroadcastSender<QEvent> {
    async fn handle(&self, msg: QEvent) {
        self.send(msg).unwrap();
    }
}

#[async_trait]
impl Handler for MpscSender<QEvent> {
    async fn handle(&self, msg: QEvent) {
        self.send(msg).await.unwrap();
    }
}

#[async_trait]
impl Handler for UnboundedSender<QEvent> {
    async fn handle(&self, msg: QEvent) {
        self.send(msg).unwrap();
    }
}

#[async_trait]
impl Handler for WatchSender<QEvent> {
    async fn handle(&self, msg: QEvent) {
        self.send(msg).unwrap();
    }
}
