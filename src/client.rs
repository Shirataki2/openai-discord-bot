use openai::{
    chat::{ChatCompletion, ChatCompletionMessage, ChatCompletionMessageRole},
    models::ModelID,
};

use crate::Data;

#[derive(Debug, Clone)]
pub struct Client {}

impl Client {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn single_completion(
        &self,
        prompt: &str,
    ) -> Result<Option<String>, anyhow::Error> {
        let mut messages = vec![];
        let user_message = ChatCompletionMessage {
            content: prompt.to_string(),
            role: ChatCompletionMessageRole::User,
            name: None,
        };
        messages.push(user_message.clone());

        let chat_completion = ChatCompletion::builder(ModelID::Gpt3_5Turbo, messages)
            .create()
            .await??;
        let return_message = chat_completion.choices.first();
        if let Some(return_message) = return_message {
            let return_message = return_message.message.clone();
            Ok(Some(return_message.content))
        } else {
            Ok(None)
        }
    }

    pub async fn completion(
        &self,
        prompt: &str,
        data: &Data,
    ) -> Result<Option<String>, anyhow::Error> {
        let system = {
            let system = data.system.lock().await;
            system.clone()
        };
        let mut messages = vec![];
        if let Some(system) = system {
            let message = ChatCompletionMessage {
                content: system,
                role: ChatCompletionMessageRole::System,
                name: None,
            };
            messages.push(message);
        }
        {
            let old_messages = data.messages.lock().await;
            for message in old_messages.iter() {
                messages.push(message.clone());
            }
        }
        let user_message = ChatCompletionMessage {
            content: prompt.to_string(),
            role: ChatCompletionMessageRole::User,
            name: None,
        };
        messages.push(user_message.clone());

        let chat_completion = ChatCompletion::builder(ModelID::Gpt3_5Turbo, messages)
            .create()
            .await??;
        let return_message = chat_completion.choices.first();
        if let Some(return_message) = return_message {
            let return_message = return_message.message.clone();
            let mut messages = data.messages.lock().await;
            messages.push(user_message.clone());
            messages.push(return_message.clone());
            Ok(Some(return_message.content))
        } else {
            Ok(None)
        }
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}
