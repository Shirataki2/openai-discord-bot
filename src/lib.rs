#[macro_use]
extern crate log;

pub mod client;
pub mod commands;
pub mod handler;

pub type Error = anyhow::Error;
pub type Context<'a> = poise::Context<'a, Data, Error>;
use std::sync::Arc;

use openai::chat::ChatCompletionMessage;
pub use poise::serenity_prelude as serenity;
use poise::serenity_prelude::Mutex;

pub struct Data {
    pub discord_bot_token: String,
    pub openai_api_key: String,
    pub discord_channel_id: serenity::ChannelId,
    pub system: Arc<Mutex<Option<String>>>,
    pub messages: Arc<Mutex<Vec<ChatCompletionMessage>>>,
    pub client: client::Client,
}

pub async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
    match error {
        poise::FrameworkError::Setup { error, .. } => panic!("Failed to start bot: {:?}", error),
        poise::FrameworkError::Command { error, ctx } => {
            error!("Error in command `{}`: {:?}", ctx.command().name, error,);
        }
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                error!("Error while handling error: {}", e)
            }
        }
    }
}
