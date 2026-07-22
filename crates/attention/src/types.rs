use serde::{Deserialize, Serialize};

/// Input supplied to the Attention Engine.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct AttentionInput {
    pub novelty: f32,
    pub importance: f32,
    pub urgency: f32,
}

impl AttentionInput {
    pub fn new(
        novelty: f32,
        importance: f32,
        urgency: f32,
    ) -> Self {
        Self {
            novelty,
            importance,
            urgency,
        }
    }
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Default,
    Serialize,
    Deserialize,
)]
pub enum Priority {
    #[default]
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct AttentionResult {
    pub score: f32,
    pub priority: Priority,
    pub accepted: bool,
}

impl AttentionResult {
    pub fn new(
        score: f32,
        priority: Priority,
        accepted: bool,
    ) -> Self {
        Self {
            score,
            priority,
            accepted,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct AttentionConfig {
    pub novelty_weight: f32,
    pub importance_weight: f32,
    pub urgency_weight: f32,
    pub acceptance_threshold: f32,
}

impl Default for AttentionConfig {
    fn default() -> Self {
        Self {
            novelty_weight: 0.30,
            importance_weight: 0.40,
            urgency_weight: 0.30,
            acceptance_threshold: 0.50,
        }
    }
}