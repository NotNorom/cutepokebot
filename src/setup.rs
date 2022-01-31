use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
};

use poise::{
    serenity_prelude::{ChannelId, Context, GuildId, Ready, RwLock},
    Framework,
};

use crate::tasks::poke_loop;

#[derive(Clone, Debug)]
pub struct Data {
    /// map of guilds and their channel which to send pokemon to
    channels: Arc<RwLock<HashMap<GuildId, ChannelId>>>,
    /// timeout in minutes
    timeout: Arc<AtomicU64>,
}

impl Data {
    fn new() -> Self {
        Self {
            channels: Arc::new(RwLock::new(HashMap::new())),
            timeout: Arc::new(AtomicU64::new(40)),
        }
    }

    /// Add channel for receiving pokemon
    pub async fn add(&self, guild: GuildId, channel: ChannelId) {
        let mut channels = self.channels.write().await;
        channels.insert(guild, channel);
    }

    /// Remove channel (inside the guild) to receive pokemon
    pub async fn remove(&self, guild: GuildId) {
        let mut channels = self.channels.write().await;
        channels.remove(&guild);
    }

    /// Get an arc to the data's channels.
    pub fn channels(&self) -> Arc<RwLock<HashMap<GuildId, ChannelId>>> {
        self.channels.clone()
    }

    /// Get the data's timeout.
    pub fn timeout(&self) -> u64 {
        self.timeout.load(Ordering::Relaxed)
    }

    /// Set the data's timeout.
    pub fn set_timeout(&self, timeout: u64) {
        self.timeout.store(timeout, Ordering::Relaxed);
    }
}

pub async fn setup<U, E>(
    context: &Context,
    _ready: &Ready,
    _framework: &Framework<U, E>,
) -> Result<crate::Data, crate::Error> {
    let data = Data::new();
    let poke_data = data.clone();

    let discord_http = context.http.clone();

    tokio::spawn(async move { poke_loop(discord_http, poke_data).await });

    Ok(data)
}
