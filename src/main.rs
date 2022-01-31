mod commands;
mod setup;

type Data = setup::Data;
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() {
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
                commands::timeout::timeout(),
            ],
            ..Default::default()
        })
        .run()
        .await
        .unwrap();
}
