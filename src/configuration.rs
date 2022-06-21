use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

use poise::{
    serenity_prelude::{ChannelId, RoleId},
    ChoiceParameter,
};
use tokio::sync::watch;
use tracing::error;

use crate::{
    constants::{MAXIMUM_TIMEOUT_MINUTES, MINIMUM_TIMEOUT_MINUTES},
    Error,
};

#[non_exhaustive]
#[derive(Debug, Default)]
pub struct GuildConfiguration {
    /// channel specific configurations
    pub(crate) channels: HashMap<ChannelId, ChannelConfiguration>,
    /// roles which are allowed to use the bot
    #[allow(unused)]
    pub(crate) moderator_roles: HashSet<RoleId>,
    /// signal for every channel that is running right now
    pub(crate) stop_signals: HashMap<ChannelId, watch::Sender<bool>>,
}

impl GuildConfiguration {
    pub fn insert(
        &mut self,
        channel: ChannelId,
        config: ChannelConfiguration,
    ) -> Option<ChannelConfiguration> {
        self.channels.insert(channel, config)
    }

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

    pub fn timeout_mode(&self, channel: &ChannelId) -> Option<TimeoutMode> {
        self.channels.get(channel).map(|c| c.timeout_mode)
    }

    pub fn set_timeout_mode(&mut self, channel: ChannelId, timeout_mode: TimeoutMode) {
        self.channels.entry(channel).or_default().timeout_mode = timeout_mode;
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
    pub(crate) active: bool,
    /// Timeout in minutes
    pub(crate) timeout: u64,
    /// True if timeout should be interpreted as a maximum timeout
    pub(crate) timeout_mode: TimeoutMode,
    /// If the query should return sfw or nsfw posts
    pub(crate) nsfw_mode: NsfwMode,
    /// The tags to search for
    pub(crate) tags: Vec<String>,
}

impl Default for ChannelConfiguration {
    fn default() -> Self {
        Self {
            active: false,
            timeout: 40,
            timeout_mode: TimeoutMode::Normal,
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
                "-fishnets",
                "-flash",
                "-foot_focus",
                "-gore",
                "-human",
                "-inflation",
                "-model_sheet",
                "-muscular",
                "-nightmare_fuel",
                "-nipples",
                "-overweight",
                "-panties",
                "-pokémorph",
                "-pregnant",
                "-seductive",
                "-syringe",
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

/// NSFW mode. Default is SFW
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Copy, ChoiceParameter)]
pub enum NsfwMode {
    #[name = "sfw"]
    SFW,
    #[name = "nsfw"]
    NSFW,
}

impl Default for NsfwMode {
    fn default() -> Self {
        Self::SFW
    }
}

impl Display for NsfwMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SFW => write!(f, "sfw"),
            Self::NSFW => write!(f, "nsfw"),
        }
    }
}

/// Timeout mode. Default is normal
#[derive(Debug, Clone, Copy, ChoiceParameter)]
pub enum TimeoutMode {
    #[name = "normal"]
    Normal,
    #[name = "random"]
    Random,
}

impl Default for TimeoutMode {
    fn default() -> Self {
        Self::Normal
    }
}

impl Display for TimeoutMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Normal => write!(f, "normal"),
            Self::Random => write!(f, "random"),
        }
    }
}

#[derive(Debug, Clone, Copy)]

pub struct Timeout {
    min: u64,
    max: u64,
}

impl Timeout {
    pub fn new(min: u64, max: u64) -> Result<Self, Error> {
        if min < MINIMUM_TIMEOUT_MINUTES {
            return Err(Error::MinTimeoutTooLow);
        }

        if max > MAXIMUM_TIMEOUT_MINUTES {
            return Err(Error::MaxTimeoutTooHigh);
        }

        Ok(Self { min, max })
    }

    pub fn min(&self) -> u64 {
        self.min
    }

    pub fn max(&self) -> u64 {
        self.max
    }
}
