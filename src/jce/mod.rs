use bytes::Bytes;
use jce_struct::*;
use std::collections::HashMap;

mod test;

JceStruct!(RequestPacket {
    1  => i_version: i16,
    2  => c_packet_type: u8,
    3  => i_message_type: i32,
    4  => i_request_id: i32,
    5  => s_servant_name: String,
    6  => s_func_name: String,
    7  => s_buffer: Bytes,
    8  => i_timeout: i32,
    9  => context: HashMap<String,String>,
    10 => status: HashMap<String,String>,
});

JceStruct!(RequestDataVersion3 {
    0 => map: HashMap<String,Bytes>,
});

// Recursive expansion of JceStruct! macro
// ========================================

// pub struct RequestDataVersion3 {
//     pub map: HashMap<String, Bytes>,
// }
// impl JcePut for RequestDataVersion3 {
//     fn put(self, jce_mut: &mut JceMut, tag: u8) -> &mut JceMut {
//         jce_mut.put_head(10, tag);
//         self.put_raw(jce_mut);
//         jce_mut.put_head(11, 0)
//     }
//     fn put_raw(self, jce_mut: &mut JceMut) -> &mut JceMut {
//         self.map.put(jce_mut, 0);
//         jce_mut
//     }
// }
// impl JceGet for RequestDataVersion3 {
//     fn get(jce: &mut Jce) -> Self {
//         let map = jce.get_by_tag(0);
//         jce.end_object();
//         RequestDataVersion3 { map }
//     }
//     fn empty() -> Self {
//         {
//             panic!("jce get empty, should have a object")
//         }
//     }
// }

JceStruct!(RequestDataVersion2 {
    0 => map: HashMap<String,Bytes>,
});

// Recursive expansion of JceStruct! macro
// ========================================

// pub struct RequestDataVersion2 {
//     pub map: HashMap<String, HashMap<String, Bytes>>,
// }
// impl JcePut for RequestDataVersion2 {
//     fn put(self, jce_mut: &mut JceMut, tag: u8) -> &mut JceMut {
//         jce_mut.put_head(10, tag);
//         self.put_raw(jce_mut);
//         jce_mut.put_head(11, 0)
//     }
//     fn put_raw(self, jce_mut: &mut JceMut) -> &mut JceMut {
//         self.map.put(jce_mut, 0);
//         jce_mut
//     }
// }
// impl JceGet for RequestDataVersion2 {
//     fn get(jce: &mut Jce) -> Self {
//         let map = jce.get_by_tag(0);
//         jce.end_object();
//         RequestDataVersion2 { map }
//     }
//     fn empty() -> Self {
//         {
//             panic!("jce get empty, should have a object")
//         }
//     }
// }

JceStruct!(HttpServerListRes {
    2 => sso_server_infos: Vec<SsoServerInfo>,
});

JceStruct!(SsoServerInfo {
    1 => server: String,
    2 => port: i32,
    8 => location: String,
});

JceStruct!(FileStoragePushFSSvcList {
    0  => upload_list: Vec<FileStorageServerInfo>,
    1  => pic_download_list: Vec<FileStorageServerInfo>,
    2  => g_pic_download_list: Vec<FileStorageServerInfo>,
    3  => q_zone_proxy_service_list: Vec<FileStorageServerInfo>,
    4  => url_encode_service_list: Vec<FileStorageServerInfo>,
    5  => big_data_channel: BigDataChannel,
    6  => vip_emotion_list: Vec<FileStorageServerInfo>,
    7  => c2c_pic_down_list: Vec<FileStorageServerInfo>,
    // 8  => fmt_ip_info: FmtIpInfo,
    // 9  => domain_ip_channel : DomainIpChannel,
    10 => ptt_list: Bytes,
});

JceStruct!(FileStorageServerInfo {
    1 => server: String,
    2 => port: i32,
});

JceStruct!(BigDataChannel {
    0 => ip_list: Vec<BigDataIp>, // gocq BigDataIPList
    1 => sig_session: Bytes,
    2 => key_session: Bytes,
    3 => sig_uin: i64,
    4 => connect_flag: i32,
    5 => pb_buf: Bytes,
});

