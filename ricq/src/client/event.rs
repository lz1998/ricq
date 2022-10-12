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
#[repr(transparent)]
pub struct Event<T>(pub T);

pub type GroupMessageEvent = Event<GroupMessage>;

impl GroupMessageEvent {
    pub async fn recall<H: RawHandler>(&self, client: Client<H>) -> RQResult<()> {
        // TODO check permission
        client
            .recall_group_message(self.0.group_code, self.0.seqs.clone(), self.0.rands.clone())
            .await
    }
}

pub type FriendMessageEvent = Event<FriendMessage>;
pub type GroupTempMessageEvent = Event<GroupTempMessage>;
pub type JoinGroupRequestEvent = Event<JoinGroupRequest>;

impl JoinGroupRequestEvent {
    pub async fn accept<H: RawHandler>(&self, client: Client<H>) -> RQResult<()> {
        client
            .solve_group_system_message(
                self.0.msg_seq,
                self.0.req_uin,
                self.0.group_code,
                self.0.suspicious,
                self.0.invitor_uin.is_some(),
                true,
                false,
                "".into(),
            )
            .await
    }

    pub async fn reject<H: RawHandler>(
        &self,
        client: Client<H>,
        reason: String,
        block: bool,
    ) -> RQResult<()> {
        client
            .solve_group_system_message(
                self.0.msg_seq,
                self.0.req_uin,
                self.0.group_code,
                self.0.suspicious,
                self.0.invitor_uin.is_some(),
                false,
                block,
                reason,
            )
            .await
    }
}

pub type NewFriendRequestEvent = Event<NewFriendRequest>;

impl NewFriendRequestEvent {
    pub async fn accept<H: RawHandler>(&self, client: Client<H>) -> RQResult<()> {
        client
            .solve_friend_system_message(self.0.msg_seq, self.0.req_uin, true)
            .await
    }

    pub async fn reject<H: RawHandler>(&self, client: Client<H>) -> RQResult<()> {
        client
            .solve_friend_system_message(self.0.msg_seq, self.0.req_uin, false)
            .await
    }
}

pub type NewMemberEvent = Event<NewMember>;
pub type GroupMuteEvent = Event<GroupMute>;
pub type FriendMessageRecallEvent = Event<FriendMessageRecall>;
pub type GroupMessageRecallEvent = Event<GroupMessageRecall>;
pub type NewFriendEvent = Event<FriendInfo>;
pub type GroupLeaveEvent = Event<GroupLeave>;
pub type GroupDisbandEvent = Event<GroupDisband>;
pub type FriendPokeEvent = Event<FriendPoke>;
pub type GroupNameUpdateEvent = Event<GroupNameUpdate>;
pub type DeleteFriendEvent = Event<DeleteFriend>;
pub type MemberPermissionChangeEvent = Event<MemberPermissionChange>;
pub type SelfInvitedEvent = Event<SelfInvited>;
pub type GroupAudioMessageEvent = Event<GroupAudioMessage>;

impl GroupAudioMessageEvent {
    pub async fn url<H: RawHandler>(&self, client: Client<H>) -> RQResult<String> {
        client
            .get_group_audio_url(self.0.group_code, self.0.audio.clone())
            .await
    }
}

pub type FriendAudioMessageEvent = Event<FriendAudioMessage>;

impl FriendAudioMessageEvent {
    pub async fn url<H: RawHandler>(&self, client: Client<H>) -> RQResult<String> {
        client
            .get_friend_audio_url(self.0.from_uin, self.0.audio.clone())
            .await
    }
}

pub type KickedOfflineEvent = Event<jce::RequestPushForceOffline>;
pub type MSFOfflineEvent = Event<jce::RequestMSFForceOffline>;
