use crate::{
    error::AttentionError,
    types::AttentionConfig,
};

pub fn validate(config: &AttentionConfig) -> Result<(), AttentionError> {
    if !(0.0..=1.0).contains(&config.novelty_weight) {
        return Err(AttentionError::InvalidNoveltyWeight);
    }

    if !(0.0..=1.0).contains(&config.importance_weight) {
        return Err(AttentionError::InvalidImportanceWeight);
    }

    if !(0.0..=1.0).contains(&config.urgency_weight) {
        return Err(AttentionError::InvalidUrgencyWeight);
    }

    if !(0.0..=1.0).contains(&config.acceptance_threshold) {
        return Err(AttentionError::InvalidAcceptanceThreshold);
    }

    Ok(())
}