JceStruct!(BigDataIp {
    0 => service_type: i64,
    1 => ip_list: BigDataIpInfo,
    2 => fragment_size: i64,
});

JceStruct!(BigDataIpInfo {
    0 => type_: i64,
    1 => server: String,
    2 => port: i64,
});

JceStruct!(SvcReqRegister {
        0  => uin: i64,
        1  => bid: i64,
        2  => conn_type: u8,
        3  => other: String,
        4  => status: i32,
        5  => online_push: u8,
        6  => is_online: u8,
        7  => is_show_online: u8,
        8  => kick_pc: u8,
        9  => kick_weak: u8,
        10 => timestamp : i64,
        11 => ios_version: i64,
        12 => net_type: u8,
        13 => build_ver: String,
        14 => reg_type: u8,
        15 => dev_param: Bytes ,
        16 => guid: Bytes,
        17 => locale_id: i32,
        18 => silent_push: u8,
        19 => dev_name: String,
        20 => dev_type: String,
        21 => os_ver: String,
        22 => open_push : u8,
        23 => large_seq: i64,
        24 => last_watch_start_time: i64,
        26 => old_sso_ip: i64,
        27 => new_sso_ip: i64,
        28 => channel_no: String,
        29 => cpid: i64,
        30 => vendor_name: String,
        31 => vendor_os_name: String,
        32 => ios_idfa: String,
        33 => b769: Bytes,
        34 => is_set_status: u8,
        35 => server_buf: Bytes,
        36 => set_mute: u8,
        38 => ext_online_status: i64,
        39 => battery_status: i32,
});

JceStruct!(SvcRespRegister {
    0  => uin: i64,
    1  => bid: i64,
    2  => reply_code: u8,
    3  => result: String,
    4  => server_time: i64,
    5  => log_qq: u8,
    6  => need_kik: u8,
    7  => update_flag: u8,
    8  => timestamp: i64,
    9  => crash_flag: u8,
    10 => client_ip: String,
    11 => client_port: i32,
    12 => hello_interval: i32,
    13 => large_seq: i32,
    14 => large_seq_update: u8,
    15 => d769_rsp_body: Bytes,
    16 => status: i32,
    17 => ext_online_status: i64,
    18 => client_battery_get_interval: i64,
    19 => client_auto_status_interval: i64,
});

// JceStruct!(SvcReqRegisterNew {
//     0  => request_optional: i64,
//     1  => c2c_msg:
// });

