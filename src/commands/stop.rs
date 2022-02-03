use poise::{send_reply, serenity_prelude::ChannelId};

use crate::{Context, Error};

/// Display your or another user's account creation date
#[poise::command(
    prefix_command,
    slash_command,
    required_permissions = "MANAGE_CHANNELS"
)]
pub async fn stop(
    ctx: Context<'_>,
    #[description = "Stop posting images in channel"] channel: Option<ChannelId>,
) -> Result<(), Error> {
    let guild = ctx.guild_id().ok_or("Command must be run in server")?;
    let channel = channel.unwrap_or_else(|| ctx.channel_id());

    send_reply(ctx, |f| {
        let content = "This server will no longer receive pokemon.";
        f.content(content).ephemeral(true)
    })
    .await?;

    ctx.data().stop(guild, channel).await;

    Ok(())
}
