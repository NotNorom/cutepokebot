use std::{collections::HashSet, sync::Arc, time::Duration};

use crate::{
    constants::{MAXIMUM_INETERACTION_TIME, MINIMUM_TIMEOUT_MINUTES},
    utils::{embed_from_post, post_buttons, NsfwMode},
    Data,
};
use futures::stream::StreamExt;
use poise::serenity_prelude::{ChannelId, GuildId, InteractionResponseType, RwLock, UserId};
use rand::Rng;
use tracing::{error, info, instrument};

/// Starts the loop for a channel in a guild
#[instrument(skip(data))]
pub async fn poke_loop(data: Data, guild: GuildId, channel: ChannelId) {
    let discord_http = data.context().http.clone();

    while data.config_available(guild, channel).await {
        let post = data.get_post(guild, channel).await;

        match post {
            None => {
                let tags = data.tags(guild, channel).await;
                let nsfw_mode = data.nsfw_mode(guild, channel).await;
                error!(
                    "There is no post to send to guild: {}, channel: {}, nsfw: {:?}, tags: {:?}",
                    guild, channel, nsfw_mode, tags
                );
                let content = if let Some(NsfwMode::NSFW) = nsfw_mode {
                    "There is no post matching the tags."
                } else {
                    "There is no post matching the tags. Nsfw-mode is set to sfw, try setting nsfw-mode to nsfw."
                };

                let _ = channel.say(&discord_http, content).await;
            }
            Some(post) => {
                info!(
                    "Posting {:?} in guild {} in channel {}",
                    post.id, guild, channel
                );
                let embed = match embed_from_post(&post) {
                    Ok(embed) => embed,
                    Err(err) => {
                        let error_message = format!("Error: {}. Stopping for this channel", err);
                        error!("{}", &error_message);
                        let _ = channel.say(&discord_http, error_message).await;
                        break;
                    }
                };
                let ctx = data.context().clone();
                tokio::spawn(async move {
                    let mut message = channel
                        .send_message(&ctx, |m| {
                            m.set_embed(embed.clone())
                                .components(|c| c.add_action_row(post_buttons(0, 4)))
                        })
                        .await
                        .unwrap();

                    info!("Sent message. Waiting for interactions");

                    let interaction_authors = Arc::new(RwLock::new(HashSet::<UserId>::new()));

                    let mut collector = message
                        .await_component_interactions(&ctx)
                        .timeout(Duration::from_secs(MAXIMUM_INETERACTION_TIME * 60))
                        .await;

                    while let Some(interaction) = collector.next().await {
                        info!("Received interaction: {:?}", interaction);
                        let arc = interaction_authors.clone();
                        let mut authors = arc.write().await;
                        if authors.len() >= 4 {
                            let _ = interaction.delete_original_interaction_response(&ctx).await;
                            info!(
                                "Original message is deleted. No longer waiting for interactions"
                            );
                            break;
                        }
                        authors.insert(interaction.user.id);

                        let _ = interaction
                            .create_interaction_response(&ctx, |response| {
                                response
                                    .kind(InteractionResponseType::UpdateMessage)
                                    .interaction_response_data(|c| {
                                        c.components(|com| {
                                            com.set_action_rows(vec![post_buttons(
                                                authors.len(),
                                                4,
                                            )])
                                        })
                                    })
                            })
                            .await;
                        info!("Interaction response sent");
                    }
                    let _ = message.edit(&ctx, |msg| msg.components(|c| c)).await;
                });
            }
        }

        let timeout_minutes = data.timeout(guild, channel).await.unwrap_or(40);

        let sleep_duration = if data.random_timeout(guild, channel).await.unwrap_or(false) {
            let lower_limit = MINIMUM_TIMEOUT_MINUTES;
            let upper_limit = timeout_minutes;

            let mut rng = rand::thread_rng();
            rng.gen_range(lower_limit..=upper_limit)
        } else {
            timeout_minutes
        };

        info!("Waiting for {} minutes for the next post", sleep_duration);

        tokio::time::sleep(Duration::from_secs(sleep_duration * 60)).await;
    }
}
