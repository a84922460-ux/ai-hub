use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct ModelMetrics {
    pub model_id: String,
    pub avg_latency_ms: u64,
    pub success_rate: f32,
    pub quality_score: f32,
    pub quota_remaining: f32,
}

impl ModelMetrics {
    pub fn normalized_score(&self) -> f32 {
        let latency_factor = (1.0 / (1.0 + self.avg_latency_ms as f32 / 1000.0)).min(1.0);
        self.success_rate * 0.4 + self.quality_score * 0.3 + latency_factor * 0.2 + self.quota_remaining * 0.1
    }
}

pub fn select_best_model(metrics: &[ModelMetrics]) -> Option<ModelMetrics> {
    metrics
        .iter()
        .max_by(|a, b| a.normalized_score().partial_cmp(&b.normalized_score()).unwrap())
        .cloned()
}

pub fn select_top_n(metrics: &[ModelMetrics], n: usize) -> Vec<ModelMetrics> {
    let mut sorted = metrics.to_vec();
    sorted.sort_by(|a, b| b.normalized_score().partial_cmp(&a.normalized_score()).unwrap());
    sorted.into_iter().take(n).collect()
}
