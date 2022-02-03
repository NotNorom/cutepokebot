use poise::send_reply;

use crate::{Context, Error};

/// Gets or sets the global timeout
#[poise::command(prefix_command, slash_command, owners_only)]
pub async fn timeout(
    ctx: Context<'_>,
    #[description = "Timeout in minutes"] timeout: Option<u64>,
) -> Result<(), Error> {
    let current_timeout = ctx.data().timeout();

    let content = if let Some(new_timeout) = timeout {
        ctx.data().set_timeout(new_timeout);
        format!(
            "Old timeout: {}, new timeout: {}",
            current_timeout, new_timeout
        )
    } else {
        format!("Current timeout: {}", current_timeout)
    };

    send_reply(ctx, |f| f.content(content).ephemeral(true)).await?;

    Ok(())
}
