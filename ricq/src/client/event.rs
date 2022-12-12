use std::sync::Arc;

use ricq_core::command::profile_service::{JoinGroupRequest, NewFriendRequest, SelfInvited};
use ricq_core::structs::{
    DeleteFriend, FriendAudioMessage, FriendInfo, FriendMessageRecall, FriendPoke,
    GroupAudioMessage, GroupDisband, GroupLeave, GroupMessageRecall, GroupMute, GroupNameUpdate,
    GroupPoke, GroupTempMessage, MemberPermissionChange, NewMember,
};
use ricq_core::{jce, RQResult};

use crate::client::NetworkStatus;
use crate::structs::{FriendMessage, GroupMessage};
use crate::Client;

#[derive(Clone, derivative::Derivative)]
#[derivative(Debug)]
pub struct EventWithClient<T> {
    #[derivative(Debug = "ignore")]
    pub client: Arc<Client>,
    pub inner: T,
}

pub type GroupMessageEvent = EventWithClient<GroupMessage>;

impl GroupMessageEvent {
    pub async fn recall(&self) -> RQResult<()> {
        // TODO check permission
        self.client
            .recall_group_message(
                self.inner.group_code,
                self.inner.seqs.clone(),
                self.inner.rands.clone(),
            )
            .await
    }
}

pub type FriendMessageEvent = EventWithClient<FriendMessage>;
pub type GroupTempMessageEvent = EventWithClient<GroupTempMessage>;
pub type JoinGroupRequestEvent = EventWithClient<JoinGroupRequest>;

impl JoinGroupRequestEvent {
    pub async fn accept(&self) -> RQResult<()> {
        self.client
            .solve_group_system_message(
                self.inner.msg_seq,
                self.inner.req_uin,
                self.inner.group_code,
                self.inner.suspicious,
                self.inner.invitor_uin.is_some(),
                true,
                false,
                "".into(),
            )
            .await
    }

    pub async fn reject(&self, reason: String, block: bool) -> RQResult<()> {
        self.client
            .solve_group_system_message(
                self.inner.msg_seq,
                self.inner.req_uin,
                self.inner.group_code,
                self.inner.suspicious,
                self.inner.invitor_uin.is_some(),
                false,
                block,
                reason,
            )
            .await
    }
}

pub type NewFriendRequestEvent = EventWithClient<NewFriendRequest>;

impl NewFriendRequestEvent {
    pub async fn accept(&self) -> RQResult<()> {
        self.client
            .solve_friend_system_message(self.inner.msg_seq, self.inner.req_uin, true)
            .await
    }

    pub async fn reject(&self) -> RQResult<()> {
        self.client
            .solve_friend_system_message(self.inner.msg_seq, self.inner.req_uin, false)
            .await
    }
}

pub type NewMemberEvent = EventWithClient<NewMember>;
pub type GroupMuteEvent = EventWithClient<GroupMute>;
pub type FriendMessageRecallEvent = EventWithClient<FriendMessageRecall>;
pub type GroupMessageRecallEvent = EventWithClient<GroupMessageRecall>;
pub type NewFriendEvent = EventWithClient<FriendInfo>;
pub type GroupLeaveEvent = EventWithClient<GroupLeave>;
pub type GroupDisbandEvent = EventWithClient<GroupDisband>;
pub type FriendPokeEvent = EventWithClient<FriendPoke>;
pub type GroupPokeEvent = EventWithClient<GroupPoke>;
pub type GroupNameUpdateEvent = EventWithClient<GroupNameUpdate>;
pub type DeleteFriendEvent = EventWithClient<DeleteFriend>;
pub type MemberPermissionChangeEvent = EventWithClient<MemberPermissionChange>;
pub type SelfInvitedEvent = EventWithClient<SelfInvited>;
pub type GroupAudioMessageEvent = EventWithClient<GroupAudioMessage>;

impl GroupAudioMessageEvent {
    pub async fn url(&self) -> RQResult<String> {
        self.client
            .get_group_audio_url(self.inner.group_code, self.inner.audio.clone())
            .await
    }
}

pub type FriendAudioMessageEvent = EventWithClient<FriendAudioMessage>;

impl FriendAudioMessageEvent {
    pub async fn url(&self) -> RQResult<String> {
        self.client
            .get_friend_audio_url(self.inner.from_uin, self.inner.audio.clone())
            .await
    }
}

pub type KickedOfflineEvent = EventWithClient<jce::RequestPushForceOffline>;
pub type MSFOfflineEvent = EventWithClient<jce::RequestMSFForceOffline>;

#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum DisconnectReason {
    /// 主动断开
    Actively(NetworkStatus),
    /// 网络原因
    Network,
}

impl DisconnectReason {
    /// 客户端网络状态
    pub fn status(&self) -> NetworkStatus {
        match self {
            Self::Actively(s) => *s,
            Self::Network => NetworkStatus::NetworkOffline,
        }
    }
}

pub type ClientDisconnect = EventWithClient<DisconnectReason>;

impl ClientDisconnect {
    pub fn reason(&self) -> DisconnectReason {
        self.inner
    }
}
