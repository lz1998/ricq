use std::future::Future;
use std::pin::Pin;

use async_trait::async_trait;
use tokio::sync::{broadcast, mpsc, watch};

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

/*
pub trait RawHandler: Sync + Send + 'static {
    #[must_use]
    #[allow(clippy :: let_unit_value, clippy :: no_effect_underscore_binding,
    clippy :: shadow_same, clippy :: type_complexity, clippy ::
    type_repetition_in_bounds, clippy :: used_underscore_binding)]
    fn handle_login<'life0, 'async_trait>(&'life0 self, _uin: i64)
        ->
            ::core::pin::Pin<Box<dyn ::core::future::Future<Output = ()> +
            ::core::marker::Send + 'async_trait>> where 'life0: 'async_trait,
        Self: 'async_trait {
        Box::pin(async move
                { let __self = self; let _uin = _uin; let _: () = {}; })
    }
}
*/

type HandlerRet<'b> = Pin<Box<dyn Future<Output = ()> + Send + 'b>>;
fn gen_handler_ret<'b>() -> HandlerRet<'b> {
    // (), Ready<>, Box<>, Pin<>, all ZST?
    Box::pin(std::future::ready(()))
}
macro_rules! gen_handler_fns {
    ($snake:tt, $e:tt) => {
        fn $snake<'a: 'b, 'b>(&'a self, _event: $e) -> HandlerRet<'b> {
            gen_handler_ret()
        }
    };
}

pub trait RawHandler: Sync + Send + 'static {
    gen_handler_fns!(handle_login, i64);
    gen_handler_fns!(handle_group_message, GroupMessageEvent);
    gen_handler_fns!(handle_group_audio, GroupAudioMessageEvent);
    gen_handler_fns!(handle_friend_message, FriendMessageEvent);
    gen_handler_fns!(handle_friend_audio, FriendAudioMessageEvent);
    gen_handler_fns!(handle_group_temp_message, GroupTempMessageEvent);
    gen_handler_fns!(handle_group_request, JoinGroupRequestEvent);
    gen_handler_fns!(handle_self_invited, SelfInvitedEvent);
    gen_handler_fns!(handle_friend_request, NewFriendRequestEvent);
    gen_handler_fns!(handle_new_member, NewMemberEvent);
    gen_handler_fns!(handle_group_mute, GroupMuteEvent);
    gen_handler_fns!(handle_friend_message_recall, FriendMessageRecallEvent);
    gen_handler_fns!(handle_group_message_recall, GroupMessageRecallEvent);
    gen_handler_fns!(handle_new_friend, NewFriendEvent);
    gen_handler_fns!(handle_group_leave, GroupLeaveEvent);
    gen_handler_fns!(handle_group_disband, GroupDisbandEvent);
    gen_handler_fns!(handle_friend_poke, FriendPokeEvent);
    gen_handler_fns!(handle_group_name_update, GroupNameUpdateEvent);
    gen_handler_fns!(handle_delete_friend, DeleteFriendEvent);
    gen_handler_fns!(handle_member_permission_change, MemberPermissionChangeEvent);
    gen_handler_fns!(handle_kicked_offline, KickedOfflineEvent);
    gen_handler_fns!(handle_msf_offline, MSFOfflineEvent);
}

// TODO: using macros
#[rustfmt::skip]
#[async_trait]
impl<T: Handler> RawHandler for T {
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

/// 事件处理器。
#[async_trait]
pub trait Handler: Sync + Send + 'static {
    /// 所有事件都会被包装为 QEvent 并在这里接收
    async fn handle(&self, e: QEvent);
}

/// 一个默认 Handler，只是把信息打印出来
pub struct DefaultHandler;

#[async_trait]
impl Handler for DefaultHandler {
    async fn handle(&self, e: QEvent) {
        tracing::info!("DefaultHandler::handle: {:?}", e);
    }
}

#[async_trait]
impl Handler for broadcast::Sender<QEvent> {
    async fn handle(&self, msg: QEvent) {
        self.send(msg).ok();
    }
}

#[async_trait]
impl Handler for mpsc::Sender<QEvent> {
    async fn handle(&self, msg: QEvent) {
        self.send(msg).await.ok();
    }
}

#[async_trait]
impl Handler for mpsc::UnboundedSender<QEvent> {
    async fn handle(&self, msg: QEvent) {
        self.send(msg).ok();
    }
}

#[async_trait]
impl Handler for watch::Sender<QEvent> {
    async fn handle(&self, msg: QEvent) {
        self.send(msg).ok();
    }
}
