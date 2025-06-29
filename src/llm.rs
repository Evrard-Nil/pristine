use crate::config::Config;
use crate::monitoring::Monitor;
use anyhow::{Result, anyhow};
use openai::{
    Credentials,
    chat::{ChatCompletion, ChatCompletionMessage, ChatCompletionMessageRole},
};
use std::sync::Arc;
use tokio::time::{Duration, sleep};

pub struct LlmClient {
    model_name: String,
    api_key_present: bool,
    credentials: Credentials,
    monitor: Option<Arc<Monitor>>,
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
            monitor: None,
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

    pub fn set_monitor(&mut self, monitor: Arc<Monitor>) {
        self.monitor = Some(monitor);
    }

    pub async fn generate_text(&self, system_prompt: &str, user_prompt: &str) -> Result<String> {
        let start_time = std::time::Instant::now();
        println!(
            "===== SYSTEM PROMPT =====\n{}\n===== USER PROMPT =====\n{}",
            system_prompt, user_prompt
        );

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

        let mut attempts = 0;
        let mut delay = Duration::from_secs(4);
        let result;

        loop {
            attempts += 1;
            let call_result = self.call_llm(messages.clone()).await;

            if call_result.is_ok() {
                result = call_result;
                break;
            }

            if attempts >= 5 {
                result = call_result; // Return the last error
                break;
            }

            eprintln!(
                "LLM call failed (attempt {}/5). Retrying in {:?}. Error: {:?}",
                attempts, delay, call_result
            );
            sleep(delay).await;
            delay *= 2;
        }

        let duration_ms = start_time.elapsed().as_millis() as u64;

        // Log the LLM call if monitor is available
        if let Some(monitor) = &self.monitor {
            if let Ok(ref response) = result {
                monitor.log_llm_call(
                    system_prompt.to_string(),
                    user_prompt.to_string(),
                    response.clone(),
                    duration_ms,
                    self.model_name.clone(),
                );
            }
        }

        result
    }
}
