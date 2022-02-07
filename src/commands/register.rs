//! The commands in this module do not require special persmission
//! checks, because the [poise::builtins::register_application_commands] function
//! does it for us.

use poise::send_reply;

use crate::{Context, Error};

/// Register the commands in the current guild
#[poise::command(prefix_command)]
pub async fn register_in_guild(ctx: Context<'_>) -> Result<(), Error> {
    poise::builtins::register_application_commands(ctx, false).await?;

    send_reply(ctx, |f| {
        f.content("Commands registered in guild").ephemeral(true)
    })
    .await?;

    Ok(())
}

/// Register the commands globally
#[poise::command(prefix_command, owners_only)]
pub async fn register_globally(ctx: Context<'_>) -> Result<(), Error> {
    poise::builtins::register_application_commands(ctx, true).await?;

    send_reply(ctx, |f| {
        f.content("Commands registered globally").ephemeral(true)
    })
    .await?;

    Ok(())
}
