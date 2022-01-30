use poise::send_reply;

use crate::{Context, Error};

/// Display your or another user's account creation date
#[poise::command(
    prefix_command,
    slash_command,
    required_permissions = "MANAGE_CHANNELS"
)]
pub async fn stop(ctx: Context<'_>) -> Result<(), Error> {
    let guild = ctx.guild_id().ok_or("Command must be run in server")?;

    send_reply(ctx, |f| {
        let content = "This server will no longer receive pokemon.";
        f.content(content).ephemeral(true)
    })
    .await?;

    ctx.data().remove(guild).await;

    Ok(())
}
