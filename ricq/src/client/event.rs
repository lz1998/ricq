use ricq_core::command::profile_service::{JoinGroupRequest, NewFriendRequest, SelfInvited};
use ricq_core::jce::{RequestPushForceOffline, RequestMSFForceOffline};
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

pub type GroupMessageEvent = GroupMessage;
pub type FriendMessageEvent = FriendMessage;
pub type GroupTempMessageEvent = GroupTempMessage;
pub type JoinGroupRequestEvent = JoinGroupRequest;
pub type NewFriendRequestEvent = NewFriendRequest;
pub type NewMemberEvent = NewMember;
pub type GroupMuteEvent = GroupMute;
pub type FriendMessageRecallEvent = FriendMessageRecall;
pub type GroupMessageRecallEvent = GroupMessageRecall;
pub type NewFriendEvent = FriendInfo;
pub type GroupLeaveEvent = GroupLeave;
pub type GroupDisbandEvent = GroupDisband;
pub type FriendPokeEvent = FriendPoke;
pub type GroupNameUpdateEvent = GroupNameUpdate;
pub type DeleteFriendEvent = DeleteFriend;
pub type MemberPermissionChangeEvent = MemberPermissionChange;
pub type SelfInvitedEvent = SelfInvited;
pub type GroupAudioMessageEvent = GroupAudioMessage;
pub type FriendAudioMessageEvent = FriendAudioMessage;
pub type KickedOfflineEvent = RequestPushForceOffline;
pub type MSFOfflineEvent = RequestMSFForceOffline;
