use thiserror::Error;

/// Errors produced by the Memory subsystem.
#[derive(Debug, Error)]
pub enum MemoryError {
    /// Memory record was not found.
    #[error("memory record not found: {0}")]
    RecordNotFound(String),

    /// Attempted to insert an invalid observation.
    #[error("invalid observation: {0}")]
    InvalidObservation(String),

    /// Duplicate memory record.
    #[error("duplicate memory record: {0}")]
    DuplicateRecord(String),

    /// Working memory has reached capacity.
    #[error("working memory capacity exceeded")]
    WorkingMemoryFull,

    /// Storage failure.
    #[error("storage error: {0}")]
    StorageError(String),

    /// Generic internal error.
    #[error("internal memory error: {0}")]
    Internal(String),
}