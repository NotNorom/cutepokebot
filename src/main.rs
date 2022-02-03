use tracing::instrument;

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

    poise::Framework::build()
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
            ],
            ..Default::default()
        })
        .run()
        .await
        .unwrap();
}
