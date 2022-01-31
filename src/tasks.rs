use std::{sync::Arc, time::Duration};

use poise::{
    futures_util::future::join_all,
    serenity_prelude::{CreateEmbed, UserId},
};
use rs621::client::Client;

use crate::Data;

pub async fn poke_loop(discord_http: Arc<poise::serenity_prelude::Http>, data: Data) -> ! {
    let channels = data.channels();
    println!("Waiting 20 seconds before first run.");
    tokio::time::sleep(Duration::from_secs(20)).await;
    let e6client = Client::new("https://e926.net", "CutePokebot/0.1.0 (norom)").unwrap();
    loop {
        let post = {
            let tags: &Vec<String> = &*data.tags().read_owned().await;
            e6client.search_random_post(&tags[..]).await
        };

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
                println!("Posting {:?}", post.id);
                let channels = channels.read().await.clone();
                let discord_http = discord_http.clone();

                tokio::spawn(async move {
                    let embed = CreateEmbed::default()
                        .colour(0x203f6c_u32)
                        .title(format!("#{}", post.id))
                        .description(post.description)
                        .url(&post.file.url.as_ref().unwrap())
                        .image(post.file.url.unwrap())
                        .field(
                            "Artist(s)",
                            format!("{}", post.tags.artist.join(", ")),
                            false,
                        )
                        .footer(|footer| {
                            let score = format!(
                                "up: {}, down: {}, total: {}",
                                post.score.up,
                                post.score.down,
                                post.score.up + post.score.down,
                            );
                            footer.text(score)
                        })
                        .to_owned();

                    let mut channel_futures = Vec::with_capacity(channels.len());

                    for channel in channels.values() {
                        let fut = channel
                            .send_message(discord_http.clone(), |f| f.set_embed(embed.clone()));
                        channel_futures.push(fut);
                    }

                    join_all(channel_futures).await;
                });
            }
        }

        let timeout_minutes = data.timeout();
        tokio::time::sleep(Duration::from_secs(timeout_minutes * 60)).await;
    }
}
