use poise::send_reply;

use crate::{Context, Error};

/// Gets or sets the timeout
#[poise::command(prefix_command, slash_command, owners_only)]
pub async fn tags(
    ctx: Context<'_>,
    #[description = "If provided, will set these as the new tags"] tags: Option<String>,
) -> Result<(), Error> {
    let current_tags_lock = ctx.data().tags();
    let content = if let Some(new_tags) = tags {
        let mut current_tags = current_tags_lock.write().await;
        let content = format!(
            "Old tags: {}\nNew tags: {}",
            current_tags.join(" "),
            new_tags
        );

        *current_tags = new_tags
            .split_ascii_whitespace()
            .map(|s| s.to_string())
            .collect();
        content
    } else {
        let current_tags = current_tags_lock.read().await;
        current_tags.join(" ")
    };

    send_reply(ctx, |f| f.content(content).ephemeral(true)).await?;

    Ok(())
}
