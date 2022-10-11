use async_trait::async_trait;
use tokio::sync::{
    broadcast::Sender as BroadcastSender,
    mpsc::{Sender as MpscSender, UnboundedSender},
    watch::Sender as WatchSender,
};

use crate::client::event::*;

/// 所有需要外发的数据的枚举打包
#[derive(Clone, derivative::Derivative)]
#[derivative(Debug)]
pub enum QEvent {
    /// 登录成功事件
    Login(i64),
    /// 群消息
    GroupMessage(GroupMessageEvent),
    /// 群语音
    GroupAudioMessage(GroupAudioMessageEvent),
    /// 好友消息
    FriendMessage(FriendMessageEvent),
    /// 群语音
    FriendAudioMessage(FriendAudioMessageEvent),
    /// 群临时消息
    GroupTempMessage(GroupTempMessageEvent),
    /// 加群申请
    GroupRequest(JoinGroupRequestEvent),
    /// 加群申请
    SelfInvited(SelfInvitedEvent),
    /// 加好友申请
    NewFriendRequest(NewFriendRequestEvent),
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
    /// 群解散
    GroupDisband(GroupDisbandEvent),
    /// 好友戳一戳
    FriendPoke(FriendPokeEvent),
    /// 群名称修改
    GroupNameUpdate(GroupNameUpdateEvent),
    /// 好友删除
    DeleteFriend(DeleteFriendEvent),
    /// 群成员权限变更
    MemberPermissionChange(MemberPermissionChangeEvent),
    /// 被其他客户端踢下线。**不能用于掉线重连，掉线重连以 start 返回为准**
    KickedOffline(KickedOfflineEvent),
    /// 服务端强制下线。**不能用于掉线重连，掉线重连以 start 返回为准**
    MSFOffline(MSFOfflineEvent),
}

/// 事件处理器。
/// 
/// 接收到事件时，事件类型对应的 `fn handle_ ...` 会被调用。默认情况下，所有事件处理函数都实现为：将事件包装成 QEvent 并转发到 `async fn handle(&self, e: QEvent)`。
/// 
/// # 示例
/// 
/// ```
/// struct MyHandler;
/// #[async_trait]
/// impl Handler for MyHandler {
///     /// 群消息事件
///     async fn handle_group_message(&self, e: GroupMessageEvent) {
///         tracing::info!("MyHandler::handle_group_message: {:?}", e.inner);
///         // 这里没有转发到 fn handle，所以 handle 只能收到……
///     }
///     /// 除 群消息 外所有事件
///     async fn handle(&self, e: QEvent) {
///         tracing::info!("MyHandler::handle: {:?}", e);
///     }
/// }
/// ```
#[rustfmt::skip]
#[async_trait]
pub trait Handler: Sync {
    /// 默认情况下，所有事件都会被包装为 QEvent 并在这里接收
    async fn handle(&self, e: QEvent); // 不默认实现，提醒用户还有其他事件
    async fn handle_login(&self, e: i64) { self.handle(QEvent::Login(e)).await }
    async fn handle_group_message(&self, e: GroupMessageEvent) { self.handle(QEvent::GroupMessage(e)).await }
    async fn handle_group_audio(&self, e: GroupAudioMessageEvent) { self.handle(QEvent::GroupAudioMessage(e)).await }
    async fn handle_friend_message(&self, e: FriendMessageEvent) { self.handle(QEvent::FriendMessage(e)).await }
    async fn handle_friend_audio(&self, e: FriendAudioMessageEvent) { self.handle(QEvent::FriendAudioMessage(e)).await }
    async fn handle_group_temp_message(&self, e: GroupTempMessageEvent) { self.handle(QEvent::GroupTempMessage(e)).await }
    async fn handle_group_request(&self, e: JoinGroupRequestEvent) { self.handle(QEvent::GroupRequest(e)).await }
    async fn handle_self_invited(&self, e: SelfInvitedEvent) { self.handle(QEvent::SelfInvited(e)).await }
    async fn handle_friend_request(&self, e: NewFriendRequestEvent) { self.handle(QEvent::NewFriendRequest(e)).await }
    async fn handle_new_member(&self, e: NewMemberEvent) { self.handle(QEvent::NewMember(e)).await }
    async fn handle_group_mute(&self, e: GroupMuteEvent) { self.handle(QEvent::GroupMute(e)).await }
    async fn handle_friend_message_recall(&self, e: FriendMessageRecallEvent) { self.handle(QEvent::FriendMessageRecall(e)).await }
    async fn handle_group_message_recall(&self, e: GroupMessageRecallEvent) { self.handle(QEvent::GroupMessageRecall(e)).await }
    async fn handle_new_friend(&self, e: NewFriendEvent) { self.handle(QEvent::NewFriend(e)).await }
    async fn handle_group_leave(&self, e: GroupLeaveEvent) { self.handle(QEvent::GroupLeave(e)).await }
    async fn handle_group_disband(&self, e: GroupDisbandEvent) { self.handle(QEvent::GroupDisband(e)).await }
    async fn handle_friend_poke(&self, e: FriendPokeEvent) { self.handle(QEvent::FriendPoke(e)).await }
    async fn handle_group_name_update(&self, e: GroupNameUpdateEvent) { self.handle(QEvent::GroupNameUpdate(e)).await }
    async fn handle_delete_friend(&self, e: DeleteFriendEvent) { self.handle(QEvent::DeleteFriend(e)).await }
    async fn handle_member_permission_change(&self, e: MemberPermissionChangeEvent) { self.handle(QEvent::MemberPermissionChange(e)).await }
    async fn handle_kicked_offline(&self, e: KickedOfflineEvent) { self.handle(QEvent::KickedOffline(e)).await }
    async fn handle_msf_offline(&self, e: MSFOfflineEvent) { self.handle(QEvent::MSFOffline(e)).await }
}

/// 一个默认 Handler，只是把信息打印出来
pub struct DefaultHandler;

#[async_trait]
impl Handler for DefaultHandler {
    async fn handle_group_message(&self, e: GroupMessageEvent) {
        tracing::info!("DefaultHandler::handle_group_message: {:?}", e.inner);
    }
    async fn handle_group_temp_message(&self, e: GroupTempMessageEvent) {
        tracing::info!("DefaultHandler::handle_group_temp_message: {:?}", e.inner);
    }
    async fn handle_friend_message(&self, e: FriendMessageEvent) {
        tracing::info!("DefaultHandler::handle_friend_message: {:?}", e.inner);
    }
    async fn handle_group_request(&self, e: JoinGroupRequestEvent) {
        tracing::info!("DefaultHandler::handle_group_request: {:?}", e.inner);
    }
    async fn handle_friend_request(&self, e: NewFriendRequestEvent) {
        tracing::info!("DefaultHandler::handle_friend_request: {:?}", e.inner);
    }
    /// 其他事件在这里输出
    async fn handle(&self, e: QEvent) {
        tracing::info!("DefaultHandler::handle: {:?}", e);
    }
}

#[async_trait]
impl Handler for BroadcastSender<QEvent> {
    async fn handle(&self, msg: QEvent) {
        self.send(msg).ok();
    }
}

#[async_trait]
impl Handler for MpscSender<QEvent> {
    async fn handle(&self, msg: QEvent) {
        self.send(msg).await.ok();
    }
}

#[async_trait]
impl Handler for UnboundedSender<QEvent> {
    async fn handle(&self, msg: QEvent) {
        self.send(msg).ok();
    }
}

#[async_trait]
impl Handler for WatchSender<QEvent> {
    async fn handle(&self, msg: QEvent) {
        self.send(msg).ok();
    }
}
