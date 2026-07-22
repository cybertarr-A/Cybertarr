//! Attention Engine.
//
//! This module orchestrates the complete attention pipeline.

use crate::{
    error::AttentionError,
    filter::accept,
    priority::determine_priority,
    score::calculate_score,
    types::{
        AttentionConfig,
        AttentionInput,
        AttentionResult,
    },
    validation::Validator,
};

/// Cognitive attention engine.
///
/// The engine evaluates normalized attention metrics and
/// determines whether an observation deserves attention.
#[derive(Debug, Clone)]
pub struct AttentionEngine {
    config: AttentionConfig,
}

impl AttentionEngine {
    /// Create a new attention engine.
    pub fn new(config: AttentionConfig) -> Self {
        Self { config }
    }

    /// Immutable access to the current configuration.
    pub fn config(&self) -> &AttentionConfig {
        &self.config
    }

    /// Replace the engine configuration.
    pub fn set_config(
        &mut self,
        config: AttentionConfig,
    ) -> Result<(), AttentionError> {
        Validator::validate_config(&config)?;
        self.config = config;
        Ok(())
    }

    /// Evaluate an attention input.
    pub fn evaluate(
        &self,
        input: AttentionInput,
    ) -> Result<AttentionResult, AttentionError> {
        // Validate everything first.
        Validator::validate_config(&self.config)?;
        Validator::validate_input(&input)?;

        // Calculate weighted score.
        let score = calculate_score(
            &input,
            &self.config,
        );

        // Determine cognitive priority.
        let priority = determine_priority(score);

        // Decide whether the stimulus is accepted.
        let accepted = accept(
            score,
            self.config.acceptance_threshold,
        );

        Ok(
            AttentionResult::new(
                score,
                priority,
                accepted,
            )
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evaluates_successfully() {
        let engine = AttentionEngine::new(
            AttentionConfig::default(),
        );

        let input = AttentionInput::new(
            0.8,
            0.6,
            0.5,
        );

        let result = engine.evaluate(input);

        assert!(result.is_ok());

        let result = result.unwrap();

        assert!(result.score >= 0.0);
        assert!(result.score <= 1.0);
    }

    #[test]
    fn rejects_invalid_input() {
        let engine = AttentionEngine::new(
            AttentionConfig::default(),
        );

        let input = AttentionInput::new(
            1.5,
            0.5,
            0.5,
        );

        assert!(engine.evaluate(input).is_err());
    }

    #[test]
    fn accepts_high_attention() {
        let engine = AttentionEngine::new(
            AttentionConfig::default(),
        );

        let input = AttentionInput::new(
            1.0,
            1.0,
            1.0,
        );

        let result = engine
            .evaluate(input)
            .unwrap();

        assert!(result.accepted);
    }
}