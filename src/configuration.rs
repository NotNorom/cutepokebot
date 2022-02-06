use std::collections::{HashMap, HashSet};

use poise::serenity_prelude::{ChannelId, RoleId};
use tokio::sync::watch;
use tracing::error;

use crate::utils::NsfwMode;

#[non_exhaustive]
#[derive(Debug, Default)]
pub struct GuildConfiguration {
    /// channel specific configurations
    channels: HashMap<ChannelId, ChannelConfiguration>,
    /// roles which are allowed to use the bot
    moderator_roles: HashSet<RoleId>,
    /// signal for every channel that is running right now
    stop_signals: HashMap<ChannelId, watch::Sender<bool>>,
}

impl GuildConfiguration {
    pub fn start(&mut self, channel: ChannelId, stop_sender: watch::Sender<bool>) {
        self.stop_signals.entry(channel).or_insert(stop_sender);
        self.channels.entry(channel).or_default().active = true;
    }

    pub fn stop(&mut self, channel: ChannelId) {
        if let Some(stop_signal) = self.stop_signals.remove(&channel) {
            if let Err(err) = stop_signal.send(true) {
                error!("Could not send stop signal for {}: {}", channel, err);
            }
        };
        self.channels.entry(channel).or_default().active = false;
    }

    /// Stops all sending tasks
    pub fn stop_all(&mut self) {
        self.stop_signals.iter().for_each(|(channel, stop_signal)| {
            if let Err(err) = stop_signal.send(true) {
                error!("Could not send stop signal for {}: {}", channel, err);
            }
        });
        self.stop_signals.clear();
        self.channels.iter_mut().for_each(|(_, channel_conf)| {
            channel_conf.active = false;
        });
    }

    pub fn timeout(&self, channel: &ChannelId) -> Option<u64> {
        self.channels.get(channel).map(|c| c.timeout)
    }

    pub fn set_timeout(&mut self, channel: ChannelId, timeout: u64) {
        self.channels.entry(channel).or_default().timeout = timeout;
    }

    pub fn random_timeout(&self, channel: &ChannelId) -> Option<bool> {
        self.channels.get(channel).map(|c| c.random_timeout)
    }

    pub fn set_random_timeout(&mut self, channel: ChannelId, random_timeout: bool) {
        self.channels.entry(channel).or_default().random_timeout = random_timeout;
    }

    pub fn nsfw_mode(&self, channel: &ChannelId) -> Option<NsfwMode> {
        self.channels.get(channel).map(|c| c.nsfw_mode)
    }

    pub fn set_nsfw_mode(&mut self, channel: ChannelId, nsfw_mode: NsfwMode) {
        self.channels.entry(channel).or_default().nsfw_mode = nsfw_mode;
    }

    pub fn tags(&self, channel: &ChannelId) -> Option<&Vec<String>> {
        self.channels.get(channel).map(|c| &c.tags)
    }

    pub fn set_tags(&mut self, channel: ChannelId, tags: Vec<String>) {
        self.channels.entry(channel).or_default().tags = tags;
    }

    pub fn is_active(&self, channel: ChannelId) -> bool {
        self.channels
            .get(&channel)
            .map(|c| c.active)
            .unwrap_or_default()
    }
}

#[non_exhaustive]
#[derive(Debug, Clone)]
pub struct ChannelConfiguration {
    /// True if the posting loop is running
    active: bool,
    /// Timeout in minutes
    timeout: u64,
    /// True if timeout should be interpreted as a maximum timeout
    random_timeout: bool,
    /// If the query should return sfw or nsfw posts
    nsfw_mode: NsfwMode,
    /// The tags to search for
    tags: Vec<String>,
}

impl Default for ChannelConfiguration {
    fn default() -> Self {
        Self {
            active: false,
            timeout: 40,
            random_timeout: false,
            nsfw_mode: NsfwMode::SFW,
            tags: vec![
                "pokémon_(species)",
                "-abs",
                "-blob(disambiguation)",
                "-breasts",
                "-butt",
                "-card_game",
                "-comic",
                "-diaper",
                "-dominatrix",
                "-expansion",
                "-foot_focus",
                "-gore",
                "-human",
                "-inflation",
                "-model_sheet",
                "-muscular",
                "-nightmare_fuel",
                "-nipples",
                "-overweight",
                "-pokémorph",
                "-pregnant",
                "-seductive",
                "-thick_thighs",
                "-transformation",
                "-vore",
                "score:>5",
            ]
            .into_iter()
            .map(|s| s.to_string())
            .collect(),
        }
    }
}
