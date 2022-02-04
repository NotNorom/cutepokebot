use std::{
    collections::{HashMap, HashSet},
    time::Duration,
};

use crate::{
    constants::MINIMUM_TIMEOUT_MINUTES,
    utils::{embed_from_post, post_buttons, NsfwMode},
    Data,
};

use futures::stream::StreamExt;
use poise::serenity_prelude::{
    ChannelId, ComponentInteractionCollectorBuilder, Context, GuildId, MessageId, UserId,
};
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
                    if let Err(err) = channel
                        .send_message(&ctx, |m| {
                            m.set_embed(embed.clone())
                                .components(|c| c.add_action_row(post_buttons(0, 4)))
                        })
                        .await
                    {
                        error!("{}", err);
                    };
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

/// listens for delete button clicks on image posts
#[instrument(skip(ctx))]
pub async fn delete_button_listener(ctx: Context) {
    let mut collector = ComponentInteractionCollectorBuilder::new(&ctx)
        .filter(|interaction| interaction.data.custom_id == "delete-post")
        .await;

    let mut authors = HashMap::<MessageId, HashSet<UserId>>::new();
    while let Some(interaction) = collector.next().await {
        let authors_of_message = authors.entry(interaction.message.id).or_default();
        if authors_of_message.len() >= 4 {
            if let Err(err) = interaction
                .delete_original_interaction_response(&ctx.http)
                .await
            {
                error!("Error deleting original interaction response: {}", err)
            } else {
                info!("Deleted message in {}", interaction.channel_id);
            }
        }
        if authors_of_message.insert(interaction.user.id) {
            if let Err(err) = interaction
                .edit_original_interaction_response(&ctx.http, |msg| {
                    msg.components(|c| {
                        c.set_action_rows(vec![post_buttons(authors_of_message.len(), 4)])
                    })
                })
                .await
            {
                error!("{}", err);
            }
        }
    }
}
