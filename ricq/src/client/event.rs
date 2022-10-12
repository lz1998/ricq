use std::sync::Arc;

use ricq_core::command::profile_service::{JoinGroupRequest, NewFriendRequest, SelfInvited};
use ricq_core::structs::{
    DeleteFriend, FriendAudioMessage, FriendInfo, FriendMessageRecall, FriendPoke,
    GroupAudioMessage, GroupDisband, GroupLeave, GroupMessageRecall, GroupMute, GroupNameUpdate,
    GroupTempMessage, MemberPermissionChange, NewMember,
};
use ricq_core::{jce, RQResult};

use crate::handler::RawHandler;
use crate::structs::{FriendMessage, GroupMessage};
use crate::Client;

#[derive(Clone, derivative::Derivative)]
#[derivative(Debug)]
pub struct EventWithClient<T, H: RawHandler> {
    #[derivative(Debug = "ignore")]
    pub client: Arc<Client<H>>,
    pub inner: T,
}

pub type GroupMessageEvent<H> = EventWithClient<GroupMessage, H>;

impl<H: RawHandler> GroupMessageEvent<H> {
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

pub type FriendMessageEvent<H> = EventWithClient<FriendMessage, H>;
pub type GroupTempMessageEvent<H> = EventWithClient<GroupTempMessage, H>;
pub type JoinGroupRequestEvent<H> = EventWithClient<JoinGroupRequest, H>;

impl<H: RawHandler> JoinGroupRequestEvent<H> {
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

pub type NewFriendRequestEvent<H> = EventWithClient<NewFriendRequest, H>;

impl<H: RawHandler> NewFriendRequestEvent<H> {
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

pub type NewMemberEvent<H> = EventWithClient<NewMember, H>;
pub type GroupMuteEvent<H> = EventWithClient<GroupMute, H>;
pub type FriendMessageRecallEvent<H> = EventWithClient<FriendMessageRecall, H>;
pub type GroupMessageRecallEvent<H> = EventWithClient<GroupMessageRecall, H>;
pub type NewFriendEvent<H> = EventWithClient<FriendInfo, H>;
pub type GroupLeaveEvent<H> = EventWithClient<GroupLeave, H>;
pub type GroupDisbandEvent<H> = EventWithClient<GroupDisband, H>;
pub type FriendPokeEvent<H> = EventWithClient<FriendPoke, H>;
pub type GroupNameUpdateEvent<H> = EventWithClient<GroupNameUpdate, H>;
pub type DeleteFriendEvent<H> = EventWithClient<DeleteFriend, H>;
pub type MemberPermissionChangeEvent<H> = EventWithClient<MemberPermissionChange, H>;
pub type SelfInvitedEvent<H> = EventWithClient<SelfInvited, H>;
pub type GroupAudioMessageEvent<H> = EventWithClient<GroupAudioMessage, H>;

impl<H: RawHandler> GroupAudioMessageEvent<H> {
    pub async fn url(&self) -> RQResult<String> {
        self.client
            .get_group_audio_url(self.inner.group_code, self.inner.audio.clone())
            .await
    }
}

pub type FriendAudioMessageEvent<H> = EventWithClient<FriendAudioMessage, H>;

impl<H: RawHandler> FriendAudioMessageEvent<H> {
    pub async fn url(&self) -> RQResult<String> {
        self.client
            .get_friend_audio_url(self.inner.from_uin, self.inner.audio.clone())
            .await
    }
}

pub type KickedOfflineEvent<H> = EventWithClient<jce::RequestPushForceOffline, H>;
pub type MSFOfflineEvent<H> = EventWithClient<jce::RequestMSFForceOffline, H>;
