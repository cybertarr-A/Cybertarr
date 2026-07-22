//! Validation utilities for the Attention Engine.

use crate::{
    error::AttentionError,
    types::{AttentionConfig, AttentionInput},
};

/// Validates attention inputs and configuration.
pub struct Validator;

impl Validator {
    /// Validate the input metrics.
    pub fn validate_input(
        input: &AttentionInput,
    ) -> Result<(), AttentionError> {
        Self::validate_metric("novelty", input.novelty)?;
        Self::validate_metric("importance", input.importance)?;
        Self::validate_metric("urgency", input.urgency)?;

        Ok(())
    }

    /// Validate the engine configuration.
    pub fn validate_config(
        config: &AttentionConfig,
    ) -> Result<(), AttentionError> {
        Self::validate_metric(
            "novelty_weight",
            config.novelty_weight,
        )?;

        Self::validate_metric(
            "importance_weight",
            config.importance_weight,
        )?;

        Self::validate_metric(
            "urgency_weight",
            config.urgency_weight,
        )?;

        Self::validate_metric(
            "acceptance_threshold",
            config.acceptance_threshold,
        )?;

        let total = config.novelty_weight
            + config.importance_weight
            + config.urgency_weight;

        const EPSILON: f32 = 0.0001;

        if (total - 1.0).abs() > EPSILON {
            return Err(
                AttentionError::InvalidWeights {
                    total,
                },
            );
        }

        Ok(())
    }

    fn validate_metric(
        field: &'static str,
        value: f32,
    ) -> Result<(), AttentionError> {
        if !(0.0..=1.0).contains(&value) {
            return Err(
                AttentionError::InvalidInput {
                    field,
                    value,
                },
            );
        }

        Ok(())
    }
}