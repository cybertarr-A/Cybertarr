//! Attention analyzer.
//!
//! Converts observations and internal state into the metrics
//! consumed by the AttentionEngine.

use crate::{
    importance::ImportanceAnalyzer,
    novelty::NoveltyAnalyzer,
    types::AttentionMetrics,
    urgency::UrgencyAnalyzer,
};

use memory::Memory;
use state::InternalState;
use world::Observation;

/// Computes the attention metrics for an observation.
pub struct AttentionAnalyzer;

impl AttentionAnalyzer {
    /// Analyze an observation and produce the metrics used by the
    /// attention engine.
    pub fn analyze(
        observation: &Observation,
        memory: &Memory,
        state: &InternalState,
    ) -> AttentionMetrics {
        let novelty =
            NoveltyAnalyzer::analyze(memory, observation);

        let importance =
            ImportanceAnalyzer::analyze(observation, state);

        let urgency =
            UrgencyAnalyzer::analyze(observation, state);

        AttentionMetrics::new(
            novelty,
            importance,
            urgency,
        )
    }
}