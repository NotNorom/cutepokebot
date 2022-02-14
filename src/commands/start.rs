use poise::{send_reply, serenity_prelude::MessageBuilder};

use crate::{Context, Error};

/// Start posting images in the channel
#[poise::command(
    prefix_command,
    slash_command,
    required_permissions = "MANAGE_CHANNELS"
)]
pub async fn start(ctx: Context<'_>) -> Result<(), Error> {
    let guild = ctx.guild_id().ok_or(Error::CommandNotRunInGuild)?;
    let channel = ctx.channel_id();

    send_reply(ctx, |f| {
        let content = MessageBuilder::new()
            .push("Channel ")
            .channel(channel)
            .push(" will receive images.")
            .build();

        f.content(content).ephemeral(true)
    })
    .await?;

    ctx.data().start(guild, channel, None).await;

    Ok(())
}
