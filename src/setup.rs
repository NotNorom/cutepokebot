use std::{collections::HashMap, sync::Arc, time::Duration};

use poise::{
    serenity_prelude::{ChannelId, Context, CreateEmbed, GuildId, Ready, RwLock, UserId},
    Framework,
};

pub struct Data {
    channels: Arc<RwLock<HashMap<GuildId, ChannelId>>>,
}

impl Data {
    fn new() -> Self {
        Self {
            channels: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn add(&self, guild: GuildId, channel: ChannelId) {
        let mut channels = self.channels.write().await;
        channels.insert(guild, channel);
    }

    pub async fn remove(&self, guild: GuildId) {
        let mut channels = self.channels.write().await;
        channels.remove(&guild);
    }

    /// Get a reference to the data's channels.
    pub fn channels(&self) -> Arc<RwLock<HashMap<GuildId, ChannelId>>> {
        self.channels.clone()
    }
}

pub async fn setup<U, E>(
    context: &Context,
    _ready: &Ready,
    _framework: &Framework<U, E>,
) -> Result<crate::Data, crate::Error> {
    let data = Data::new();
    let channels = data.channels();

    let discord_http = context.http.clone();

    tokio::spawn(async move {
        let e6client =
            rs621::client::Client::new("https://e926.net", "CutePokebot/0.1.0 (norom)").unwrap();
        loop {
            let channels = channels.read().await;
            {
                let post = e6client
                    .search_random_post(
                        &[
                            "pokémon_(species)",
                            "-vore",
                            "-gore",
                            "-transformation",
                            "-comic",
                            "score:>55",
                        ][..],
                    )
                    .await;
                match post {
                    Err(err) => {
                        let channel = UserId(160518747713437696)
                            .create_dm_channel(&discord_http)
                            .await
                            .unwrap();
                        let _ = channel
                            .say(&discord_http, format!("Error: ```{:#?}```", err))
                            .await;
                    }
                    Ok(post) => {
                        let embed = CreateEmbed::default()
                            .title("title")
                            .image(&post.preview.url.unwrap())
                            .to_owned();
                        for channel in channels.values() {
                            let _ = channel
                                .send_message(&discord_http, |f| f.set_embed(embed.clone()))
                                .await;
                        }
                    }
                }
            }
            // 20 * 60s = 1200s
            tokio::time::sleep(Duration::from_secs(1200)).await;
        }
    });

    Ok(data)
}
