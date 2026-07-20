//! Core types used by the Attention module.

use serde::{Deserialize, Serialize};

/// Represents the importance assigned to an observation.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
)]
pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

impl Default for Priority {
    fn default() -> Self {
        Self::Low
    }
}

/// Configuration for the attention engine.
#[derive(Debug, Clone, Serialize, Deserialize)]
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

/// Result produced after evaluating an observation.
#[derive(Debug, Clone, Serialize, Deserialize)]
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