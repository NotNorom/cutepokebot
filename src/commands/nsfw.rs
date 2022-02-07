use poise::send_reply;

use crate::{utils::NsfwMode, Context, Error};

/// Gets or sets if the channel is nsfw
#[poise::command(
    prefix_command,
    slash_command,
    required_permissions = "MANAGE_CHANNELS"
)]
pub async fn nsfw(
    ctx: Context<'_>,
    #[description = "Nsfw mode"] nsfw: Option<NsfwMode>,
) -> Result<(), Error> {
    let guild = ctx.guild_id().ok_or("Command must be run in server")?;
    let channel =ctx.channel_id();

    let current_nsfw_mode = ctx.data().nsfw_mode(guild, channel).await;

    let content = if let Some(new_nsfw_mode) = nsfw {
        let content = if let Some(current_nsfw_mode) = current_nsfw_mode {
            format!(
                "Old nsfw mode: {}\nNew nsfw mode: {}",
                current_nsfw_mode, new_nsfw_mode
            )
        } else {
            format!(
                "Old nsfw mode is not set.\nNew nsfw mode: {}",
                new_nsfw_mode
            )
        };

        ctx.data()
            .set_nsfw_mode(guild, channel, new_nsfw_mode)
            .await;

        content
    } else if let Some(current_nsfw_mode) = current_nsfw_mode {
        current_nsfw_mode.to_string()
    } else {
        "Nsfw mode is not set.\n".to_string()
    };

    send_reply(ctx, |f| f.content(content).ephemeral(true)).await?;

    Ok(())
}
