use poise::{send_reply, serenity_prelude::ChannelId};

use crate::{Context, Error};

/// Gets or sets the timeout
#[poise::command(prefix_command, slash_command, owners_only)]
pub async fn tags(
    ctx: Context<'_>,
    #[description = "Selected channel"] channel: Option<ChannelId>,
    #[description = "If provided, will set these as the new tags"] tags: Option<String>,
) -> Result<(), Error> {
    let guild = ctx.guild_id().ok_or("Command must be run in server")?;
    let channel = channel.unwrap_or_else(|| ctx.channel_id());

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
    } else {
        if let Some(current_tags) = current_tags {
            current_tags.join(" ")
        } else {
            "Tags are not set.\n".to_string()
        }
    };

    send_reply(ctx, |f| f.content(content).ephemeral(true)).await?;

    Ok(())
}
