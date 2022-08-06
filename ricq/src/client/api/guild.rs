use ricq_core::{RQResult, command::guild::GuildUserProfile};

impl super::super::Client {
    pub async fn get_guild_tiny_id(&self) -> RQResult<Option<u64>> {
        let engine = self.engine.read().await;
        let pkt = engine.build_sync_channel_first_view_packet();
        let rsp = self.send_and_wait(pkt).await?;

        let first_view = engine.decode_first_view_response(rsp.body)?;

        Ok(first_view.map(|f| f.self_tinyid))
    }

    pub async fn get_guild_user_profile(&self, tiny_id: u64) -> RQResult<Option<GuildUserProfile>> {
        let engine = self.engine.read().await;

        let pkt = engine.build_get_user_profile_packet(tiny_id);
        let rsp = self.send_and_wait(pkt).await?;
        engine.decode_user_profile(rsp.body)
    }
}