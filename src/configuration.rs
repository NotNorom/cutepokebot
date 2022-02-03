use std::collections::HashMap;

use poise::serenity_prelude::ChannelId;

use crate::utils::NsfwMode;

#[non_exhaustive]
#[derive(Debug, Clone, Default)]
pub struct GuildConfiguration {
    channels: HashMap<ChannelId, ChannelConfiguration>,
}

impl GuildConfiguration {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn add_channel(&mut self, channel: ChannelId, config: ChannelConfiguration) {
        self.channels.insert(channel, config);
    }

    pub fn remove_channel(&mut self, channel: &ChannelId) {
        self.channels.remove(channel);
    }

    pub fn tags(&self, channel: &ChannelId) -> Option<&Vec<String>> {
        self.channels.get(channel).map(|c| &c.tags)
    }

    pub fn set_tags(&mut self, channel: ChannelId, tags: Vec<String>) {
        self.channels.entry(channel).or_default().tags = tags;
    }

    pub fn nsfw_mode(&self, channel: &ChannelId) -> Option<NsfwMode> {
        self.channels.get(channel).map(|c| c.nsfw_mode)
    }

    pub fn set_nsfw_mode(&mut self, channel: ChannelId, nsfw_mode: NsfwMode) {
        self.channels.entry(channel).or_default().nsfw_mode = nsfw_mode;
    }

    pub fn timeout(&self, channel: &ChannelId) -> Option<u64> {
        self.channels.get(channel).map(|c| c.timeout)
    }

    pub fn set_timeout(&mut self, channel: ChannelId, timeout: u64) {
        self.channels.entry(channel).or_default().timeout = timeout;
    }
}

#[non_exhaustive]
#[derive(Debug, Clone)]
pub struct ChannelConfiguration {
    pub timeout: u64,
    pub nsfw_mode: NsfwMode,
    pub tags: Vec<String>,
}

impl Default for ChannelConfiguration {
    fn default() -> Self {
        Self {
            timeout: 40,
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

impl ChannelConfiguration {
    /// Ff tags is None, use default list of tags (cute pokemon)
    pub fn new(timeout: u64, nsfw_mode: NsfwMode, tags: Option<Vec<String>>) -> Self {
        match tags {
            Some(tags) => Self {
                timeout,
                nsfw_mode,
                tags,
            },
            None => Self {
                timeout,
                nsfw_mode,
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
            },
        }
    }
}
