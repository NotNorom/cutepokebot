use poise::{
    send_reply,
    serenity_prelude::{ChannelId, MessageBuilder},
};

use crate::{Context, Error};

/// Stops sending pokemon
#[poise::command(
    prefix_command,
    slash_command,
    required_permissions = "MANAGE_CHANNELS"
)]
pub async fn stop(
    ctx: Context<'_>,
    #[description = "Channel to stop"] channel: Option<ChannelId>,
) -> Result<(), Error> {
    let guild = ctx.guild_id().ok_or("Command must be run in server")?;
    let channel = channel.unwrap_or_else(|| ctx.channel_id());

    send_reply(ctx, |f| {
        let content = MessageBuilder::new()
            .channel(channel)
            .push(" will no longer receive pokemon.")
            .build();
        f.content(content).ephemeral(true)
    })
    .await?;

    ctx.data().stop(guild, channel).await;

    Ok(())
}
