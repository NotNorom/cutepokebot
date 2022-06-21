use std::sync::Arc;

use poise::{serenity_prelude::GatewayIntents, Framework};
use tokio::sync::watch::{self, Receiver};
use tracing::{debug, error, info, instrument, warn};

mod checks;
mod commands;
mod configuration;
mod constants;
mod error;
mod persistence;
mod setup;
mod tasks;
mod utils;

type Data = setup::Data;
type Error = error::Error;
type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
#[instrument]
async fn main() {
    tracing_subscriber::fmt()
        .pretty()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let (shutdown_sender, shutdown_receiver) = watch::channel(false);

    let framework = poise::Framework::build()
        .token(dotenv::var("DISCORD_BOT_TOKEN").unwrap())
        .intents(GatewayIntents::non_privileged())
        .user_data_setup(move |ctx, ready, framework| {
            Box::pin(setup::setup(ctx, ready, framework, shutdown_sender))
        })
        .options(poise::FrameworkOptions {
            // configure framework here
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("~".into()),
                ..Default::default()
            },
            commands: vec![
                commands::start::start(),
                commands::stop::stop(),
                commands::tags::tags(),
                commands::nsfw::nsfw(),
                commands::timeout::timeout(),
                commands::timeout_mode::timeout_mode(),
                commands::register::register_in_guild(),
                commands::register::register_globally(),
                commands::shutdown::shutdown(),
            ],
            command_check: Some(|_ctx| Box::pin(async move { Ok(true) })),
            ..Default::default()
        })
        .build()
        .await
        .unwrap();

    tokio::spawn(stop_bot_handler(framework.clone(), shutdown_receiver));

    warn!("Starting up");
    if let Err(err) = framework.start_autosharded().await {
        error!("Error starting up: {err:?}");
    } else {
        info!("Startup successfull");
    }
}

/// This function waits for different kinds signals and the shutdown command, then shuts the bot down
///
/// Calling this function will block asynchronously
async fn stop_bot_handler(
    framework: Arc<Framework<Data, Error>>,
    mut shutdown_receiver: Receiver<bool>,
) {
    #[cfg(unix)]
    {
        use tokio::signal::unix as signal;

        let [mut s1, mut s2, mut s3] = [
            signal::signal(signal::SignalKind::hangup()).unwrap(),
            signal::signal(signal::SignalKind::interrupt()).unwrap(),
            signal::signal(signal::SignalKind::terminate()).unwrap(),
        ];

        tokio::select! {
            v = shutdown_receiver.changed() => {
                if v.is_err() {
                    error!("shutdown_receiver has been dropped. setup_user_data must have failed. unclean shutdown");
                    return
                }
                debug!("received signal: shutdown_receiver: {v:?}");
            },
            _ = s1.recv() => {debug!("received signal: hangup")},
            _ = s2.recv() => {debug!("received signal: interrupt")},
            _ = s3.recv() => {debug!("received signal: terminate")},
        };
    }
    #[cfg(windows)]
    {
        use tokio::signal::windows as signal;
        let (mut s1, mut s2) = (signal::ctrl_c().unwrap(), signal::ctrl_break().unwrap());

        tokio::select! {
            _ = shutdown_receiver.changed() => {debug!("received signal: shutdown_receiver")},
            _ = s1.recv() => debug!("received signal: ctrl_c"),
            _ = s2.recv() => debug!("received signal: ctrl_break"),
        };
    }

    warn!("Shutting down tasks");
    {
        let user_data = framework.user_data().await;
        debug!("Got user data, starting shutdown for guild tasks");
        user_data.stop_all();
        debug!("User data tasks stopped");
    }
    warn!("Shutting down shards");
    {
        framework.shard_manager().lock().await.shutdown_all().await;
    }
    warn!("Storing state to db");
    {
        let user_data = framework.user_data().await;
        debug!("Got user data, storing data");
        if let Err(err) = user_data.store_to_db().await {
            error!("Error storing data: {err:?}");
        } else {
            debug!("Data stored");
        }
    }
}
