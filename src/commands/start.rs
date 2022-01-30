use poise::{
    send_reply,
    serenity_prelude::{ChannelId, MessageBuilder},
};

use crate::{Context, Error};

/// Display your or another user's account creation date
#[poise::command(
    prefix_command,
    slash_command,
    required_permissions = "MANAGE_CHANNELS"
)]
pub async fn start(
    ctx: Context<'_>,
    #[description = "Selected user"] channel: Option<ChannelId>,
) -> Result<(), Error> {
    let guild = ctx.guild_id().ok_or("Command must be run in server")?;
    let channel = channel.unwrap_or_else(|| ctx.channel_id());

    send_reply(ctx, |f| {
        let content = MessageBuilder::new()
            .push("Channel ")
            .channel(channel)
            .push(" will receive pokemon.")
            .build();

        f.content(content).ephemeral(true)
    })
    .await?;

    ctx.data().add(guild, channel).await;

    Ok(())
}
