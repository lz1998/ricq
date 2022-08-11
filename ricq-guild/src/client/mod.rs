use ricq_core::protocol::packet::Packet;
use std::collections::HashMap;
use std::ops::Deref;
use std::sync::Arc;
use tokio::sync::{broadcast, RwLockReadGuard};
use ricq_core::RQResult;
use crate::client::decoder::Decoder;
use crate::protocol::{FirstViewResponse, GuildSelfProfile};

pub mod builder;
pub mod decoder;
pub mod processor;

pub struct GuildClient {
    rq_client: Arc<ricq::Client>,
    listeners: HashMap<&'static str, broadcast::Receiver<Packet>>,
}

impl GuildClient {
    pub async fn new(rq_client: &Arc<ricq::Client>) -> Self {
        let rq_client = rq_client.clone();

        let listeners = HashMap::new();

        Self {
            rq_client,
            listeners
        }
    }

    pub async fn engine(&self) -> Engine<'_> {
        Engine::from_rq(self.rq_client.engine.read().await)
    }

    pub async fn fetch_guild_first_view(&self) -> RQResult<Option<FirstViewResponse>> {
        let engine = self.engine().await;
        let pkt = engine.build_sync_channel_first_view_packet();
        let rsp = self.rq_client.send_and_wait(pkt).await?;

        let first_view = Decoder.decode_guild_first_view_response(rsp.body)?;

        Ok(first_view)
    }

    pub async fn fetch_guild_self_profile(&self, tiny_id: u64) -> RQResult<Option<GuildSelfProfile>> {
        let engine = self.engine().await;

        let pkt = engine.build_get_user_profile_packet(tiny_id);
        let rsp = self.rq_client.send_and_wait(pkt).await?;
        Decoder.decode_guild_self_profile(rsp.body)
    }
}

pub struct Engine<'a>(RwLockReadGuard<'a, ricq_core::Engine>);

impl<'a> Engine<'a> {
    fn from_rq(engine: RwLockReadGuard<'a, ricq_core::Engine>) -> Self {
        Self(engine)
    }
}

impl<'a> Deref for Engine<'a> {
    type Target = ricq_core::Engine;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}