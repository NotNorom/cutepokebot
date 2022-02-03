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

use crate::{
    configuration::{ChannelConfiguration, GuildConfiguration},
    tasks::poke_loop,
    utils::NsfwMode,
};

#[derive(Clone, Debug)]
pub struct Data {
    /// map of guilds and their channel which to send pokemon to
    channels: Arc<RwLock<HashMap<GuildId, ChannelId>>>,
    /// configurations for all known guilds
    guild_configurations: Arc<RwLock<HashMap<GuildId, GuildConfiguration>>>,
    /// timeout in minutes
    timeout: Arc<AtomicU64>,

    
}

impl Data {
    fn new() -> Self {
        Self {
            channels: Arc::new(RwLock::new(HashMap::new())),
            guild_configurations: Arc::new(RwLock::new(HashMap::new())),
            // 40 minutes as the default timeout
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

    /// Get the tags for a channel in a guild
    pub async fn tags(&self, guild: GuildId, channel: ChannelId) -> Option<Vec<String>> {
        let conf = self.guild_configurations.read().await;
        conf.get(&guild)
            .map(|c| c.tags(&channel).cloned())
            .flatten()
    }

    /// Set the tags for a channel in a guild
    pub async fn set_tags(&self, guild: GuildId, channel: ChannelId, tags: Vec<String>) {
        let mut conf = self.guild_configurations.write().await;
        conf.entry(guild).and_modify(|c| c.set_tags(channel, tags));
    }

    /// Get the nsfw_mode for a channel in a guild
    pub async fn nsfw_mode(&self, guild: GuildId, channel: ChannelId) -> Option<NsfwMode> {
        let conf = self.guild_configurations.read().await;
        conf.get(&guild).map(|c| c.nsfw_mode(&channel)).flatten()
    }
    /// Set the nsfw_mode for a channel in a guild
    pub async fn set_nsfw_mode(&self, guild: GuildId, channel: ChannelId, nsfw_mode: NsfwMode) {
        let mut conf = self.guild_configurations.write().await;
        conf.entry(guild)
            .and_modify(|c| c.set_nsfw_mode(channel, nsfw_mode));
    }
    }
}

pub async fn setup<U, E>(
    context: &Context,
    _ready: &Ready,
    _framework: &Framework<U, E>,
) -> Result<crate::Data, crate::Error> {
    let data = Data::new();

    tokio::spawn(poke_loop(context.clone(), data.clone()));

    Ok(data)
}
