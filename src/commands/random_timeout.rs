use poise::send_reply;

use crate::{configuration::TimeoutMode, Context, Error};

/// Gets or sets the timeout mode
#[poise::command(
    prefix_command,
    slash_command,
    required_permissions = "MANAGE_CHANNELS"
)]
pub async fn timeout_mode(
    ctx: Context<'_>,
    #[description = "Timeout mode"] timeout_mode: Option<TimeoutMode>,
) -> Result<(), Error> {
    let guild = ctx.guild_id().ok_or(Error::CommandNotRunInGuild)?;
    let channel = ctx.channel_id();

    let current_timeout_mode = ctx.data().timeout_mode(guild, channel).await;

    let content = if let Some(new_timeout_mode) = timeout_mode {
        let content = if let Some(current_timeout_mode) = current_timeout_mode {
            format!(
                "Old timeout mode: {}\nNew timeout mode: {}",
                current_timeout_mode, new_timeout_mode
            )
        } else {
            format!(
                "Old timeout mode is not set.\nNew timeout mode: {}",
                new_timeout_mode
            )
        };

        ctx.data()
            .set_timeout_mode(guild, channel, new_timeout_mode)
            .await;

        content
    } else if let Some(current_timeout_mode) = current_timeout_mode {
        current_timeout_mode.to_string()
    } else {
        "Timeout mode is not set.\n".to_string()
    };

    send_reply(ctx, |f| f.content(content).ephemeral(true)).await?;

    Ok(())
}
