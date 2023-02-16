use std::future::Future;
use std::pin::Pin;

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
    /// 群成员戳一戳
    GroupPoke(GroupPokeEvent),
    /// 群名称修改
    GroupNameUpdate(GroupNameUpdateEvent),
    /// 好友删除
    DeleteFriend(DeleteFriendEvent),
    /// 群成员权限变更
    MemberPermissionChange(MemberPermissionChangeEvent),
    /// 被其他客户端踢下线
    /// 不能用于掉线重连，掉线重连以 start 返回为准
    KickedOffline(KickedOfflineEvent),
    /// 服务端强制下线
    /// 不能用于掉线重连，掉线重连以 start 返回为准
    MSFOffline(MSFOfflineEvent),
    /// 网络原因/客户端主动掉线
    /// 可用于掉线重连
    ClientDisconnect(ClientDisconnect),
}

/// 处理外发数据的接口
///
/// 同时，所有 `async fn(QEvent)` 都已自动实现 `Handler`。
///
/// # Examples
///
/// 你可以为自己的 struct 实现 Handler：
///
/// ```
/// struct MyHandler;
/// impl Handler for MyHandler { ... }
/// ```
///
/// 或者只定义单个事件处理函数，更简洁：
///
/// ```
/// async fn on_event(e: QEvent) {
///     dbg!(e);
/// }
/// let client = Client::new(
///     device,
///     Protocol::MacOS.into(),
///     on_event as fn(_) -> _,
/// );
/// ```
#[async_trait]
pub trait Handler: Sync {
    async fn handle(&self, event: QEvent);
}

// 这里还有一种 Fn(QEvent) -> Fut 的写法，但是会与 PartlyHandler 冲突
impl<Fut> Handler for fn(QEvent) -> Fut
where
    Fut: Future<Output = ()> + Send,
{
    fn handle<'a: 'b, 'b>(&'a self, e: QEvent) -> Pin<Box<dyn Future<Output = ()> + Send + 'b>> {
        Box::pin(async move { self(e).await })
    }
}

/// 一个默认 Handler，只是把信息打印出来
pub struct DefaultHandler;

#[async_trait]
impl Handler for DefaultHandler {
    async fn handle(&self, e: QEvent) {
        match e {
            QEvent::GroupMessage(m) => {
                tracing::info!(
                    "MESSAGE (GROUP={}): {}",
                    m.inner.group_code,
                    m.inner.elements
                )
            }
            QEvent::FriendMessage(m) => {
                tracing::info!(
                    "MESSAGE (FRIEND={}): {}",
                    m.inner.from_uin,
                    m.inner.elements
                )
            }
            QEvent::GroupTempMessage(m) => {
                tracing::info!("MESSAGE (TEMP={}): {}", m.inner.from_uin, m.inner.elements)
            }
            QEvent::GroupRequest(m) => {
                tracing::info!(
                    "REQUEST (GROUP={}, UIN={}): {}",
                    m.inner.group_code,
                    m.inner.req_uin,
                    m.inner.message
                )
            }
            QEvent::NewFriendRequest(m) => {
                tracing::info!("REQUEST (UIN={}): {}", m.inner.req_uin, m.inner.message)
            }
            _ => tracing::info!("{:?}", e),
        }
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

#[async_trait]
pub trait PartlyHandler: Sync {
    async fn handle_login(&self, _: i64) {}
    async fn handle_group_message(&self, _event: GroupMessageEvent) {}
    async fn handle_group_audio(&self, _event: GroupAudioMessageEvent) {}
    async fn handle_friend_message(&self, _event: FriendMessageEvent) {}
    async fn handle_friend_audio(&self, _event: FriendAudioMessageEvent) {}
    async fn handle_group_temp_message(&self, _event: GroupTempMessageEvent) {}
    async fn handle_group_request(&self, _event: JoinGroupRequestEvent) {}
    async fn handle_self_invited(&self, _event: SelfInvitedEvent) {}
    async fn handle_friend_request(&self, _event: NewFriendRequestEvent) {}
    async fn handle_new_member(&self, _event: NewMemberEvent) {}
    async fn handle_group_mute(&self, _event: GroupMuteEvent) {}
    async fn handle_friend_message_recall(&self, _event: FriendMessageRecallEvent) {}
    async fn handle_group_message_recall(&self, _event: GroupMessageRecallEvent) {}
    async fn handle_new_friend(&self, _event: NewFriendEvent) {}
    async fn handle_group_leave(&self, _event: GroupLeaveEvent) {}
    async fn handle_group_disband(&self, _event: GroupDisbandEvent) {}
    async fn handle_friend_poke(&self, _event: FriendPokeEvent) {}
    async fn handle_group_poke(&self, _event: GroupPokeEvent) {}
    async fn handle_group_name_update(&self, _event: GroupNameUpdateEvent) {}
    async fn handle_delete_friend(&self, _event: DeleteFriendEvent) {}
    async fn handle_member_permission_change(&self, _event: MemberPermissionChangeEvent) {}
    async fn handle_kicked_offline(&self, _event: KickedOfflineEvent) {}
    async fn handle_msf_offline(&self, _event: MSFOfflineEvent) {}
    async fn handle_client_disconnect(&self, _event: ClientDisconnect) {}
}

#[async_trait]
impl<PH> Handler for PH
where
    PH: PartlyHandler,
{
    async fn handle(&self, event: QEvent) {
        match event {
            QEvent::Login(uin) => self.handle_login(uin).await,
            QEvent::GroupMessage(m) => self.handle_group_message(m).await,
            QEvent::GroupAudioMessage(m) => self.handle_group_audio(m).await,
            QEvent::FriendMessage(m) => self.handle_friend_message(m).await,
            QEvent::FriendAudioMessage(m) => self.handle_friend_audio(m).await,
            QEvent::GroupTempMessage(m) => self.handle_group_temp_message(m).await,
            QEvent::GroupRequest(m) => self.handle_group_request(m).await,
            QEvent::SelfInvited(m) => self.handle_self_invited(m).await,
            QEvent::NewFriendRequest(m) => self.handle_friend_request(m).await,
            QEvent::NewMember(m) => self.handle_new_member(m).await,
            QEvent::GroupMute(m) => self.handle_group_mute(m).await,
            QEvent::FriendMessageRecall(m) => self.handle_friend_message_recall(m).await,
            QEvent::GroupMessageRecall(m) => self.handle_group_message_recall(m).await,
            QEvent::NewFriend(m) => self.handle_new_friend(m).await,
            QEvent::GroupLeave(m) => self.handle_group_leave(m).await,
            QEvent::GroupDisband(m) => self.handle_group_disband(m).await,
            QEvent::FriendPoke(m) => self.handle_friend_poke(m).await,
            QEvent::GroupPoke(m) => self.handle_group_poke(m).await,
            QEvent::GroupNameUpdate(m) => self.handle_group_name_update(m).await,
            QEvent::DeleteFriend(m) => self.handle_delete_friend(m).await,
            QEvent::MemberPermissionChange(m) => self.handle_member_permission_change(m).await,
            QEvent::KickedOffline(m) => self.handle_kicked_offline(m).await,
            QEvent::MSFOffline(m) => self.handle_msf_offline(m).await,
            QEvent::ClientDisconnect(m) => self.handle_client_disconnect(m).await,
        }
    }
}
