use poise::serenity_prelude::ChannelId;

use crate::{Context, Error};

#[allow(dead_code, unused_variables)]
pub async fn channel_is_in_current_guild(
    ctx: Context<'_>,
    channel_id: ChannelId,
) -> Result<bool, Error> {
    Ok(true)
}
