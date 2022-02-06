use tracing::{instrument, warn};

mod checks;
mod commands;
mod configuration;
mod constants;
mod setup;
mod tasks;
mod utils;

type Data = setup::Data;
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
#[instrument]
async fn main() {
    tracing_subscriber::fmt().pretty().init();

    let framework = poise::Framework::build()
        .token(dotenv::var("DISCORD_BOT_TOKEN").unwrap())
        .user_data_setup(move |ctx, ready, framework| Box::pin(setup::setup(ctx, ready, framework)))
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
                commands::random_timeout::random_timeout(),
                commands::register::register_in_guild(),
                commands::register::register_globally(),
            ],
            command_check: Some(|ctx| Box::pin(async move {
                Ok(true)
            })),
            ..Default::default()
        })
        .build()
        .await
        .unwrap();

    let framework_stop_copy = framework.clone();
    tokio::spawn(async move {
        #[cfg(unix)]
        {
            use tokio::signal::unix as signal;

            let [mut s1, mut s2, mut s3] = [
                signal::signal(signal::SignalKind::hangup()).unwrap(),
                signal::signal(signal::SignalKind::interrupt()).unwrap(),
                signal::signal(signal::SignalKind::terminate()).unwrap(),
            ];

            tokio::select! {
                v = s1.recv() => v.unwrap(),
                v = s2.recv() => v.unwrap(),
                v = s3.recv() => v.unwrap(),
            };
        }
        #[cfg(windows)]
        {
            use tokio::signal::windows as signal;
            let (mut s1, mut s2) = (signal::ctrl_c().unwrap(), signal::ctrl_break().unwrap());

            tokio::select! {
                v = s1.recv() => v.unwrap(),
                v = s2.recv() => v.unwrap(),
            };
        }

        warn!("Shutting down");

        framework_stop_copy.user_data().await.stop_all();

        framework_stop_copy
            .shard_manager()
            .lock()
            .await
            .shutdown_all()
            .await;
    });

    warn!("Starting up");
    framework.start_autosharded().await.unwrap();
}
