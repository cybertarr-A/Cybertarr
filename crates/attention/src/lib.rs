//! # Attention
//!
//! A lightweight biologically-inspired attention system.
//!
//! ## Pipeline
//!
//! ```text
//! Observation
//!        │
//!        ▼
//! Novelty Analysis
//!        │
//!        ├────────────┐
//!        ▼            │
//! Importance Analysis │
//!        ▼            │
//! Urgency Analysis    │
//!        └────────────┘
//!              │
//!              ▼
//!       AttentionInput
//!              │
//!              ▼
//!         Validation
//!              │
//!              ▼
//!       Weighted Score
//!              │
//!              ▼
//!      Priority Mapping
//!              │
//!              ▼
//!      Acceptance Filter
//!              │
//!              ▼
//!      AttentionResult
//! ```
//!
//! This crate provides both the attention analysis layer and the
//! attention evaluation engine.

mod analyzer;
mod attention;
mod error;
mod filter;
mod importance;
mod novelty;
mod priority;
mod score;
mod types;
mod urgency;
mod validation;

pub use analyzer::AttentionAnalyzer;
pub use attention::AttentionEngine;
pub use error::AttentionError;
pub use types::{
    AttentionConfig,
    AttentionInput,
    AttentionResult,
    Priority,
};