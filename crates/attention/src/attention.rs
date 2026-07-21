use crate::{
    filter::accept,
    priority::determine_priority,
    score::calculate_score,
    types::{AttentionConfig, AttentionResult},
};

/// Cognitive attention engine.
///
/// Responsible for evaluating whether an observation deserves
/// further processing based on weighted attention factors.
#[derive(Debug, Clone)]
pub struct AttentionEngine {
    config: AttentionConfig,
}

impl AttentionEngine {
    /// Creates a new attention engine.
    pub fn new(config: AttentionConfig) -> Self {
        Self { config }
    }

    /// Returns the current engine configuration.
    pub fn config(&self) -> &AttentionConfig {
        &self.config
    }

    /// Replaces the engine configuration.
    pub fn set_config(&mut self, config: AttentionConfig) {
        self.config = config;
    }

    /// Evaluate an observation.
    ///
    /// All values must be normalized between `0.0` and `1.0`.
    pub fn process(
        &self,
        novelty: f32,
        importance: f32,
        urgency: f32,
    ) -> AttentionResult {
        let score = self.calculate_score(
            novelty,
            importance,
            urgency,
        );

        let priority = self.assign_priority(score);

        let accepted = self.should_accept(score);

        AttentionResult::new(
            score,
            priority,
            accepted,
        )
    }

    /// Calculates the weighted attention score.
    fn calculate_score(
        &self,
        novelty: f32,
        importance: f32,
        urgency: f32,
    ) -> f32 {
        calculate_score(
            novelty,
            importance,
            urgency,
            self.config.novelty_weight,
            self.config.importance_weight,
            self.config.urgency_weight,
        )
    }

    /// Maps a score to a priority.
    fn assign_priority(&self, score: f32) -> crate::types::Priority {
        determine_priority(score)
    }

    /// Determines whether the observation should continue
    /// through the cognitive pipeline.
    fn should_accept(&self, score: f32) -> bool {
        accept(
            score,
            self.config.acceptance_threshold,
        )
    }
}

#[derive(Debug, Clone)]
pub struct AttentionMetrics {
    pub novelty: f32,
    pub importance: f32,
    pub urgency: f32,
}