use std::{fmt::Debug, sync::Arc};

use dashmap::DashMap;
use poise::{
    serenity_prelude::{ChannelId, Context, GuildId, Ready},
    Framework,
};
use rs621::{client::Client, post::Post};
use tokio::sync::watch;
use tracing::{debug, error, info, instrument};

use crate::{
    configuration::GuildConfiguration,
    tasks::{delete_button_listener, poke_loop},
    utils::NsfwMode,
};

#[derive(Clone)]
pub struct Data {
    /// configurations for all known guilds
    guild_configurations: Arc<DashMap<GuildId, GuildConfiguration>>,
    /// nsfw client
    e621_client: Arc<Client>,
    /// sfw client
    e926_client: Arc<Client>,
    /// serenity context
    context: Context,
}

impl Debug for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Data")
            .field("guild_configurations", &self.guild_configurations)
            .field("e621_client", &self.e621_client)
            .field("e926_client", &self.e926_client)
            //.field("context", &self.context)
            .finish()
    }
}

impl Data {
    fn new(context: Context) -> Result<Self, crate::Error> {
        let user_agent = "CutePokebot/0.1.0 (norom)";

        let (e6_client, e9_client) =
            if let (Ok(login), Ok(token)) = (dotenv::var("E6_LOGIN"), dotenv::var("E6_TOKEN")) {
                info!("Using logged in clients with user {}", &login);
                let mut e6_client = Client::new("https://e621.net", &user_agent)?;
                e6_client.login(login.clone(), token.clone());

                let mut e9_client = Client::new("https://e926.net", &user_agent)?;
                e9_client.login(login, token);
                (e6_client, e9_client)
            } else {
                info!("Using logged out clients");
                (
                    Client::new("https://e621.net", &user_agent)?,
                    Client::new("https://e926.net", &user_agent)?,
                )
            };

        Ok(Self {
            guild_configurations: Arc::new(DashMap::new()),
            e621_client: Arc::new(e6_client),
            e926_client: Arc::new(e9_client),
            context,
        })
    }

    /// Start sending images to channel inside guild
    #[instrument(skip(self))]
    pub async fn start(&self, guild: GuildId, channel: ChannelId) {
        let mut entry = self.guild_configurations.entry(guild).or_default();
        if !entry.is_active(channel) {
            let (tx, rx) = watch::channel(false);
            entry.start(channel, tx);
            let _ = tokio::spawn(poke_loop(self.clone(), guild, channel, rx));
            info!("Started sending images to {}", channel);
        }
        debug!("Already sending images to {}", channel);
    }

    /// Stop sending images (inside the guild)
    #[instrument(skip(self))]
    pub async fn stop(&self, guild: GuildId, channel: ChannelId) {
        self.guild_configurations
            .entry(guild)
            .and_modify(|config| config.stop(channel));

        info!("Requesting task for {} to be stopped", channel);
    }

    /// Stop sending images (inside the guild)
    #[instrument(skip(self))]
    pub fn stop_all(&self) {
        self.guild_configurations
            .iter_mut()
            .for_each(|mut guild_conf| {
                guild_conf.stop_all();
            });
    }

    /// Get the data's timeout.
    pub async fn timeout(&self, guild: GuildId, channel: ChannelId) -> Option<u64> {
        let timeout = self
            .guild_configurations
            .get(&guild)
            .map(|c| c.timeout(&channel))
            .flatten();
        debug!("{:?} minutes", timeout);
        timeout
    }

    /// Set the data's timeout.
    pub async fn set_timeout(&self, guild: GuildId, channel: ChannelId, timeout: u64) {
        debug!("{:?} minutes", timeout);
        self.guild_configurations
            .entry(guild)
            .or_default()
            .set_timeout(channel, timeout);
    }

    /// Get the tags for a channel in a guild
    pub async fn tags(&self, guild: GuildId, channel: ChannelId) -> Option<Vec<String>> {
        let tags = self
            .guild_configurations
            .get(&guild)
            .map(|c| c.tags(&channel).cloned())
            .flatten();
        debug!("{:?}", tags);
        tags
    }

    /// Set the tags for a channel in a guild
    pub async fn set_tags(&self, guild: GuildId, channel: ChannelId, tags: Vec<String>) {
        debug!("{:?}", tags);
        self.guild_configurations
            .entry(guild)
            .or_default()
            .set_tags(channel, tags);
    }

    /// Get the nsfw_mode for a channel in a guild
    pub async fn nsfw_mode(&self, guild: GuildId, channel: ChannelId) -> Option<NsfwMode> {
        let nsfw_mode = self
            .guild_configurations
            .get(&guild)
            .map(|c| c.nsfw_mode(&channel))
            .flatten();
        debug!("{:?}", nsfw_mode);
        nsfw_mode
    }

    /// Set the nsfw_mode for a channel in a guild
    pub async fn set_nsfw_mode(&self, guild: GuildId, channel: ChannelId, nsfw_mode: NsfwMode) {
        debug!("{:?}", nsfw_mode);
        self.guild_configurations
            .entry(guild)
            .or_default()
            .set_nsfw_mode(channel, nsfw_mode);
    }

    /// Return true if random_timeout is enabled for a channel in a guild
    pub async fn random_timeout(&self, guild: GuildId, channel: ChannelId) -> Option<bool> {
        let random_timeout = self
            .guild_configurations
            .get(&guild)
            .map(|c| c.random_timeout(&channel))
            .flatten();
        debug!("{:?}", random_timeout);
        random_timeout
    }

    /// Set if random_timeout is to be used for a channel in a guild
    pub async fn set_random_timeout(
        &self,
        guild: GuildId,
        channel: ChannelId,
        random_timeout: bool,
    ) {
        debug!(
            "setting random_timeout for {}/{}: {:?}",
            guild, channel, random_timeout
        );
        self.guild_configurations
            .entry(guild)
            .or_default()
            .set_random_timeout(channel, random_timeout);
    }

    /// Get's a random post according to the configuration of the given channel
    /// inside the given guild
    #[instrument(skip(self))]
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
            Ok(post) => {
                info!(post_id = post.id);
                Some(post)
            }
            Err(err) => {
                error!("{:?}", err);
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
    let data = Data::new(context.clone())?;
    let _ = tokio::spawn(delete_button_listener(context.clone()));
    Ok(data)
}
