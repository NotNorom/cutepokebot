use poise::send_reply;

use crate::{Context, Error};

/// Gets or sets the timeout
#[poise::command(prefix_command, slash_command, owners_only)]
pub async fn tags(ctx: Context<'_>) -> Result<(), Error> {
    let current_tags_lock = ctx.data().tags();
    let current_tags = current_tags_lock.read().await;

    let content = current_tags.join(", ");

    send_reply(ctx, |f| f.content(content).ephemeral(true)).await?;

    Ok(())
}
