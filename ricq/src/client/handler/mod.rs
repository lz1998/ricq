use std::future::Future;
use std::pin::Pin;

use async_trait::async_trait;
use tokio::sync::{broadcast, mpsc, watch};

use crate::client::event::*;

/// 所有需要外发的数据的枚举打包
#[derive(Clone, derivative::Derivative)]
#[derivative(Debug)]
pub enum QEvent<H: RawHandler> {
    /// 登录成功事件
    Login(i64),
    /// 群消息
    GroupMessage(GroupMessageEvent<H>),
    /// 群语音
    GroupAudioMessage(GroupAudioMessageEvent<H>),
    /// 好友消息
    FriendMessage(FriendMessageEvent<H>),
    /// 群语音
    FriendAudioMessage(FriendAudioMessageEvent<H>),
    /// 群临时消息
    GroupTempMessage(GroupTempMessageEvent<H>),
    /// 加群申请
    GroupRequest(JoinGroupRequestEvent<H>),
    /// 加群申请
    SelfInvited(SelfInvitedEvent<H>),
    /// 加好友申请
    NewFriendRequest(NewFriendRequestEvent<H>),
    /// 新成员入群
    NewMember(NewMemberEvent<H>),
    /// 成员被禁言
    GroupMute(GroupMuteEvent<H>),
    /// 好友消息撤回
    FriendMessageRecall(FriendMessageRecallEvent<H>),
    /// 群消息撤回
    GroupMessageRecall(GroupMessageRecallEvent<H>),
    /// 新好友
    NewFriend(NewFriendEvent<H>),
    /// 退群/被踢
    GroupLeave(GroupLeaveEvent<H>),
    /// 群解散
    GroupDisband(GroupDisbandEvent<H>),
    /// 好友戳一戳
    FriendPoke(FriendPokeEvent<H>),
    /// 群名称修改
    GroupNameUpdate(GroupNameUpdateEvent<H>),
    /// 好友删除
    DeleteFriend(DeleteFriendEvent<H>),
    /// 群成员权限变更
    MemberPermissionChange(MemberPermissionChangeEvent<H>),
    /// 被其他客户端踢下线。**不能用于掉线重连，掉线重连以 start 返回为准**
    KickedOffline(KickedOfflineEvent<H>),
    /// 服务端强制下线。**不能用于掉线重连，掉线重连以 start 返回为准**
    MSFOffline(MSFOfflineEvent<H>),
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
macro_rules! gen_raw_handler_fns {
    ($snake:ident, $e:ty) => {
        fn $snake<'a: 'b, 'b>(&'a self, _event: $e) -> HandlerRet<'b>
        where Self:Sized,
        {
            gen_handler_ret()
        }
    };
}

#[rustfmt::skip]
pub trait RawHandler: Sync + Send + 'static {
    gen_raw_handler_fns!(handle_login, i64);
    gen_raw_handler_fns!(handle_group_message, GroupMessageEvent<Self>);
    gen_raw_handler_fns!(handle_group_audio, GroupAudioMessageEvent<Self>);
    gen_raw_handler_fns!(handle_friend_message, FriendMessageEvent<Self>);
    gen_raw_handler_fns!(handle_friend_audio, FriendAudioMessageEvent<Self>);
    gen_raw_handler_fns!(handle_group_temp_message, GroupTempMessageEvent<Self>);
    gen_raw_handler_fns!(handle_group_request, JoinGroupRequestEvent<Self>);
    gen_raw_handler_fns!(handle_self_invited, SelfInvitedEvent<Self>);
    gen_raw_handler_fns!(handle_friend_request, NewFriendRequestEvent<Self>);
    gen_raw_handler_fns!(handle_new_member, NewMemberEvent<Self>);
    gen_raw_handler_fns!(handle_group_mute, GroupMuteEvent<Self>);
    gen_raw_handler_fns!(handle_friend_message_recall, FriendMessageRecallEvent<Self>);
    gen_raw_handler_fns!(handle_group_message_recall, GroupMessageRecallEvent<Self>);
    gen_raw_handler_fns!(handle_new_friend, NewFriendEvent<Self>);
    gen_raw_handler_fns!(handle_group_leave, GroupLeaveEvent<Self>);
    gen_raw_handler_fns!(handle_group_disband, GroupDisbandEvent<Self>);
    gen_raw_handler_fns!(handle_friend_poke, FriendPokeEvent<Self>);
    gen_raw_handler_fns!(handle_group_name_update, GroupNameUpdateEvent<Self>);
    gen_raw_handler_fns!(handle_delete_friend, DeleteFriendEvent<Self>);
    gen_raw_handler_fns!(handle_member_permission_change, MemberPermissionChangeEvent<Self>);
    gen_raw_handler_fns!(handle_kicked_offline, KickedOfflineEvent<Self>);
    gen_raw_handler_fns!(handle_msf_offline, MSFOfflineEvent<Self>);
}

