use poise::serenity_prelude::{self, Color};

use crate::{Context, Data, Error};

#[poise::command(prefix_command, hide_in_help)]
pub async fn register(ctx: Context<'_>) -> Result<(), Error> {
    poise::builtins::register_application_commands_buttons(ctx).await?;
    Ok(())
}

/// ChatGPTに前情報として人格などの情報を設定します。
#[poise::command(slash_command)]
pub async fn system(
    ctx: Context<'_>,
    #[description = "人格などの情報"] system: Option<String>,
) -> Result<(), Error> {
    let message = if let Some(ref system) = system {
        format!("システム情報を以下のように設定しました\n```\n{}```\n", system)
    } else {
        "システム情報をリセットしました".to_string()
    };
    let mut data = ctx.data().system.lock().await;
    *data = system;
    ctx.send(|m| m.content(message))
        .await?;
    Ok(())
}

#[derive(Debug, poise::Modal)]
#[allow(dead_code)] // fields only used for Debug print
pub struct Modal {
    #[name = "システム情報"]
    #[paragraph]
    pub system: Option<String>,
}

/// ChatGPTに前情報として人格などの情報をモーダルで設定します。
#[poise::command(slash_command)]
pub async fn system_modal(
    ctx: poise::ApplicationContext<'_, Data, Error>,
) -> Result<(), Error> {
    use poise::Modal as _;
    let modal = match Modal::execute(ctx).await? {
        Some(modal) => modal,
        None => return Ok(()),
    };
    let system = modal.system;
    let message = if let Some(ref system) = system {
        format!("システム情報を以下のように設定しました\n```\n{}```\n", system)
    } else {
        "システム情報をリセットしました".to_string()
    };
    let mut data = ctx.data().system.lock().await;
    *data = system;
    ctx.send(|m| m.content(message))
        .await?;
    Ok(())
}

/// 今までの会話情報をリセットします。
#[poise::command(slash_command)]
pub async fn reset(ctx: Context<'_>) -> Result<(), Error> {
    let mut data = ctx.data().messages.lock().await;
    *data = vec![];
    ctx.send(|m| m.content("チャットログをリセットしました"))
        .await?;
    Ok(())
}

#[poise::command(context_menu_command = "ChatGPTに質問")]
pub async fn ask(
    ctx: Context<'_>,
    msg: serenity_prelude::Message,
) -> Result<(), Error> {
    let content = &msg.content;
    let url = msg.link();
    let client = &ctx.data().client;
    ctx.defer().await?;
    let msg = client.single_completion(content).await?;
    if let Some(msg) = msg {
        ctx.send(
            |b| b.embed(|e| {
                e.color(Color::DARK_BLUE)
                    .description(
                        format!("[質問]({}):\n\n{}\n\n回答:\n\n{}\n", url, content, msg)
                    )
            })
        ).await?;
    }
    Ok(())
}

pub fn list_commands() -> Vec<poise::Command<Data, Error>> {
    vec![register(), system(), system_modal(), reset(), ask()]
}
