use memory::Memory;
use state::Observation;

pub struct NoveltyAnalyzer;

impl NoveltyAnalyzer {
    pub fn calculate(
        observation: &Observation,
        memory: &Memory,
    ) -> f32 {
        match memory.recall(&observation.value) {
            Ok(record) => record.novelty_score,
            Err(_) => 1.0,
        }
    }
}