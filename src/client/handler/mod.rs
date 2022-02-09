use async_trait::async_trait;
use tokio::sync::{
    broadcast::Sender as BroadcastSender,
    mpsc::{Sender as MpscSender, UnboundedSender},
    watch::Sender as WatchSender,
};

use crate::client::event::{
    DeleteFriendEvent, FriendMessageRecallEvent, FriendPokeEvent, FriendRequestEvent,
    GroupLeaveEvent, GroupMessageEvent, GroupMessageRecallEvent, GroupMuteEvent,
    GroupNameUpdateEvent, GroupRequestEvent, MemberPermissionChangeEvent, NewFriendEvent,
    NewMemberEvent, PrivateMessageEvent, SelfInvitedEvent,
};

/// 所有需要外发的数据的枚举打包
#[derive(Clone, derivative::Derivative)]
#[derivative(Debug)]
pub enum QEvent {
    TcpConnect,
    TcpDisconnect,
    /// 登录成功事件
    LoginEvent(i64),
    /// 群消息
    GroupMessage(GroupMessageEvent),
    /// 群自身消息
    SelfGroupMessage(GroupMessageEvent),
    /// 私聊消息
    PrivateMessage(PrivateMessageEvent),
    /// 加群申请
    GroupRequest(GroupRequestEvent),
    /// 加群申请
    SelfInvited(SelfInvitedEvent),
    /// 加好友申请
    FriendRequest(FriendRequestEvent),
    /// 新成员入群
    NewMember(NewMemberEvent),
    /// 成员被禁言
    GroupMute(GroupMuteEvent),
    /// 好友消息撤回
    FriendMessageRecall(FriendMessageRecallEvent),
    /// 群消息撤回
    GroupMessageRecall(GroupMessageRecallEvent),
    /// 新好友
    NewFriend(NewFriendEvent),
    /// 退群/被踢
    GroupLeave(GroupLeaveEvent),
    /// 好友戳一戳
    FriendPoke(FriendPokeEvent),
    /// 群名称修改
    GroupNameUpdate(GroupNameUpdateEvent),
    /// 好友删除
    DeleteFriend(DeleteFriendEvent),
    /// 群成员权限变更
    MemberPermissionChange(MemberPermissionChangeEvent),
    // FriendList(decoder::friendlist::FriendListResponse),
    // GroupMemberInfo(structs::GroupMemberInfo),

    // 群消息发送成功事件 内部处理
    // GroupMessageReceipt(GroupMessageReceiptEvent)
}

/// 处理外发数据的接口
#[async_trait]
pub trait Handler: Sync {
    async fn handle(&self, _msg: QEvent);
}

/// 一个默认 Handler，只是把信息打印出来
pub struct DefaultHandler;

#[async_trait]
impl Handler for DefaultHandler {
    async fn handle(&self, e: QEvent) {
        match e {
            QEvent::GroupMessage(m) => {
                println!(
                    "MESSAGE (GROUP={}): {}",
                    m.message.group_code, m.message.elements
                )
            }
            QEvent::PrivateMessage(m) => {
                println!(
                    "MESSAGE (FRIEND={}): {}",
                    m.message.from_uin, m.message.elements
                )
            }
            QEvent::GroupRequest(m) => {
                println!(
                    "REQUEST (GROUP={}, UIN={}): {}",
                    m.request.group_code, m.request.req_uin, m.request.message
                )
            }
            QEvent::FriendRequest(m) => {
                println!("REQUEST (UIN={}): {}", m.request.req_uin, m.request.message)
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
