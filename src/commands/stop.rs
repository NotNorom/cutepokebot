use poise::{send_reply, serenity_prelude::MessageBuilder};

use crate::{Context, Error};

/// Stop sending images
#[poise::command(
    prefix_command,
    slash_command,
    required_permissions = "MANAGE_CHANNELS"
)]
pub async fn stop(ctx: Context<'_>) -> Result<(), Error> {
    let guild = ctx.guild_id().ok_or(Error::CommandNotRunInGuild)?;
    let channel = ctx.channel_id();

    send_reply(ctx, |f| {
        let content = MessageBuilder::new()
            .channel(channel)
            .push(" will no longer receive images.")
            .build();
        f.content(content).ephemeral(true)
    })
    .await?;

    ctx.data().stop(guild, channel).await;

    Ok(())
}
