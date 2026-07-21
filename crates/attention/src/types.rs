//! Core data types for the Attention subsystem.

use serde::{Deserialize, Serialize};

/// Priority assigned to an observation after evaluation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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
    /// Weight assigned to novelty.
    pub novelty_weight: f32,

    /// Weight assigned to importance.
    pub importance_weight: f32,

    /// Weight assigned to urgency.
    pub urgency_weight: f32,

    /// Minimum score required to pass attention.
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

/// Raw attention metrics produced by the analyzer.
///
/// These are the inputs to the AttentionEngine.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct AttentionMetrics {
    pub novelty: f32,
    pub importance: f32,
    pub urgency: f32,
}

impl AttentionMetrics {
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

/// Final result produced by the AttentionEngine.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttentionResult {
    /// Final weighted score.
    pub score: f32,

    /// Assigned cognitive priority.
    pub priority: Priority,

    /// Whether the observation passed the attention filter.
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