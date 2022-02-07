use poise::send_reply;

use crate::{Context, Error};

/// Shuts down the bot (can only be used by bot owners)
#[poise::command(prefix_command, slash_command, owners_only)]
pub async fn shutdown(ctx: Context<'_>) -> Result<(), Error> {
    ctx.data().shutdown();

    send_reply(ctx, |f| f.content("Shutting down").ephemeral(true)).await?;

    Ok(())
}
