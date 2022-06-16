#![allow(unused_imports)]

use std::{fmt::Debug, sync::Arc, time::Duration};

use dashmap::DashMap;
use fred::{
    client::RedisClient,
    prelude::RedisError,
    types::{ReconnectPolicy, RedisConfig},
};

use poise::{
    futures_util::{FutureExt, StreamExt},
    serenity_prelude::{ChannelId, Context, GuildId, MessageId, Ready},
    Framework,
};
use rand::Rng;
use rs621::{client::Client, post::Post};
use tokio::{
    sync::watch::{self, Sender},
    time::sleep,
};
use tracing::{debug, info, instrument};

use crate::{
    configuration::{GuildConfiguration, NsfwMode},
    persistance::{get_channel_config, get_guild_config, known_channel_ids, known_guild_ids},
    tasks::{delete_button_listener, send_images_loop},
    Error,
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
    /// redis db handle
    redis: RedisClient,
    /// when a shutdown command is executed, this signal
    /// will be switched to true, signaling the shutdown functions
    /// to run
    shutdown_sender: Arc<Sender<bool>>,
    // post history
    // post_history: Arc<DashMap<MessageId, u64>>,
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
    async fn new(context: Context, shutdown_sender: Sender<bool>) -> Result<Self, crate::Error> {
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

        let redis = async {
            let config = RedisConfig::default();
            let policy = ReconnectPolicy::new_exponential(0, 100, 30_000, 2);
            let client = RedisClient::new(config);
            client.connect(Some(policy)).await??;
            client.wait_for_connect().await?;
            Ok::<RedisClient, RedisError>(client)
        }
        .await?;

        Ok(Self {
            guild_configurations: Arc::new(DashMap::new()),
            e621_client: Arc::new(e6_client),
            e926_client: Arc::new(e9_client),
            context,
            redis,
            shutdown_sender: Arc::new(shutdown_sender),
        })
    }

    pub(crate) async fn store_to_db(&self) -> Result<(), crate::Error> {
        Ok(())
    }

    pub(crate) async fn restore_from_db(&self) -> Result<(), crate::Error> {
        for guild_id in known_guild_ids(&self.redis).await? {
            let mut guild_conf = get_guild_config(&self.redis, guild_id).await?;

            for channel_id in known_channel_ids(&self.redis, guild_id).await? {
                guild_conf.insert(
                    channel_id,
                    get_channel_config(&self.redis, channel_id).await?,
                );
            }

            self.guild_configurations.insert(guild_id, guild_conf);
        }
        Ok(())
    }
    /// Start sending images to channel inside guild
    pub async fn start(&self, guild: GuildId, channel: ChannelId, delay: Option<u64>) {
        let mut entry = self.guild_configurations.entry(guild).or_default();
        if !entry.is_active(channel) {
            let (tx, rx) = watch::channel(false);
            entry.start(channel, tx);
            let self_clone = self.clone();
            tokio::spawn(async move {
                if let Some(delay) = delay {
                    sleep(Duration::from_secs(delay)).await;
                }
                send_images_loop(self_clone, guild, channel, rx).await;
            });
            info!("Started sending images to {}", channel);
        }
        debug!("Already sending images to {}", channel);
    }

    /// Stop sending images (inside the guild)
    pub async fn stop(&self, guild: GuildId, channel: ChannelId) {
        self.guild_configurations
            .entry(guild)
            .and_modify(|config| config.stop(channel));

        info!("Requesting task for {} to be stopped", channel);
    }

    /// Starts all tasks marked active.
    ///
    /// This function is supposed to be called only once,
    /// right after Data has been restored from the database.
    async fn start_all(&self) {
        for guild_conf in self.guild_configurations.iter() {
            let guild_id = *guild_conf.key();
            for (idx, (channel_id, channel_conf)) in guild_conf.channels.iter().enumerate() {
                if channel_conf.active {
                    // generate some random delay as not to spam e6 all at once.
                    // we'd get ratelimited anyway
                    let jitter = {
                        let mut rng = rand::thread_rng();
                        rng.gen_range(idx..idx + 180) as u64
                    };
                    self.start(guild_id, *channel_id, Some(jitter)).await;
                }
            }
        }
    }

    /// Stop sending images (inside the guild)
    pub fn stop_all(&self) {
        self.guild_configurations
            .iter_mut()
            .for_each(|mut guild_conf| {
                debug!("Stopping tasks for guild {}", guild_conf.key());
                guild_conf.stop_all();
            });
    }

    /// Sends a shutdown signal (as if presing ctrl-c)
    pub fn shutdown(&self) {
        let _ = self.shutdown_sender.send(true);
    }

    /// Get the data's timeout.
    pub async fn timeout(&self, guild: GuildId, channel: ChannelId) -> Option<u64> {
        let timeout = self
            .guild_configurations
            .get(&guild)
            .and_then(|c| c.timeout(&channel));
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
            .and_then(|c| c.tags(&channel).cloned());
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
            .and_then(|c| c.nsfw_mode(&channel));
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
            .and_then(|c| c.random_timeout(&channel));
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
    pub async fn get_post(&self, guild: GuildId, channel: ChannelId) -> Result<Post, Error> {
        let client = match self.nsfw_mode(guild, channel).await.unwrap_or_default() {
            NsfwMode::SFW => self.e926_client.clone(),
            NsfwMode::NSFW => self.e621_client.clone(),
        };

        let mut tags = self.tags(guild, channel).await.ok_or(Error::NoTagsSet)?;
        tags.extend_from_slice(&["order:random".to_string(), "limit:20".to_string()]);

        let mut post_search = Box::pin(
            client
                .post_search(&tags[..])
                .filter_map(|a| async { Result::ok(a) }),
        );

        let post = post_search.next().await;

        post.ok_or_else(|| Error::Uhhh("No posts this time...".to_string()))
    }

    /// Get a reference to the data's serenity context.
    pub fn context(&self) -> &Context {
        &self.context
    }
}

pub async fn setup<U, E>(
    context: &Context,
    _ready: &Ready,
    _framework: &Framework<U, E>,
    shutdown_sender: Sender<bool>,
) -> Result<crate::Data, crate::Error> {
    let data = Data::new(context.clone(), shutdown_sender).await?;
    data.restore_from_db().await?;
    data.start_all().await;
    let _ = tokio::spawn(delete_button_listener(context.clone()));
    Ok(data)
}