// macro_rules! gen_handler_fns {
//     ($snake:ident, $e:ty,$v:tt) => {
//         fn $snake<'a: 'b, 'b>(&'a self, event: $e) -> HandlerRet<'b>
//         where Self:Sized,
//         {
//             Box::pin(async{
//                 self.handle(QEvent::$v(event)).await;
//             })
//         }
//     };
// }

// TODO: using macros
#[rustfmt::skip]
#[async_trait]
impl<T: Handler> RawHandler for T {
    // gen_handler_fns!(handle_group_message, GroupMessageEvent<T>, GroupMessage);
    async fn handle_login(&self, e: i64) { self.handle(QEvent::<Self>::Login(e)).await }
    async fn handle_group_message(&self, e: GroupMessageEvent<T>) { self.handle(QEvent::GroupMessage(e)).await }
    async fn handle_group_audio(&self, e: GroupAudioMessageEvent<T>) { self.handle(QEvent::GroupAudioMessage(e)).await }
    async fn handle_friend_message(&self, e: FriendMessageEvent<T>) { self.handle(QEvent::FriendMessage(e)).await }
    async fn handle_friend_audio(&self, e: FriendAudioMessageEvent<T>) { self.handle(QEvent::FriendAudioMessage(e)).await }
    async fn handle_group_temp_message(&self, e: GroupTempMessageEvent<T>) { self.handle(QEvent::GroupTempMessage(e)).await }
    async fn handle_group_request(&self, e: JoinGroupRequestEvent<T>) { self.handle(QEvent::GroupRequest(e)).await }
    async fn handle_self_invited(&self, e: SelfInvitedEvent<T>) { self.handle(QEvent::SelfInvited(e)).await }
    async fn handle_friend_request(&self, e: NewFriendRequestEvent<T>) { self.handle(QEvent::NewFriendRequest(e)).await }
    async fn handle_new_member(&self, e: NewMemberEvent<T>) { self.handle(QEvent::NewMember(e)).await }
    async fn handle_group_mute(&self, e: GroupMuteEvent<T>) { self.handle(QEvent::GroupMute(e)).await }
    async fn handle_friend_message_recall(&self, e: FriendMessageRecallEvent<T>) { self.handle(QEvent::FriendMessageRecall(e)).await }
    async fn handle_group_message_recall(&self, e: GroupMessageRecallEvent<T>) { self.handle(QEvent::GroupMessageRecall(e)).await }
    async fn handle_new_friend(&self, e: NewFriendEvent<T>) { self.handle(QEvent::NewFriend(e)).await }
    async fn handle_group_leave(&self, e: GroupLeaveEvent<T>) { self.handle(QEvent::GroupLeave(e)).await }
    async fn handle_group_disband(&self, e: GroupDisbandEvent<T>) { self.handle(QEvent::GroupDisband(e)).await }
    async fn handle_friend_poke(&self, e: FriendPokeEvent<T>) { self.handle(QEvent::FriendPoke(e)).await }
    async fn handle_group_name_update(&self, e: GroupNameUpdateEvent<T>) { self.handle(QEvent::GroupNameUpdate(e)).await }
    async fn handle_delete_friend(&self, e: DeleteFriendEvent<T>) { self.handle(QEvent::DeleteFriend(e)).await }
    async fn handle_member_permission_change(&self, e: MemberPermissionChangeEvent<T>) { self.handle(QEvent::MemberPermissionChange(e)).await }
    async fn handle_kicked_offline(&self, e: KickedOfflineEvent<T>) { self.handle(QEvent::KickedOffline(e)).await }
    async fn handle_msf_offline(&self, e: MSFOfflineEvent<T>) { self.handle(QEvent::MSFOffline(e)).await }
}

/// 事件处理器。
#[async_trait]
pub trait Handler: Sync + Send + 'static {
    /// 所有事件都会被包装为 QEvent 并在这里接收
    async fn handle<T: RawHandler>(&self, e: QEvent<T>);
}

/// 一个默认 Handler，只是把信息打印出来
#[derive(Debug)]
pub struct DefaultHandler;

// #[async_trait]
// impl <D>Handler for DefaultHandler {
//     async fn handle<T: RawHandler + std::fmt::Debug>(&self, e: QEvent<T>) {
//         tracing::info!("DefaultHandler::handle: {:?}", e);
//     }
// }

// #[async_trait]
// impl<T: RawHandler> Handler for broadcast::Sender<QEvent<T>> {
//     async fn handle(&self, msg: QEvent<T>) {
//         self.send(msg).ok();
//     }
// }

// #[async_trait]
// impl<T: RawHandler> Handler for mpsc::Sender<QEvent<T>> {
//     async fn handle(&self, msg: QEvent<T>) {
//         self.send(msg).await.ok();
//     }
// }

// #[async_trait]
// impl<T: RawHandler> Handler for mpsc::UnboundedSender<QEvent<T>> {
//     async fn handle(&self, msg: QEvent<T>) {
//         self.send(msg).ok();
//     }
// }

// #[async_trait]
// impl<T: RawHandler> Handler for watch::Sender<QEvent<T>> {
//     async fn handle(&self, msg: QEvent<T>) {
//         self.send(msg).ok();
//     }
// }
