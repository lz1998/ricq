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
    NewMemberEvent, PrivateMessageEvent, SelfInvitedEvent, TempMessageEvent,
};

/// 所有需要外发的数据的枚举打包
#[derive(Clone, derivative::Derivative)]
#[derivative(Debug)]
pub enum QEvent {
    TcpConnect,
    TcpDisconnect,
    /// 登录成功事件
    Login(i64),
    /// 群消息
    GroupMessage(GroupMessageEvent),
    /// 群自身消息
    SelfGroupMessage(GroupMessageEvent),
    /// 私聊消息
    PrivateMessage(PrivateMessageEvent),
    /// 私聊消息
    TempMessage(TempMessageEvent),
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
    async fn handle(&self, _event: QEvent);
}

/// 一个默认 Handler，只是把信息打印出来
pub struct DefaultHandler;

#[async_trait]
impl Handler for DefaultHandler {
    async fn handle(&self, e: QEvent) {
        match e {
            QEvent::GroupMessage(m) => {
                tracing::info!(
                    target = "rs_qq",
                    "MESSAGE (GROUP={}): {}",
                    m.message.group_code,
                    m.message.elements
                )
            }
            QEvent::PrivateMessage(m) => {
                tracing::info!(
                    target = "rs_qq",
                    "MESSAGE (FRIEND={}): {}",
                    m.message.from_uin,
                    m.message.elements
                )
            }
            QEvent::TempMessage(m) => {
                tracing::info!(
                    target = "rs_qq",
                    "MESSAGE (TEMP={}): {}",
                    m.message.from_uin,
                    m.message.elements
                )
            }
            QEvent::GroupRequest(m) => {
                tracing::info!(
                    target = "rs_qq",
                    "REQUEST (GROUP={}, UIN={}): {}",
                    m.request.group_code,
                    m.request.req_uin,
                    m.request.message
                )
            }
            QEvent::FriendRequest(m) => {
                tracing::info!(
                    target = "rs_qq",
                    "REQUEST (UIN={}): {}",
                    m.request.req_uin,
                    m.request.message
                )
            }
            _ => tracing::info!(target = "rs_qq", "{:?}", e),
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

#[async_trait]
pub trait PartlyHandler: Sync {
    async fn handle_connect(&self) {}
    async fn handle_disconnect(&self) {}
    async fn handle_login(&self, _: i64) {}
    async fn handle_group_message(&self, _event: GroupMessageEvent) {}
    async fn handle_self_group_message(&self, _event: GroupMessageEvent) {}
    async fn handle_private_message(&self, _event: PrivateMessageEvent) {}
    async fn handle_temp_message(&self, _event: TempMessageEvent) {}
    async fn handle_group_request(&self, _event: GroupRequestEvent) {}
    async fn handle_self_invited(&self, _event: SelfInvitedEvent) {}
    async fn handle_friend_request(&self, _event: FriendRequestEvent) {}
    async fn handle_new_member(&self, _event: NewMemberEvent) {}
    async fn handle_group_mute(&self, _event: GroupMuteEvent) {}
    async fn handle_friend_message_recall(&self, _event: FriendMessageRecallEvent) {}
    async fn handle_group_message_recall(&self, _event: GroupMessageRecallEvent) {}
    async fn handle_new_friend(&self, _event: NewFriendEvent) {}
    async fn handle_group_leave(&self, _event: GroupLeaveEvent) {}
    async fn handle_friend_poke(&self, _event: FriendPokeEvent) {}
    async fn handle_group_name_update(&self, _event: GroupNameUpdateEvent) {}
    async fn handle_delete_friend(&self, _event: DeleteFriendEvent) {}
    async fn handle_member_permission_change(&self, _event: MemberPermissionChangeEvent) {}
}

#[async_trait]
impl<PH> Handler for PH
where
    PH: PartlyHandler,
{
    async fn handle(&self, event: QEvent) {
        match event {
            QEvent::TcpConnect => self.handle_connect().await,
            QEvent::TcpDisconnect => self.handle_disconnect().await,
            QEvent::Login(uin) => self.handle_login(uin).await,
            QEvent::GroupMessage(m) => self.handle_group_message(m).await,
            QEvent::SelfGroupMessage(m) => self.handle_self_group_message(m).await,
            QEvent::PrivateMessage(m) => self.handle_private_message(m).await,
            QEvent::TempMessage(m) => self.handle_temp_message(m).await,
            QEvent::GroupRequest(m) => self.handle_group_request(m).await,
            QEvent::SelfInvited(m) => self.handle_self_invited(m).await,
            QEvent::FriendRequest(m) => self.handle_friend_request(m).await,
            QEvent::NewMember(m) => self.handle_new_member(m).await,
            QEvent::GroupMute(m) => self.handle_group_mute(m).await,
            QEvent::FriendMessageRecall(m) => self.handle_friend_message_recall(m).await,
            QEvent::GroupMessageRecall(m) => self.handle_group_message_recall(m).await,
            QEvent::NewFriend(m) => self.handle_new_friend(m).await,
            QEvent::GroupLeave(m) => self.handle_group_leave(m).await,
            QEvent::FriendPoke(m) => self.handle_friend_poke(m).await,
            QEvent::GroupNameUpdate(m) => self.handle_group_name_update(m).await,
            QEvent::DeleteFriend(m) => self.handle_delete_friend(m).await,
            QEvent::MemberPermissionChange(m) => self.handle_member_permission_change(m).await,
        }
    }
}
