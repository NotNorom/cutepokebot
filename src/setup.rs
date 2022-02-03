use std::{
    collections::HashMap,
    sync::Arc,
};

use poise::{
    serenity_prelude::{ChannelId, Context, GuildId, Ready, RwLock},
    Framework,
};
use rs621::{client::Client, post::Post};

use crate::{
    configuration::GuildConfiguration,
    tasks::poke_loop,
    utils::NsfwMode,
};

#[derive(Clone)]
pub struct Data {
    /// configurations for all known guilds
    guild_configurations: Arc<RwLock<HashMap<GuildId, GuildConfiguration>>>,
    /// nsfw client
    e621_client: Arc<Client>,
    /// sfw client
    e926_client: Arc<Client>,
    /// serenity context
    context: Context,
}

impl Data {
    fn new(context: Context) -> Result<Self, crate::Error> {
        let user_agent = "CutePokebot/0.1.0 (norom)";

        Ok(Self {
            guild_configurations: Arc::new(RwLock::new(HashMap::new())),
            e621_client: Arc::new(Client::new("https://e621.net", &user_agent)?),
            e926_client: Arc::new(Client::new("https://e926.net", &user_agent)?),
            context,
        })
    }

    /// Add channel for receiving pokemon
    pub async fn start(&self, guild: GuildId, channel: ChannelId, config: ChannelConfiguration) {
        let mut guild_config = self.guild_configurations.write().await;
        guild_config
            .entry(guild)
            .or_default()
            .add_channel(channel, config);

        let _handle = tokio::spawn(poke_loop(self.clone(), guild, channel));
    }

    /// Remove channel (inside the guild) to receive pokemon
    pub async fn stop(&self, guild: GuildId, channel: ChannelId) {
        let mut guild_config = self.guild_configurations.write().await;
        guild_config
            .entry(guild)
            .and_modify(|config| config.remove_channel(&channel));
    }

    /// Get the data's timeout.
    pub async fn timeout(&self, guild: GuildId, channel: ChannelId) -> Option<u64> {
        let conf = self.guild_configurations.read().await;
        conf.get(&guild).map(|c| c.timeout(&channel)).flatten()
    }

    /// Set the data's timeout.
    pub async fn set_timeout(&self, guild: GuildId, channel: ChannelId, timeout: u64) {
        let mut conf = self.guild_configurations.write().await;
        conf.entry(guild).or_default().set_timeout(channel, timeout);
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

    /// Get's a random post according to the configuration of the given channel
    /// inside the given guild
    pub async fn get_post(&self, guild: GuildId, channel: ChannelId) -> Option<Post> {
        let client = match self.nsfw_mode(guild, channel).await {
            None => return None,
            Some(nsfw_mode) => match nsfw_mode {
                NsfwMode::SFW => self.e926_client.clone(),
                NsfwMode::NSFW => self.e621_client.clone(),
            },
        };

        let tags = self.tags(guild, channel).await;

        let post = match tags {
            Some(tags) => client.search_random_post(&tags[..]).await,
            None => return None,
        };

        match post {
            Ok(post) => Some(post),
            Err(err) => {
                eprintln!("{:?}", err);
                None
            }
        }
    }

    /// Get a reference to the data's context.
    pub fn context(&self) -> &Context {
        &self.context
    }
}

pub async fn setup<U, E>(
    context: &Context,
    _ready: &Ready,
    _framework: &Framework<U, E>,
) -> Result<crate::Data, crate::Error> {
    Ok(Data::new(context.clone())?)
}
