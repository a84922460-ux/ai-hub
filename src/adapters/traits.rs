use crate::storage::models::Message;
use std::error::Error;

pub trait ModelAdapter: Send + Sync {
    fn model_id(&self) -> &'static str;
    async fn chat(&self, messages: &[Message], api_key: &str) -> Result<String, Box<dyn Error + Send + Sync>>;
}
