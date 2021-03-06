use poise::send_reply;

use crate::{Context, Error};

/// Gets or sets the tags for the channel
#[poise::command(
    prefix_command,
    slash_command,
    required_permissions = "MANAGE_CHANNELS"
)]
pub async fn tags(
    ctx: Context<'_>,
    #[description = "If provided, will set these as the new tags"] tags: Option<String>,
) -> Result<(), Error> {
    let guild = ctx.guild_id().ok_or(Error::CommandNotRunInGuild)?;
    let channel = ctx.channel_id();

    let current_tags = ctx.data().tags(guild, channel).await;

    let content = if let Some(new_tags) = tags {
        let new_tags: Vec<String> = new_tags
            .split_ascii_whitespace()
            .map(|s| s.to_string())
            .collect();

        let content = if let Some(current_tags) = current_tags {
            format!(
                "Old tags: {}\nNew tags: {}",
                current_tags.join(" "),
                &new_tags.join(" ")
            )
        } else {
            format!("Old tags are not set.\nNew tags: {}", new_tags.join(" "))
        };

        ctx.data().set_tags(guild, channel, new_tags).await;

        content
    } else if let Some(current_tags) = current_tags {
        current_tags.join(" ")
    } else {
        "Tags are not set.\n".to_string()
    };

    send_reply(ctx, |f| f.content(content).ephemeral(true)).await?;

    Ok(())
}
