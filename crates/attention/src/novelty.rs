//! Novelty analysis.
//!
//! This module determines how novel an observation is by querying
//! the Memory subsystem.
//!
//! The Attention subsystem should not implement its own novelty
//! algorithm. Memory is the source of truth for familiarity.

use memory::Memory;

/// Computes novelty scores for observations.
pub struct NoveltyAnalyzer;

impl NoveltyAnalyzer {
    /// Returns a novelty score between 0.0 and 1.0.
    ///
    /// 1.0 = completely novel
    /// 0.0 = completely familiar
    pub fn analyze(
        memory: &Memory,
        observation: &str,
    ) -> f32 {
        // Unknown observations are maximally novel.
        match memory.recall(observation) {
            Ok(record) => record.novelty_score,
            Err(_) => 1.0,
        }
    }
}