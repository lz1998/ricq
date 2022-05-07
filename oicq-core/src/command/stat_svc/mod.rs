pub mod builder;
pub mod decoder;

#[derive(Debug, Clone)]
pub struct Status {
    pub online_status: i32,
    pub ext_online_status: i64,
    pub custom_status: Option<CustomOnlineStatus>,
}

#[derive(Debug, Copy, Clone)]
pub enum OnlineStatus {
    Online = 11,    // 在线
    Offline = 21,   // 离线
    Away = 31,      // 离开
    Invisible = 41, // 隐身
    Busy = 50,      // 忙
    Qme = 60,       // Q我吧
    Dnd = 70,       // 请勿打扰
}

impl From<OnlineStatus> for Status {
    fn from(s: OnlineStatus) -> Self {
        Self {
            online_status: s as i32,
            ext_online_status: 0,
            custom_status: None,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum ExtOnlineStatus {
    Battery = 1000,       // 当前电量
    Listening = 1028,     // 听歌中
    Constellation = 1040, // 星座运势
    Weather = 1030,       // 今日天气
    MeetSpring = 1069,    // 遇见春天
    Timi = 1027,          // Timi中
    EatChicken = 1064,    // 吃鸡中
    Loving = 1051,        // 恋爱中
    WangWang = 1053,      // 汪汪汪
    CookedRice = 1019,    // 干饭中
    Study = 1018,         // 学习中
    StayUp = 1032,        // 熬夜中
    PlayBall = 1050,      // 打球中
    Signal = 1011,        // 信号弱
    StudyOnline = 1024,   // 在线学习
    Gaming = 1017,        // 游戏中
    Vacationing = 1022,   // 度假中
    WatchingTV = 1021,    // 追剧中
    Fitness = 1020,       // 健身中
}

impl From<ExtOnlineStatus> for Status {
    fn from(s: ExtOnlineStatus) -> Self {
        Self {
            online_status: 11,
            ext_online_status: s as i64,
            custom_status: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CustomOnlineStatus {
    pub face_index: u64,
    pub wording: String,
}

impl From<CustomOnlineStatus> for Status {
    fn from(s: CustomOnlineStatus) -> Self {
        Self {
            online_status: 11,
            ext_online_status: 2000,
            custom_status: Some(s),
        }
    }
}
