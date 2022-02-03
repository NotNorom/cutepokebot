use std::time::Duration;

use poise::serenity_prelude::{ChannelId, GuildId, UserId};

use crate::{
    utils::{embed_from_post, post_buttons},
    Data,
};

/// Starts the loop for a channel in a guild
pub async fn poke_loop(data: Data, guild: GuildId, channel: ChannelId) {
    let discord_http = data.context().http.clone();

    loop {
        if data.nsfw_mode(guild, channel).await.is_none() {
            break;
        }

        let post = data.get_post(guild, channel).await;

        match post {
            None => {
                let channel = UserId(160518747713437696)
                    .create_dm_channel(&discord_http)
                    .await
                    .unwrap();
                let _ = channel
                    .say(
                        &discord_http,
                        format!("Error: ```Guild: {}, Channel: {}```", guild, channel),
                    )
                    .await;
            }
            Some(post) => {
                println!(
                    "Posting {:?} in guild {} in channel {}",
                    post.id, guild, channel
                );
                let embed = embed_from_post(&post).expect("Embed creation shall not fail!");
                let ctx = data.context().clone();
                tokio::spawn(async move {
                    let message = channel
                        .send_message(&ctx, |m| {
                            m.set_embed(embed.clone())
                                .components(|c| c.add_action_row(post_buttons()))
                        })
                        .await;

                    // TODO: Figure out how to do component interactions correctly
                    let _res = match message
                        .unwrap()
                        .await_component_interaction(&ctx)
                        .timeout(Duration::from_secs(40 * 60))
                        .await
                    {
                        Some(ci) => ci,
                        None => return,
                    };
                });
            }
        }

        let timeout_minutes = data.timeout();
        tokio::time::sleep(Duration::from_secs(timeout_minutes * 60)).await;
    }
}
