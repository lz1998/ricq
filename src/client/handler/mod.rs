use std::sync::Arc;

use async_trait::async_trait;
use tokio::sync::{
    broadcast::Sender as BroadcastSender,
    mpsc::{Sender as MpscSender, UnboundedSender},
    watch::Sender as WatchSender,
};

use crate::engine::*;
use crate::Client;

/// 所有需要外发的数据的枚举打包
#[derive(Clone, derivative::Derivative)]
#[derivative(Debug)]
pub enum QEvent {
    TcpConnect,
    TcpDisconnect,
    /// 登录成功事件
    LoginEvent(i64),
    /// 群消息
    GroupMessage(
        #[derivative(Debug = "ignore")] Arc<Client>,
        GroupMessageEvent,
    ),
    /// 群自身消息
    SelfGroupMessage(
        #[derivative(Debug = "ignore")] Arc<Client>,
        GroupMessageEvent,
    ),
    /// 私聊消息
    PrivateMessage(
        #[derivative(Debug = "ignore")] Arc<Client>,
        PrivateMessageEvent,
    ),
    // FriendList(decoder::friendlist::FriendListResponse),
    // GroupMemberInfo(structs::GroupMemberInfo),

    // 群消息发送成功事件 内部处理
    // GroupMessageReceipt(GroupMessageReceiptEvent),
}

/// 处理外发数据的接口
#[async_trait]
pub trait Handler: Sync {
    async fn handle(&self, msg: QEvent) {
        match msg {
            QEvent::LoginEvent(uin) => self.handle_login_event(uin).await,
            QEvent::GroupMessage(_, group_message) => {
                self.handle_group_message(group_message).await
            }
            QEvent::SelfGroupMessage(_, group_message) => {
                self.handle_self_group_message(group_message).await
            }
            QEvent::PrivateMessage(_, private_message) => {
                self.handle_private_message(private_message).await
            }
            QEvent::TcpConnect => self.handle_tcp_connect_event().await,
            QEvent::TcpDisconnect => self.handle_tcp_connect_event().await,
        }
    }
    async fn handle_login_event(&self, _uin: i64) {}
    async fn handle_tcp_connect_event(&self) {}
    async fn handle_tcp_disconnect_event(&self) {}
    async fn handle_group_message(&self, _group_message: GroupMessageEvent) {}
    async fn handle_self_group_message(&self, _group_message: GroupMessageEvent) {}
    async fn handle_private_message(&self, _private_message: PrivateMessageEvent) {}
}

/// 一个默认 Handler，只是把信息打印出来
pub struct DefaultHandler;

#[async_trait]
impl Handler for DefaultHandler {
    async fn handle(&self, e: QEvent) {
        match e {
            QEvent::GroupMessage(_, m) => {
                println!("MESSAGE (GROUP={}): {}", m.group_code, m.elements)
            }
            QEvent::PrivateMessage(_, m) => {
                println!("MESSAGE (FRIEND={}): {}", m.sender.uin, m.elements)
            }
            _ => println!("{:?}", e),
        }
    }
}

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
