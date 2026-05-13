use crate::storage::models::Message;
use crate::adapters::traits::ModelAdapter;
use std::sync::Arc;

pub struct Router {
    adapters: Vec<Arc<dyn ModelAdapter>>,
}

impl Router {
    pub fn new(adapters: Vec<Arc<dyn ModelAdapter>>) -> Self {
        Self { adapters }
    }

    pub async fn route_best(
        &self,
        messages: &[Message],
        api_keys: &std::collections::HashMap<String, String>,
    ) -> Result<String, String> {
        // 简化版：直接用第一个可用的适配器，真实产品会整合 metrics
        for adapter in &self.adapters {
            let key = api_keys.get(adapter.model_id()).ok_or("未找到API Key")?;
            match adapter.chat(messages, key).await {
                Ok(reply) => return Ok(reply),
                Err(e) => eprintln!("模型 {} 调用失败: {}", adapter.model_id(), e),
            }
        }
        Err("所有模型调用失败".into())
    }
}
