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
    /// the tags which to search for
    tags: Arc<RwLock<Vec<String>>>,
}

impl Data {
    fn new<T, S>(tags: T) -> Self
    where
        T: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let tags = tags.into_iter().map(|s| s.into()).collect();
        Self {
            channels: Arc::new(RwLock::new(HashMap::new())),
            timeout: Arc::new(AtomicU64::new(40)),
            // e6 allows up to 40 tags in their search
            tags: Arc::new(RwLock::new(tags)),
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

    /// Get an arc to the data's tags.
    pub fn tags(&self) -> Arc<RwLock<Vec<String>>> {
        self.tags.clone()
    }
}

pub async fn setup<U, E>(
    context: &Context,
    _ready: &Ready,
    _framework: &Framework<U, E>,
) -> Result<crate::Data, crate::Error> {
    let data = Data::new([
        "pokémon_(species)",
        "-vore",
        "-gore",
        "-transformation",
        "-pokémorph",
        "-comic",
        "-pregnant",
        "-foot_focus",
        "-seductive",
        "score:>55",
    ]);

    tokio::spawn(poke_loop(context.http.clone(), data.clone()));

    Ok(data)
}
