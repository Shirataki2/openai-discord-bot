use std::sync::Arc;

use ai_helper_bot::{
    client::Client, commands::list_commands, handler::handle_event, on_error, serenity::*, Data,
};
use poise::{Framework, FrameworkOptions};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    pretty_env_logger::init();
    std::env::set_var("RUST_BACKTRACE", "1");

    let discord_bot_token = std::env::var("DISCORD_BOT_TOKEN")?;
    let openai_api_key = std::env::var("OPENAI_KEY")?;
    let discord_channel_id = std::env::var("DISCORD_CHANNEL_ID")?.parse::<u64>()?;

    let framework = Framework::builder()
        .options(FrameworkOptions {
            commands: list_commands(),
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("ai!".to_owned()),
                ..Default::default()
            },
            on_error: |e| Box::pin(on_error(e)),
            event_handler: handle_event,
            ..Default::default()
        })
        .token(discord_bot_token.clone())
        .intents(GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT)
        .setup(move |_ctx, _ready, _framework| {
            Box::pin(async move {
                Ok(Data {
                    discord_bot_token,
                    openai_api_key: openai_api_key.clone(),
                    discord_channel_id: ChannelId(discord_channel_id),
                    system: Arc::new(Mutex::new(None)),
                    client: Client::new(),
                    messages: Arc::new(Mutex::new(Vec::new())),
                })
            })
        });

    framework.run().await?;
    Ok(())
}
