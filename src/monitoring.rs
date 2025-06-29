use crate::actions::Actions;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionLog {
    pub timestamp: DateTime<Utc>,
    pub action: Actions,
    pub result: String,
    pub duration_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmCallLog {
    pub timestamp: DateTime<Utc>,
    pub system_prompt: String,
    pub user_prompt: String,
    pub response: String,
    pub duration_ms: u64,
    pub model: String,
}

#[derive(Clone)]
pub struct Monitor {
    action_logs: Arc<Mutex<Vec<ActionLog>>>,
    llm_call_logs: Arc<Mutex<Vec<LlmCallLog>>>,
}

impl Monitor {
    pub fn new() -> Self {
        Self {
            action_logs: Arc::new(Mutex::new(Vec::new())),
            llm_call_logs: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn log_action(&self, action: Actions, result: String, duration_ms: u64) {
        let log = ActionLog {
            timestamp: Utc::now(),
            action,
            result,
            duration_ms,
        };

        if let Ok(mut logs) = self.action_logs.lock() {
            logs.push(log);
            // Keep only last 1000 entries to prevent memory issues
            if logs.len() > 1000 {
                let drain_count = logs.len() - 1000;
                logs.drain(0..drain_count);
            }
        }
    }

    pub fn log_llm_call(
        &self,
        system_prompt: String,
        user_prompt: String,
        response: String,
        duration_ms: u64,
        model: String,
    ) {
        let log = LlmCallLog {
            timestamp: Utc::now(),
            system_prompt,
            user_prompt,
            response,
            duration_ms,
            model,
        };

        if let Ok(mut logs) = self.llm_call_logs.lock() {
            logs.push(log);
            // Keep only last 1000 entries to prevent memory issues
            if logs.len() > 1000 {
                let drain_count = logs.len() - 1000;
                logs.drain(0..drain_count);
            }
        }
    }

    pub fn get_action_logs(&self) -> Vec<ActionLog> {
        self.action_logs
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .clone()
    }

    pub fn get_llm_call_logs(&self) -> Vec<LlmCallLog> {
        self.llm_call_logs
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .clone()
    }
}
