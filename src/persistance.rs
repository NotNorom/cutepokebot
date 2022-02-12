use std::str::FromStr;

use fred::{
    self,
    error::RedisErrorKind,
    prelude::RedisError,
    types::{RedisResponse, RedisValue},
};
use poise::serenity_prelude::{GuildId, ChannelId};

use crate::{configuration::ChannelConfiguration, utils::NsfwMode};

pub async fn get_channel_config(guild: GuildId, channel: ChannelId) -> Result<ChannelConfiguration, RedisError> {
    Err(RedisError::new_canceled())
}


impl RedisResponse for ChannelConfiguration {
    fn from_value(value: RedisValue) -> Result<Self, RedisError> {
        let value = value.into_map()?;

        let active = value
            .get("active")
            .ok_or(RedisError::new(
                RedisErrorKind::NotFound,
                "missing key: active",
            ))?
            .as_bool()
            .ok_or(RedisError::new(
                RedisErrorKind::Parse,
                "invalid value for key: active",
            ))?;

        let timeout = value
            .get("timeout")
            .ok_or(RedisError::new(
                RedisErrorKind::NotFound,
                "missing key: timeout",
            ))?
            .as_u64()
            .ok_or(RedisError::new(
                RedisErrorKind::Parse,
                "invalid value for key: timeout",
            ))?;

        let random_timeout = value
            .get("random_timeout")
            .ok_or(RedisError::new(
                RedisErrorKind::NotFound,
                "missing key: random_timeout",
            ))?
            .as_bool()
            .ok_or(RedisError::new(
                RedisErrorKind::Parse,
                "invalid value for key: random_timeout",
            ))?;

        let nsfw_mode = value
            .get("nsfw_mode")
            .ok_or(RedisError::new(
                RedisErrorKind::NotFound,
                "missing key: nsfw_mode",
            ))?
            .clone()
            .convert::<NsfwMode>()?;

        let tags = value
            .get("tags")
            .ok_or(RedisError::new(
                RedisErrorKind::NotFound,
                "missing key: tags",
            ))?
            .clone()
            .convert::<Vec<String>>()?;

        Ok(Self {
            active,
            timeout,
            random_timeout,
            nsfw_mode,
            tags,
        })
    }
}

impl RedisResponse for NsfwMode {
    fn from_value(value: RedisValue) -> Result<Self, RedisError> {
        let value = value.as_str().ok_or(RedisError::new(
            RedisErrorKind::NotFound,
            "Nsfw mode is not a string",
        ))?;
        let mode = NsfwMode::from_str(&value)
            .map_err(|e| RedisError::new(RedisErrorKind::Parse, e.to_string()))?;
        Ok(mode)
    }
}
