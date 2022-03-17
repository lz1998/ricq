use std::sync::Arc;

use crate::engine::command::profile_service::{JoinGroupRequest, NewFriendRequest, SelfInvited};
use crate::engine::structs::{
    DeleteFriend, FriendAudioMessage, FriendInfo, FriendMessageRecall, FriendPoke,
    GroupAudioMessage, GroupLeave, GroupMessageRecall, GroupMute, GroupNameUpdate,
    MemberPermissionChange, NewMember, TempMessage,
};
use crate::engine::{jce, RQResult};

use crate::structs::{FriendMessage, Group, GroupMemberInfo, GroupMessage};
use crate::Client;

#[derive(Clone, derivative::Derivative)]
#[derivative(Debug)]
pub struct GroupMessageEvent {
    #[derivative(Debug = "ignore")]
    pub client: Arc<Client>,
    pub message: GroupMessage,
}

impl GroupMessageEvent {
    pub async fn group(&self) -> Option<Arc<Group>> {
        self.client.find_group(self.message.group_code, true).await
    }

    pub async fn member(&self) -> Option<GroupMemberInfo> {
        let group = self.group().await?;
        let members = group.members.read().await;
        members
            .iter()
            .filter(|m| m.uin == self.message.from_uin)
            .last()
            .cloned()
    }

    pub async fn recall(&self) -> RQResult<()> {
        // TODO check permission
        self.client
            .recall_group_message(
                self.message.group_code,
                self.message.seqs.clone(),
                self.message.rands.clone(),
            )
            .await
    }
}

#[derive(Clone, derivative::Derivative)]
#[derivative(Debug)]
pub struct FriendMessageEvent {
    #[derivative(Debug = "ignore")]
    pub client: Arc<Client>,
    pub message: FriendMessage,
}

impl FriendMessageEvent {
    pub async fn friend(&self) -> Option<Arc<FriendInfo>> {
        self.client.find_friend(self.message.from_uin).await
    }
}

#[derive(Clone, derivative::Derivative)]
#[derivative(Debug)]
pub struct TempMessageEvent {
    #[derivative(Debug = "ignore")]
    pub client: Arc<Client>,
    pub message: TempMessage,
}

#[derive(Clone, derivative::Derivative)]
#[derivative(Debug)]
pub struct GroupRequestEvent {
    #[derivative(Debug = "ignore")]
    pub client: Arc<Client>,
    pub request: JoinGroupRequest,
}

impl GroupRequestEvent {
    pub async fn accept(&self) -> RQResult<()> {
        self.client
            .solve_group_system_message(
                self.request.msg_seq,
                self.request.req_uin,
                self.request.group_code,
                self.request.suspicious,
                self.request.invitor_uin.is_some(),
                true,
                false,
                "".into(),
            )
            .await
    }

    pub async fn reject(&self, reason: String, block: bool) -> RQResult<()> {
        self.client
            .solve_group_system_message(
                self.request.msg_seq,
                self.request.req_uin,
                self.request.group_code,
                self.request.suspicious,
                self.request.invitor_uin.is_some(),
                false,
                block,
                reason,
            )
            .await
    }
}

#[derive(Clone, derivative::Derivative)]
#[derivative(Debug)]
pub struct FriendRequestEvent {
    #[derivative(Debug = "ignore")]
    pub client: Arc<Client>,
    pub request: NewFriendRequest,
}

impl FriendRequestEvent {
    pub async fn accept(&self) -> RQResult<()> {
        self.client
            .solve_friend_system_message(self.request.msg_seq, self.request.req_uin, true)
            .await
    }

    pub async fn reject(&self) -> RQResult<()> {
        self.client
            .solve_friend_system_message(self.request.msg_seq, self.request.req_uin, false)
            .await
    }
}

#[derive(Clone, derivative::Derivative)]
#[derivative(Debug)]
pub struct NewMemberEvent {
    #[derivative(Debug = "ignore")]
    pub client: Arc<Client>,
    pub new_member: NewMember,
}

impl NewMemberEvent {
    pub async fn group(&self) -> Option<Arc<Group>> {
        self.client
            .find_group(self.new_member.group_code, true)
            .await
    }

