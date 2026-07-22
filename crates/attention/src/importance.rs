use state::{InternalState, Observation};

pub struct ImportanceAnalyzer;

impl ImportanceAnalyzer {
    pub fn calculate(
        observation: &Observation,
        state: &InternalState,
    ) -> f32 {
        let mut score = 0.0;

        // Curiosity drives exploration.
        score += state.curiosity * 0.35;

        // Stress increases importance.
        score += state.stress * 0.25;

        // Healthy organisms investigate more.
        score += state.health * 0.20;

        // Energy determines available resources.
        score += state.energy * 0.20;

        // Visual observations receive a slight bonus.
        if observation.source.eq_ignore_ascii_case("vision") {
            score += 0.05;
        }

        score.clamp(0.0, 1.0)
    }
}