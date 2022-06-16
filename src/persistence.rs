use std::{collections::HashSet, str::FromStr};

use fred::{
    self,
    clients::RedisClient,
    error::RedisErrorKind,
    interfaces::{HashesInterface, SetsInterface},
    prelude::RedisError,
    types::{FromRedis, RedisKey, RedisValue},
};
use poise::serenity_prelude::{ChannelId, GuildId, MessageId, RoleId};

use crate::{
    configuration::{ChannelConfiguration, GuildConfiguration, NsfwMode, TimeoutMode},
    constants::{REDIS_PATH_SEPARATOR as SEP, REDIS_PREFIX},
};

pub async fn known_guild_ids(redis: &RedisClient) -> Result<Vec<GuildId>, RedisError> {
    let guild_ids: Vec<String> = redis
        .smembers(format!("{REDIS_PREFIX}{SEP}KNOWN_GUILDS"))
        .await?;
    guild_ids
        .into_iter()
        .map(|id| id.parse::<u64>())
        .map(|maybe_id| {
            maybe_id
                .map(GuildId)
                .map_err(|err| RedisError::new(RedisErrorKind::Parse, err.to_string()))
        })
        .collect()
}

pub async fn known_channel_ids(
    redis: &RedisClient,
    guild: GuildId,
) -> Result<Vec<ChannelId>, RedisError> {
    let channel_ids: Vec<String> = redis
        .smembers(format!("{REDIS_PREFIX}{SEP}KNOWN_CHANNELS{SEP}{guild}"))
        .await?;
    channel_ids
        .into_iter()
        .map(|id| id.parse::<u64>())
        .map(|maybe_id| {
            maybe_id
                .map(ChannelId)
                .map_err(|err| RedisError::new(RedisErrorKind::Parse, err.to_string()))
        })
        .collect()
}

pub async fn known_message_ids(
    redis: &RedisClient,
    channel: ChannelId,
) -> Result<Vec<MessageId>, RedisError> {
    let message_ids: Vec<String> = redis
        .smembers(format!("{REDIS_PREFIX}{SEP}KNOWN_MESSAGES{SEP}{channel}"))
        .await?;
    message_ids
        .into_iter()
        .map(|id| id.parse::<u64>())
        .map(|maybe_id| {
            maybe_id
                .map(MessageId)
                .map_err(|err| RedisError::new(RedisErrorKind::Parse, err.to_string()))
        })
        .collect()
}

pub async fn get_guild_config(
    redis: &RedisClient,
    guild: GuildId,
) -> Result<GuildConfiguration, RedisError> {
    let config = redis
        .hgetall(format!("{REDIS_PREFIX}{SEP}GUILD_CONF{SEP}{guild}"))
        .await;
    config
}

pub async fn get_channel_config(
    redis: &RedisClient,
    channel: ChannelId,
) -> Result<ChannelConfiguration, RedisError> {
    let config = redis
        .hgetall(format!("{REDIS_PREFIX}{SEP}CHANNEL_CONF{SEP}{channel}"))
        .await;
    config
}

impl FromRedis for GuildConfiguration {
    fn from_value(value: RedisValue) -> Result<Self, RedisError> {
        let value = value.into_map()?;
        let moderator_roles = value
            .get(&RedisKey::from_static_str("moderator_roles"))
            .ok_or_else(|| {
                RedisError::new(RedisErrorKind::NotFound, "missing key: moderator_roles")
            })?
            .clone()
            .convert::<String>()?;

        let moderator_roles: Result<HashSet<RoleId>, RedisError> = moderator_roles
            .split_whitespace()
            .map(|s| {
                s.parse::<u64>()
                    .map(RoleId)
                    .map_err(|err| RedisError::new(RedisErrorKind::Parse, err.to_string()))
            })
            .collect();

        let moderator_roles = moderator_roles?;

        Ok(Self {
            channels: Default::default(),
            moderator_roles,
            stop_signals: Default::default(),
        })
    }
}

impl FromRedis for ChannelConfiguration {
    fn from_value(value: RedisValue) -> Result<Self, RedisError> {
        let value = value.into_map()?;

        let active = value
            .get(&RedisKey::from_static_str("active"))
            .ok_or_else(|| RedisError::new(RedisErrorKind::NotFound, "missing key: active"))?
            .as_bool()
            .ok_or_else(|| {
                RedisError::new(RedisErrorKind::Parse, "invalid value for key: active")
            })?;

        let timeout = value
            .get(&RedisKey::from_static_str("timeout"))
            .ok_or_else(|| RedisError::new(RedisErrorKind::NotFound, "missing key: timeout"))?
            .as_u64()
            .ok_or_else(|| {
                RedisError::new(RedisErrorKind::Parse, "invalid value for key: timeout")
            })?;

        let timeout_mode = value
            .get(&RedisKey::from_static_str("timeout_mode"))
            .ok_or_else(|| RedisError::new(RedisErrorKind::NotFound, "missing key: timeout_mode"))?
            .clone()
            .convert::<TimeoutMode>()?;

        let nsfw_mode = value
            .get(&RedisKey::from_static_str("nsfw_mode"))
            .ok_or_else(|| RedisError::new(RedisErrorKind::NotFound, "missing key: nsfw_mode"))?
            .clone()
            .convert::<NsfwMode>()?;

        let tags = value
            .get(&RedisKey::from_static_str("tags"))
            .ok_or_else(|| RedisError::new(RedisErrorKind::NotFound, "missing key: tags"))?
            .clone()
            .convert::<Vec<String>>()?;

        Ok(Self {
            active,
            timeout,
            timeout_mode,
            nsfw_mode,
            tags,
        })
    }
}

impl FromRedis for NsfwMode {
    fn from_value(value: RedisValue) -> Result<Self, RedisError> {
        let value = value.as_str().ok_or_else(|| {
            RedisError::new(RedisErrorKind::NotFound, "Nsfw mode is not a string")
        })?;
        let mode = Self::from_str(&value)
            .map_err(|e| RedisError::new(RedisErrorKind::Parse, e.to_string()))?;
        Ok(mode)
    }
}

impl FromRedis for TimeoutMode {
    fn from_value(value: RedisValue) -> Result<Self, RedisError> {
        let value = value.as_str().ok_or_else(|| {
            RedisError::new(RedisErrorKind::NotFound, "Timeout mode is not a string")
        })?;
        let mode = Self::from_str(&value)
            .map_err(|e| RedisError::new(RedisErrorKind::Parse, e.to_string()))?;
        Ok(mode)
    }
}