    pub async fn member(&self) -> Option<GroupMemberInfo> {
        let group = self.group().await?;
        let members = group.members.read().await;
        members
            .iter()
            .filter(|m| m.uin == self.new_member.member_uin)
            .last()
            .cloned()
    }
}

#[derive(Clone, derivative::Derivative)]
#[derivative(Debug)]
pub struct GroupMuteEvent {
    #[derivative(Debug = "ignore")]
    pub client: Arc<Client>,
    pub group_mute: GroupMute,
}

#[derive(Clone, derivative::Derivative)]
#[derivative(Debug)]
pub struct FriendMessageRecallEvent {
    #[derivative(Debug = "ignore")]
    pub client: Arc<Client>,
    pub recall: FriendMessageRecall,
}

#[derive(Clone, derivative::Derivative)]
#[derivative(Debug)]
pub struct GroupMessageRecallEvent {
    #[derivative(Debug = "ignore")]
    pub client: Arc<Client>,
    pub recall: GroupMessageRecall,
}

#[derive(Clone, derivative::Derivative)]
#[derivative(Debug)]
pub struct NewFriendEvent {
    #[derivative(Debug = "ignore")]
    pub client: Arc<Client>,
    pub friend: FriendInfo,
}

#[derive(Clone, derivative::Derivative)]
#[derivative(Debug)]
pub struct GroupLeaveEvent {
    #[derivative(Debug = "ignore")]
    pub client: Arc<Client>,
    pub leave: GroupLeave,
}

#[derive(Clone, derivative::Derivative)]
#[derivative(Debug)]
pub struct FriendPokeEvent {
    #[derivative(Debug = "ignore")]
    pub client: Arc<Client>,
    pub poke: FriendPoke,
}

#[derive(Clone, derivative::Derivative)]
#[derivative(Debug)]
pub struct GroupNameUpdateEvent {
    #[derivative(Debug = "ignore")]
    pub client: Arc<Client>,
    pub update: GroupNameUpdate,
}

#[derive(Clone, derivative::Derivative)]
#[derivative(Debug)]
pub struct DeleteFriendEvent {
    #[derivative(Debug = "ignore")]
    pub client: Arc<Client>,
    pub delete: DeleteFriend,
}

#[derive(Clone, derivative::Derivative)]
#[derivative(Debug)]
pub struct MemberPermissionChangeEvent {
    #[derivative(Debug = "ignore")]
    pub client: Arc<Client>,
    pub change: MemberPermissionChange,
}

#[derive(Clone, derivative::Derivative)]
#[derivative(Debug)]
pub struct SelfInvitedEvent {
    #[derivative(Debug = "ignore")]
    pub client: Arc<Client>,
    pub request: SelfInvited,
}

#[derive(Clone, derivative::Derivative)]
#[derivative(Debug)]
pub struct GroupAudioMessageEvent {
    #[derivative(Debug = "ignore")]
    pub client: Arc<Client>,
    pub message: GroupAudioMessage,
}

impl GroupAudioMessageEvent {
    pub async fn url(&self) -> RQResult<String> {
        self.client
            .get_group_audio_url(self.message.group_code, self.message.audio.clone())
            .await
    }
}

#[derive(Clone, derivative::Derivative)]
#[derivative(Debug)]
pub struct FriendAudioMessageEvent {
    #[derivative(Debug = "ignore")]
    pub client: Arc<Client>,
    pub message: FriendAudioMessage,
}

impl FriendAudioMessageEvent {
    pub async fn url(&self) -> RQResult<String> {
        self.client
            .get_friend_audio_url(self.message.from_uin, self.message.audio.clone())
            .await
    }
}

#[derive(Clone, derivative::Derivative)]
#[derivative(Debug)]
pub struct KickedOfflineEvent {
    #[derivative(Debug = "ignore")]
    pub client: Arc<Client>,
    pub offline: jce::RequestPushForceOffline,
}

#[derive(Clone, derivative::Derivative)]
#[derivative(Debug)]
pub struct MSFOfflineEvent {
    #[derivative(Debug = "ignore")]
    pub client: Arc<Client>,
    pub offline: jce::RequestMSFForceOffline,
}
