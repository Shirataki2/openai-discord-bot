use poise::BoxFuture;

use crate::{serenity, Data, Error};

pub fn handle_event<'a>(
    ctx: &'a serenity::Context,
    event: &'a poise::Event<'a>,
    _framework: poise::FrameworkContext<'a, Data, Error>,
    data: &'a Data,
) -> BoxFuture<'a, Result<(), Error>> {
    Box::pin(async move {
        use poise::Event::*;
        #[allow(clippy::single_match)]
        match event {
            Message { new_message: msg } => {
                if msg.author.bot {
                    return Ok(());
                }
                if msg.channel_id != data.discord_channel_id {
                    return Ok(());
                }
                println!("Message: {:?}", &msg.content);
                let client = &data.client;
                let typing = msg.channel_id.start_typing(&ctx.http);
                let resp = client.completion(&msg.content, data).await?;
                println!("Response: {:?}", &resp);
                if let Some(message) = resp {
                    msg.channel_id.say(&ctx.http, message).await?;
                } else {
                    msg.channel_id.say(&ctx.http, "No response").await?;
                }
                if let Err(e) = typing.map(|t| t.stop()) {
                    error!("Error while stopping typing: {}", e);
                }
            }
            _ => {}
        }
        Ok(())
    })
}
