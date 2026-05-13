use crate::storage::models::UpgradeProposal;
use std::collections::HashMap;

/// 简单的自诊断函数：根据传入的错误率判断是否需要生成提案
pub fn diagnose(error_counts: &HashMap<String, u64>, threshold: u64) -> Vec<UpgradeProposal> {
    let mut proposals = Vec::new();
    for (model_id, count) in error_counts {
        if *count >= threshold {
            proposals.push(UpgradeProposal {
                id: format!("proposal-{}", uuid::Uuid::new_v4()),
                proposal_type: "routing".into(),
                target_file: "scripts/routing_strategy.lua".into(),
                diff_content: format!("-- 降低模型 {} 的优先级 (错误次数: {})", model_id, count),
                status: "Pending".into(),
            });
        }
    }
    proposals
}
