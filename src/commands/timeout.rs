use poise::{send_reply, serenity_prelude::ChannelId};

use crate::{constants::MINIMUM_TIMEOUT_MINUTES, Context, Error};

/// Gets or sets the timeout for the channel in the guild
#[poise::command(prefix_command, slash_command, owners_only)]
pub async fn timeout(
    ctx: Context<'_>,
    #[description = "Selected channel"] channel: Option<ChannelId>,
    #[description = "Timeout in minutes"] timeout: Option<u64>,
) -> Result<(), Error> {
    let guild = ctx.guild_id().ok_or("Command must be run in server")?;
    let channel = channel.unwrap_or_else(|| ctx.channel_id());

    let current_timeout = ctx.data().timeout(guild, channel).await;

    let content = if let Some(new_timeout) = timeout {
        if new_timeout <= MINIMUM_TIMEOUT_MINUTES {
            format!("Timeout must be greater than {}", MINIMUM_TIMEOUT_MINUTES)
        } else if let Some(current_timeout) = current_timeout {
            ctx.data().set_timeout(guild, channel, new_timeout).await;
            format!(
                "Old timeout: {} minutes\nNew timeout: {} minutes",
                current_timeout, new_timeout
            )
        } else {
            ctx.data().set_timeout(guild, channel, new_timeout).await;
            format!(
                "Old timeout is not set.\nNew timeout is: {} minutes",
                new_timeout
            )
        }
    } else if let Some(current_timeout) = current_timeout {
        current_timeout.to_string()
    } else {
        "Timeout is not set.\n".to_string()
    };

    send_reply(ctx, |f| f.content(content).ephemeral(true)).await?;

    Ok(())
}
