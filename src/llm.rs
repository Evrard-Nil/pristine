use crate::config::Config;
use anyhow::{Result, anyhow};
use octocrab::models::issues::{Comment, Issue};
use openai::{
    Credentials,
    chat::{ChatCompletion, ChatCompletionMessage, ChatCompletionMessageRole},
};
use serde::{Deserialize, Serialize};

pub struct LlmClient {
    model_name: String,
    api_key_present: bool,
    credentials: Credentials,
}

impl LlmClient {
    pub fn new(config: &Config) -> Result<Self> {
        let api_key = config.openai_api_key.clone();
        let base_url = config.openai_api_base.clone();

        let credentials = if api_key.is_empty() {
            Credentials::from_env()
        } else {
            Credentials::new(api_key, base_url)
        };

        Ok(Self {
            model_name: config.openai_api_model.clone(),
            api_key_present: !config.openai_api_key.is_empty()
                || std::env::var("OPENAI_KEY").is_ok(),
            credentials,
        })
    }

    async fn call_llm(&self, messages: Vec<ChatCompletionMessage>) -> Result<String> {
        if !self.api_key_present {
            return Err(anyhow!(
                "OpenAI API key not configured. LLM functionality disabled."
            ));
        }

        let chat_completion = ChatCompletion::builder(&self.model_name, messages)
            .credentials(self.credentials.clone())
            .create()
            .await
            .map_err(|e| anyhow!("Failed to create chat completion: {:?}", e))?;

        let returned_message = chat_completion
            .choices
            .first()
            .ok_or_else(|| anyhow!("No message returned from LLM"))?
            .message
            .clone();

        let content = returned_message
            .content
            .ok_or_else(|| anyhow!("LLM returned empty content"));

        // Sanitize the content to remove any leading/trailing whitespace
        let content = content?.trim().to_string();
        let content = content.replace("```json", "");
        let content = content.replace("```", ""); // Remove any remaining code block markers

        Ok(content)
    }

    pub async fn generate_text(&self, system_prompt: &str, user_prompt: &str) -> Result<String> {
        let messages = vec![
            ChatCompletionMessage {
                role: ChatCompletionMessageRole::System,
                content: Some(system_prompt.to_string()),
                name: None,
                function_call: None,
                tool_call_id: None,
                tool_calls: None,
            },
            ChatCompletionMessage {
                role: ChatCompletionMessageRole::User,
                content: Some(user_prompt.to_string()),
                name: None,
                function_call: None,
                tool_call_id: None,
                tool_calls: None,
            },
        ];
        self.call_llm(messages).await
    }
}
