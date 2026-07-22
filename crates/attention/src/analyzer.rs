use memory::Memory;
use state::{InternalState, Observation};

use crate::{
    importance::ImportanceAnalyzer,
    novelty::NoveltyAnalyzer,
    urgency::UrgencyAnalyzer,
    AttentionInput,
};

pub struct AttentionAnalyzer;

impl AttentionAnalyzer {
    pub fn analyze(
        observation: &Observation,
        memory: &Memory,
        state: &InternalState,
    ) -> AttentionInput {
        let novelty = NoveltyAnalyzer::calculate(
            observation,
            memory,
        );

        let importance = ImportanceAnalyzer::calculate(
            observation,
            state,
        );

        let urgency = UrgencyAnalyzer::calculate(
            observation,
        );

        AttentionInput::new(
            novelty,
            importance,
            urgency,
        )
    }
}