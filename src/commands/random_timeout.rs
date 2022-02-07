use poise::send_reply;

use crate::{Context, Error};

/// Gets or sets if the timeout should be random
#[poise::command(
    prefix_command,
    slash_command,
    required_permissions = "MANAGE_CHANNELS"
)]
pub async fn random_timeout(
    ctx: Context<'_>,
    #[description = "Random timeout"] random_timeout: Option<bool>,
) -> Result<(), Error> {
    let guild = ctx.guild_id().ok_or("Command must be run in server")?;
    let channel = ctx.channel_id();

    let current_random_timeout = ctx.data().random_timeout(guild, channel).await;

    let content = if let Some(new_random_timeout) = random_timeout {
        let content = if let Some(current_random_timeout) = current_random_timeout {
            format!(
                "Old random timeout: {}\nNew random timeout: {}",
                current_random_timeout, new_random_timeout
            )
        } else {
            format!(
                "Old random timeout is not set.\nNew random timeout: {}",
                new_random_timeout
            )
        };

        ctx.data()
            .set_random_timeout(guild, channel, new_random_timeout)
            .await;

        content
    } else if let Some(current_random_timeout) = current_random_timeout {
        current_random_timeout.to_string()
    } else {
        "Random timeout is not set.\n".to_string()
    };

    send_reply(ctx, |f| f.content(content).ephemeral(true)).await?;

    Ok(())
}
