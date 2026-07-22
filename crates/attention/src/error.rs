//! Error types for the Attention Engine.

use std::fmt;

/// Errors that can occur while evaluating attention.
#[derive(Debug, Clone, PartialEq)]
pub enum AttentionError {
    /// One or more input values are outside the valid range [0.0, 1.0].
    InvalidInput {
        field: &'static str,
        value: f32,
    },

    /// Configuration weights do not sum to 1.0.
    InvalidWeights {
        total: f32,
    },

    /// Acceptance threshold is outside the valid range.
    InvalidThreshold {
        value: f32,
    },
}

impl fmt::Display for AttentionError {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            AttentionError::InvalidInput { field, value } => {
                write!(
                    f,
                    "invalid value {} for '{}'. Expected a value between 0.0 and 1.0",
                    value,
                    field
                )
            }

            AttentionError::InvalidWeights { total } => {
                write!(
                    f,
                    "attention weights must sum to 1.0 (current total: {})",
                    total
                )
            }

            AttentionError::InvalidThreshold { value } => {
                write!(
                    f,
                    "acceptance threshold must be between 0.0 and 1.0 (received {})",
                    value
                )
            }
        }
    }
}

impl std::error::Error for AttentionError {}