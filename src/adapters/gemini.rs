use super::traits::ModelAdapter;
use crate::storage::models::Message;
use reqwest::Client;
use serde_json::json;
use std::error::Error;

pub struct GeminiAdapter;

impl GeminiAdapter {
    pub fn new() -> Self { Self }
}

#[async_trait::async_trait]
impl ModelAdapter for GeminiAdapter {
    fn model_id(&self) -> &'static str { "gemini-1.5-flash" }

    async fn chat(&self, messages: &[Message], api_key: &str) -> Result<String, Box<dyn Error + Send + Sync>> {
        let client = Client::new();
        let contents: Vec<_> = messages.iter().map(|m| {
            json!({
                "role": if m.role == "assistant" { "model" } else { "user" },
                "parts": [{"text": &m.content}]
            })
        }).collect();

        let resp = client.post(format!(
            "https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash:generateContent?key={}",
            api_key
        ))
        .json(&json!({ "contents": contents }))
        .send()
        .await?;

        let body: serde_json::Value = resp.json().await?;
        let text = body["candidates"][0]["content"]["parts"][0]["text"]
            .as_str()
            .unwrap_or("无回复")
            .to_string();
        Ok(text)
    }
}
