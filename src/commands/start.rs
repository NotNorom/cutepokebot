use poise::{
    send_reply,
    serenity_prelude::{ChannelId, MessageBuilder},
};

use crate::{Context, Error};

/// Start posting images in the channel
#[poise::command(
    prefix_command,
    slash_command,
    required_permissions = "MANAGE_CHANNELS"
)]
pub async fn start(
    ctx: Context<'_>,
    #[description = "Selected channel"] channel: Option<ChannelId>,
) -> Result<(), Error> {
    let guild = ctx.guild_id().ok_or("Command must be run in server")?;
    let channel = channel.unwrap_or_else(|| ctx.channel_id());

    send_reply(ctx, |f| {
        let content = MessageBuilder::new()
            .push("Channel ")
            .channel(channel)
            .push(" will receive images.")
            .build();

        f.content(content).ephemeral(true)
    })
    .await?;

    ctx.data().start(guild, channel).await;

    Ok(())
}