// 下面是生成的
//
// JceStruct!(RequestPacket {
// 		1 => IVersion : i16
// 		2 => CPacketType : u8
// 		3 => IMessageType : i32
// 		4 => IRequestId : i32
// 		5 => SServantName : String
// 		6 => SFuncName : String
// 		7 => SBuffer : Bytes
// 		8 => ITimeout : i32
// 		9 => Context : map[String]String
// 		10 => Status : map[String]String
// 	});
//
// JceStruct!(RequestDataVersion3 {
// 		0 => Map : map[String]Bytes
// 	});
//
// JceStruct!(RequestDataVersion2 {
// 		0 => Map : map[String]map[String]Bytes
// 	});
//
// JceStruct!(SsoServerInfo {
// 		1 => Server : String
// 		2 => Port : i32
// 		8 => Location : String
// 	});
//
// JceStruct!(FileStoragePushFSSvcList {
// 		0 => UploadList : []FileStorageServerInfo
// 		1 => PicDownloadList : []FileStorageServerInfo
// 		2 => GPicDownloadList : []FileStorageServerInfo
// 		3 => QZoneProxyServiceList : []FileStorageServerInfo
// 		4 => UrlEncodeServiceList : []FileStorageServerInfo
// 		5 => BigDataChannel : BigDataChannel
// 		6 => VipEmotionList : []FileStorageServerInfo
// 		7 => C2CPicDownList : []FileStorageServerInfo
// 		10 => PttList : Bytes
// 	});
//
// JceStruct!(FileStorageServerInfo {
// 		1 => Server : String
// 		2 => Port : i32
// 	});
//
// JceStruct!(BigDataChannel {
// 		0 => IPLists : []BigDataIPList
// 		1 => SigSession : Bytes
// 		2 => KeySession : Bytes
// 		3 => SigUin : i64
// 		4 => ConnectFlag : i32
// 		5 => PbBuf : Bytes
// 	});
//
// JceStruct!(BigDataIPList {
// 		0 => ServiceType : i64
// 		1 => IPList : []BigDataIPInfo
// 		3 => FragmentSize : i64
// 	});
//
// JceStruct!(BigDataIPInfo {
// 		0 => Type : i64
// 		1 => Server : String
// 		2 => Port : i64
// 	});
//
// JceStruct!(SvcReqRegister {
//
// 		0 => Uin : i64
// 		1 => Bid : i64
// 		2 => ConnType : u8
// 		3 => Other : String
// 		4 => Status : i32
// 		5 => OnlinePush : u8
// 		6 => IsOnline : u8
// 		7 => IsShowOnline : u8
// 		8 => KickPC : u8
// 		9 => KickWeak : u8
// 		10 => Timestamp : i64
// 		11 => IOSVersion : i64
// 		12 => NetType : u8
// 		13 => BuildVer : String
// 		14 => RegType : u8
// 		15 => DevParam : Bytes
// 		16 => Guid : Bytes
// 		17 => LocaleId : i32
// 		18 => SilentPush : u8
// 		19 => DevName : String
// 		20 => DevType : String
// 		21 => OSVer : String
// 		22 => OpenPush : u8
// 		23 => LargeSeq : i64
// 		24 => LastWatchStartTime : i64
// 		26 => OldSSOIp : i64
// 		27 => NewSSOIp : i64
// 		28 => ChannelNo : String
// 		29 => CPID : i64
// 		30 => VendorName : String
// 		31 => VendorOSName : String
// 		32 => IOSIdfa : String
// 		33 => B769 : Bytes
// 		34 => IsSetStatus : u8
// 		35 => ServerBuf : Bytes
// 		36 => SetMute : u8
// 		38 => ExtOnlineStatus : i64
// 		39 => BatteryStatus : i32
// 	});
//
// JceStruct!(SvcRespRegister {
// 		0 => Uin : i64
// 		1 => Bid : i64
// 		2 => ReplyCode : u8
// 		3 => Result : String
// 		4 => ServerTime : i64
// 		5 => LogQQ : u8
// 		6 => NeedKik : u8
// 		7 => UpdateFlag : u8
// 		8 => Timestamp : i64
// 		9 => CrashFlag : u8
// 		10 => ClientIp : String
// 		11 => ClientPort : i32
// 		12 => HelloInterval : i32
// 		13 => LargeSeq : i32
// 		14 => LargeSeqUpdate : u8
// 		15 => D769RspBody : Bytes
// 		16 => Status : i32
// 		17 => ExtOnlineStatus : i64
// 		18 => ClientBatteryGetInterval : i64
// 		19 => ClientAutoStatusInterval : i64
// 	});
//
// JceStruct!(SvcReqRegisterNew {
//
// 		0 => RequestOptional : i64
// 		1 => C2CMsg :  // SvcReqGetMsgV2
// 		2 => GroupMsg :  // SvcReqPullGroupMsgSeq
// 		14 => DisGroupMsgFilter : u8
// 		15 => GroupMask : u8
// 		16 => EndSeq : i64
// 		20 => O769Body : Bytes
// 	});
//
// JceStruct!(SvcReqGetMsgV2 {
//
// 		0 => Uin : i64
// 		1 => DateTime : i32
// 		4 => RecivePic : u8
// 		6 => Ability : i16
// 		9 => Channel : u8
// 		16 => Inst : u8
// 		17 => ChannelEx : u8
// 		18 => SyncCookie : Bytes
// 		19 => SyncFlag : int
// 		20 => RambleFlag : u8
// 		26 => GeneralAbi : i64
// 		27 => PubAccountCookie : Bytes
// 	});
//
// JceStruct!(SvcReqPullGroupMsgSeq {
//
// 		0 => GroupInfo : [] // PullGroupSeqParam
// 		1 => VerifyType : u8
// 		2 => Filter : i32
// 	});
//
// JceStruct!(PullGroupSeqParam {
//
// 		0 => GroupCode : i64
// 		1 => LastSeqId : i64
// 	});
//
// JceStruct!(SvcRespParam {
// 		0 => PCStat : i32
// 		1 => IsSupportC2CRoamMsg : i32
// 		2 => IsSupportDataLine : i32
// 		3 => IsSupportPrintable : i32
// 		4 => IsSupportViewPCFile : i32
// 		5 => PcVersion : i32
// 		6 => RoamFlag : i64
// 		7 => OnlineInfos : []OnlineInfo
// 		8 => PCClientType : i32
// 	});
//
// JceStruct!(RequestPushNotify {
// 		0 => Uin : i64
// 		1 => Type : u8
// 		2 => Service : String
// 		3 => Cmd : String
// 		4 => NotifyCookie : Bytes
// 		5 => MsgType : i32
// 		6 => UserActive : i32
// 		7 => GeneralFlag : i32
// 		8 => BindedUin : i64
// 	});
//
// JceStruct!(OnlineInfo {
// 		0 => InstanceId : i32
// 		1 => ClientType : i32
// 		2 => OnlineStatus : i32
// 		3 => PlatformId : i32
// 		4 => SubPlatform : String
// 		5 => UClientType : i64
// 	});
//
// JceStruct!(SvcReqMSFLoginNotify {
// 		0 => AppId : i64
// 		1 => Status : u8
// 		2 => Tablet : u8
// 		3 => Platform : i64
// 		4 => Title : String
// 		5 => Info : String
// 		6 => ProductType : i64
// 		7 => ClientType : i64
// 		8 => InstanceList : []InstanceInfo
// 	});
//
// JceStruct!(InstanceInfo {
// 		0 => AppId : i32
// 		1 => Tablet : u8
// 		2 => Platform : i64
// 		3 => ProductType : i64
// 		4 => ClientType : i64
// 	});
//
// JceStruct!(PushMessageInfo {
// 		0 => FromUin : i64
// 		1 => MsgTime : i64
// 		2 => MsgType : i16
// 		3 => MsgSeq : i16
// 		4 => Msg : String
// 		5 => RealMsgTime : i32
// 		6 => VMsg : Bytes
// 		7 => AppShareID : i64
// 		8 => MsgCookies : Bytes
// 		9 => AppShareCookie : Bytes
// 		10 => MsgUid : i64
// 		11 => LastChangeTime : i64
// 		14 => FromInstId : i64
// 		15 => RemarkOfSender : Bytes
// 		16 => FromMobile : String
// 		17 => FromName : String
// 	});
//
// JceStruct!(SvcRespPushMsg {
//
// 		0 => Uin : i64
// 		1 => DelInfos : []
// 		2 => Svrip : i32
// 		3 => PushToken : Bytes
// 		4 => ServiceType : i32
// 	});
//
// JceStruct!(SvcReqGetDevLoginInfo {
//
// 		0 => Guid : Bytes
// 		1 => AppName : String
// 		2 => LoginType : i64
// 		3 => Timestamp : i64
// 		4 => NextItemIndex : i64
// 		5 => RequireMax : i64
// 		6 => GetDevListType : i64 // 1: getLoginDevList 2: getRecentLoginDevList 4: getAuthLoginDevList
// 	});
//
// JceStruct!(SvcDevLoginInfo {
// 		AppId          i64
// 		Guid           Bytes
// 		LoginTime      i64
// 		LoginPlatform  i64
// 		LoginLocation  String
// 		DeviceName     String
// 		DeviceTypeInfo String
// 		TerType        i64
// 		ProductType    i64
// 		CanBeKicked    i64
// 	});
//
// JceStruct!(DelMsgInfo {
//
// 		0 => FromUin : i64
// 		1 => MsgTime : i64
// 		2 => MsgSeq : i16
// 		3 => MsgCookies : Bytes
// 		4 => Cmd : i16
// 		5 => MsgType : i64
// 		6 => AppId : i64
// 		7 => SendTime : i64
// 		8 => SsoSeq : i32
// 		9 => SsoIp : i32
// 		10 => ClientIp : i32
// 	});
//
JceStruct!(FriendListRequest {
		0 => reqtype : i32,
		1 => if_reflush : u8,
		2 => uin : i64,
		3 => start_index : i16,
		4 => friend_count : i16,
		5 => group_id : u8,
		6 => if_get_group_info : u8,
		7 => group_start_index : u8,
		8 => group_count : u8,
		9 => if_get_msf_group : u8,
		10 => if_show_term_type : u8,
		11 => version : i64,
		12 => uin_list : Vec<i64>,
		13 => app_type : i32,
		14 => if_get_dov_id : u8,
		15 => if_get_both_flag : u8,
		16 => d50 : Bytes,
		17 => d6b : Bytes,
		18 => sns_type_list : Vec<i64>,
});
//
// JceStruct!(FriendInfo {
// 		0 => FriendUin : i64
// 		1 => GroupId : u8
// 		2 => FaceId : i16
// 		3 => Remark : String
// 		4 => QQType : u8
// 		5 => Status : u8
// 		6 => MemberLevel : u8
// 		7 => IsMqqOnLine : u8
// 		8 => QQOnlineState : u8
// 		9 => IsIphoneOnline : u8
// 		10 => DetailStatusFlag : u8
// 		11 => QQOnlineStateV2 : u8
// 		12 => ShowName : String
// 		13 => IsRemark : u8
// 		14 => Nick : String
// 		15 => SpecialFlag : u8
// 		16 => IMGroupID : Bytes
// 		17 => MSFGroupID : Bytes
// 		18 => TermType : i32
// 		20 => Network : u8
// 		21 => Ring : Bytes
// 		22 => AbiFlag : i64
// 		23 => FaceAddonId : i64
// 		24 => NetworkType : i32
// 		25 => VipFont : i64
// 		26 => IconType : i32
// 		27 => TermDesc : String
// 		28 => ColorRing : i64
// 		29 => ApolloFlag : u8
// 		30 => ApolloTimestamp : i64
// 		31 => Sex : u8
// 		32 => FounderFont : i64
// 		33 => EimId : String
// 		34 => EimMobile : String
// 		35 => OlympicTorch : u8
// 		36 => ApolloSignTime : i64
// 		37 => LaviUin : i64
// 		38 => TagUpdateTime : i64
// 		39 => GameLastLoginTime : i64
// 		40 => GameAppId : i64
// 		41 => CardID : Bytes
// 		42 => BitSet : i64
// 		43 => KingOfGloryFlag : u8
// 		44 => KingOfGloryRank : i64
// 		45 => MasterUin : String
// 		46 => LastMedalUpdateTime : i64
// 		47 => FaceStoreId : i64
// 		48 => FontEffect : i64
// 		49 => DOVId : String
// 		50 => BothFlag : i64
// 		51 => CentiShow3DFlag : u8
// 		52 => IntimateInfo : Bytes
// 		53 => ShowNameplate : u8
// 		54 => NewLoverDiamondFlag : u8
// 		55 => ExtSnsFrdData : Bytes
// 		56 => MutualMarkData : Bytes
// 	});
//
// JceStruct!(TroopListRequest {
//
// 		0 => Uin : i64
// 		1 => GetMSFMsgFlag : u8
// 		2 => Cookies : Bytes
// 		3 => GroupInfo : []i64
// 		4 => GroupFlagExt : u8
// 		5 => Version : i32
// 		6 => CompanyId : i64
// 		7 => VersionNum : i64
// 		8 => GetLongGroupName : u8
// 	});
//
// JceStruct!(TroopNumber {
// 		0 => GroupUin : i64
// 		1 => GroupCode : i64
// 		2 => Flag : u8
// 		3 => GroupInfoSeq : i64
// 		4 => GroupName : String
// 		5 => GroupMemo : String
// 		6 => GroupFlagExt : i64
// 		7 => GroupRankSeq : i64
// 		8 => CertificationType : i64
// 		9 => ShutUpTimestamp : i64
// 		10 => MyShutUpTimestamp : i64
// 		11 => CmdUinUinFlag : i64
// 		12 => AdditionalFlag : i64
// 		13 => GroupTypeFlag : i64
// 		14 => GroupSecType : i64
// 		15 => GroupSecTypeInfo : i64
// 		16 => GroupClassExt : i64
// 		17 => AppPrivilegeFlag : i64
// 		18 => SubscriptionUin : i64
// 		19 => MemberNum : i64
// 		20 => MemberNumSeq : i64
// 		21 => MemberCardSeq : i64
// 		22 => GroupFlagExt3 : i64
// 		23 => GroupOwnerUin : i64
// 		24 => IsConfGroup : u8
// 		25 => IsModifyConfGroupFace : u8
// 		26 => IsModifyConfGroupName : u8
// 		27 => CmdUinJoinTime : i64
// 		28 => CompanyId : i64
// 		29 => MaxGroupMemberNum : i64
// 		30 => CmdUinGroupMask : i64
// 		31 => GuildAppId : i64
// 		32 => GuildSubType : i64
// 		33 => CmdUinRingtoneID : i64
// 		34 => CmdUinFlagEx2 : i64
// 	});
//
// JceStruct!(TroopMemberListRequest {
//
// 		0 => Uin : i64
// 		1 => GroupCode : i64
// 		2 => NextUin : i64
// 		3 => GroupUin : i64
// 		4 => Version : i64
// 		5 => ReqType : i64
// 		6 => GetListAppointTime : i64
// 		7 => RichCardNameVer : u8
// 	});
//
// JceStruct!(TroopMemberInfo {
// 		0 => MemberUin : i64
// 		1 => FaceId : i16
// 		2 => Age : u8
// 		3 => Gender : u8
// 		4 => Nick : String
// 		5 => Status : u8
// 		6 => ShowName : String
// 		8 => Name : String
// 		12 => Memo : String
// 		13 => AutoRemark : String
// 		14 => MemberLevel : i64
// 		15 => JoinTime : i64
// 		16 => LastSpeakTime : i64
// 		17 => CreditLevel : i64
// 		18 => Flag : i64
// 		19 => FlagExt : i64
// 		20 => Point : i64
// 		21 => Concerned : u8
// 		22 => Shielded : u8
// 		23 => SpecialTitle : String
// 		24 => SpecialTitleExpireTime : i64
// 		25 => Job : String
// 		26 => ApolloFlag : u8
// 		27 => ApolloTimestamp : i64
// 		28 => GlobalGroupLevel : i64
// 		29 => TitleId : i64
// 		30 => ShutUpTimestap : i64
// 		31 => GlobalGroupPoint : i64
// 		33 => RichCardNameVer : u8
// 		34 => VipType : i64
// 		35 => VipLevel : i64
// 		36 => BigClubLevel : i64
// 		37 => BigClubFlag : i64
// 		38 => Nameplate : i64
// 		39 => GroupHonor : Bytes
// 	});
//
// JceStruct!(ModifyGroupCardRequest {
//
// 		0 => Zero : i64
// 		1 => GroupCode : i64
// 		2 => NewSeq : i64
// 		3 => UinInfo : []
// 	});
//
// JceStruct!(UinInfo {
//
// 		0 => Uin : i64
// 		1 => Flag : i64
// 		2 => Name : String
// 		3 => Gender : u8
// 		4 => Phone : String
// 		5 => Email : String
// 		6 => Remark : String
// 	});
//
// JceStruct!(SummaryCardReq {
//
// 		0 => Uin : i64
// 		1 => ComeFrom : i32
// 		2 => QzoneFeedTimestamp : i64
// 		3 => IsFriend : u8
// 		4 => GroupCode : i64
// 		5 => GroupUin : i64
// 		8 => GetControl : i64
// 		9 => AddFriendSource : i32
// 		10 => SecureSig : Bytes
// 		14 => ReqServices : []Bytes
// 		15 => TinyId : i64
// 		16 => LikeSource : i64
// 		18 => ReqMedalWallInfo : u8
// 		19 => Req0x5ebFieldId : []i64
// 		20 => ReqNearbyGodInfo : u8
// 		22 => ReqExtendCard : u8
// 	});
//
// JceStruct!(SummaryCardReqSearch {
//
// 		0 => Keyword : String
// 		1 => CountryCode : String
// 		2 => Version : i32
// 		3 => ReqServices : []Bytes // busi
// 	});
//
// JceStruct!(DelFriendReq {
//
// 		0 => Uin : i64
// 		1 => DelUin : i64
// 		2 => DelType : u8
// 		3 => Version : i32
// 	});