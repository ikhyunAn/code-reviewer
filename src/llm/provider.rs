use anyhow::Result;
use async_trait::async_trait;
use futures_core::Stream;
use serde::{Deserialize, Serialize};
use std::pin::Pin;

// Alias for LLM streaming responses
pub type LlmStream = Pin<Box<dyn Stream<Item = Result<LlmStreamDelta>> + Send>>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Role {
    System,
    User,
    Assistant,
    Tool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: Role,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    pub temperature: Option<f32>,
    pub top_p: Option<f32>,
    pub max_tokens: Option<u32>,
    pub metadata: Option<serde_json::Value>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenUsage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmResponse {
    pub model: String,
    pub content: String,
    pub finish_reason: Option<String>,
    pub usage: Option<TokenUsage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmStreamDelta {
    pub delta: String,
    pub done: bool,
}

#[async_trait]
pub trait LlmProvider: Send + Sync {
    fn provider_id(&self) -> &'static str;
    fn supports_model(&self, model:&str) -> bool;
    async fn complete(&self, req: LlmRequest) -> Result<LlmResponse>;
    async fn stream(&self, req: LlmRequest) -> Result<LlmStream>;
}