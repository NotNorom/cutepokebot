use poise::serenity_prelude::ChannelId;

use crate::{Context, Error};

pub async fn channel_is_in_current_guild(
    ctx: Context<'_>,
    channel_id: ChannelId,
) -> Result<bool, Error> {
    Ok(true)
}
