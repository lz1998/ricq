pub enum ShareTarget {
    Friend(i64),
    Group(i64),
    Guild { guild_id: u64, channel_id: u64 },
}

impl ShareTarget {
    pub fn send_type(&self) -> u32 {
        match self {
            ShareTarget::Friend { .. } => 0,
            ShareTarget::Group { .. } => 1,
            ShareTarget::Guild { .. } => 3,
        }
    }
}
