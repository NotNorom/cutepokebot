use poise::{
    send_reply,
    serenity_prelude::{ChannelId, MessageBuilder},
};

use crate::{configuration::ChannelConfiguration, utils::NsfwMode, Context, Error};

/// Display your or another user's account creation date
#[poise::command(
    prefix_command,
    slash_command,
    required_permissions = "MANAGE_CHANNELS"
)]
pub async fn start(
    ctx: Context<'_>,
    #[description = "Selected channel"] channel: Option<ChannelId>,
    #[description = "Use nsfw or sfw mode"] nsfw_mode: Option<NsfwMode>,
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

    let config = ChannelConfiguration::new(nsfw_mode.unwrap_or_default(), None);

    ctx.data().start(guild, channel, config).await;

    Ok(())
}
