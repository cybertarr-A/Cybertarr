//! # Attention
//!
//! A lightweight cognitive attention engine.
//!
//! The engine evaluates normalized attention metrics and determines
//! whether a stimulus deserves further processing.
//!
//! ## Pipeline
//!
//! ```text
//! AttentionInput
//!        │
//!        ▼
//! Validation
//!        │
//!        ▼
//! Weighted Score
//!        │
//!        ▼
//! Priority Mapping
//!        │
//!        ▼
//! Acceptance Filter
//!        │
//!        ▼
//! AttentionResult
//! ```
//!
//! This crate is intentionally independent of any application-specific
//! types such as observations, memories, or world models.

mod attention;
mod error;
mod filter;
mod priority;
mod score;
mod types;
mod validation;

pub use attention::AttentionEngine;
pub use error::AttentionError;
pub use types::{
    AttentionConfig,
    AttentionInput,
    AttentionResult,
    Priority,
};