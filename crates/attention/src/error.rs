use std::fmt;

/// Errors that can occur within the Attention module.
#[derive(Debug)]
pub enum AttentionError {
    InvalidNoveltyWeight,
    InvalidImportanceWeight,
    InvalidUrgencyWeight,
    InvalidAcceptanceThreshold,
}

impl fmt::Display for AttentionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidNoveltyWeight => {
                write!(f, "Novelty weight must be between 0.0 and 1.0")
            }
            Self::InvalidImportanceWeight => {
                write!(f, "Importance weight must be between 0.0 and 1.0")
            }
            Self::InvalidUrgencyWeight => {
                write!(f, "Urgency weight must be between 0.0 and 1.0")
            }
            Self::InvalidAcceptanceThreshold => {
                write!(f, "Acceptance threshold must be between 0.0 and 1.0")
            }
        }
    }
}

impl std::error::Error for AttentionError {}