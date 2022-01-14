use crate::client::msg::*;
use async_trait::async_trait;

/// 所有需要外发的数据的枚举打包
#[derive(Debug)]
pub enum Msg {
    /// 群消息撤回
    GroupMessageReceipt(GroupMessage),
    /// 群消息
    GroupMessage(GroupMessage),
    /// 群自身消息
    SelfGroupMessage(GroupMessage),
    // FriendList(decoder::friendlist::FriendListResponse),
    // GroupMemberInfo(structs::GroupMemberInfo),
}

/// 处理外发数据的接口
#[async_trait]
pub trait Handler: Sync {
    async fn handle(&self, msg: Msg) {
        match msg {
            Msg::GroupMessageReceipt(group_message) => {
                self.handle_group_message_receipt(group_message).await
            }
            Msg::GroupMessage(group_message) => self.handle_group_message(group_message).await,
            Msg::SelfGroupMessage(group_message) => {
                self.handle_self_group_message(group_message).await
            }
        }
    }
    async fn handle_group_message_receipt(&self, _group_message: GroupMessage) {}
    async fn handle_group_message(&self, _group_message: GroupMessage) {}
    async fn handle_self_group_message(&self, _group_message: GroupMessage) {}
}

/// 一个默认 Handler，只是把信息打印出来
pub struct DefaultHandler;

#[async_trait]
impl Handler for DefaultHandler {
    async fn handle(&self, msgs: Msg) {
        println!("{:?}", msgs);
    }
}

use tokio::sync::{
    broadcast::Sender as BroadcastSender,
    mpsc::{Sender as MpscSender, UnboundedSender},
    watch::Sender as WatchSender,
};

#[async_trait]
impl Handler for BroadcastSender<Msg> {
    async fn handle(&self, msg: Msg) {
        self.send(msg).unwrap();
    }
}

#[async_trait]
impl Handler for MpscSender<Msg> {
    async fn handle(&self, msg: Msg) {
        self.send(msg).await.unwrap();
    }
}

#[async_trait]
impl Handler for UnboundedSender<Msg> {
    async fn handle(&self, msg: Msg) {
        self.send(msg).unwrap();
    }
}

#[async_trait]
impl Handler for WatchSender<Msg> {
    async fn handle(&self, msg: Msg) {
        self.send(msg).unwrap();
    }
}
