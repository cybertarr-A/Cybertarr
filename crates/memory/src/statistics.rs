use crate::storage::MemoryStorage;

/// Derived statistics for the Memory subsystem.
///
/// These values are calculated from the current memory storage
/// and are not persisted.
#[derive(Debug, Clone)]
pub struct MemoryStatistics {
    /// Total number of unique memories.
    pub total_memories: usize,

    /// Average novelty across all memories.
    pub average_novelty: f32,

    /// Average importance across all memories.
    pub average_importance: f32,

    /// Average confidence across all memories.
    pub average_confidence: f32,

    /// Total observation count.
    pub total_observations: u64,

    /// Highest repetition count.
    pub most_seen_count: u64,

    /// Observation seen the most.
    pub most_seen_observation: Option<String>,
}

impl MemoryStatistics {
    /// Compute statistics from the current memory storage.
    pub fn from_storage(storage: &MemoryStorage) -> Self {
        let total_memories = storage.len();

        if total_memories == 0 {
            return Self {
                total_memories: 0,
                average_novelty: 0.0,
                average_importance: 0.0,
                average_confidence: 0.0,
                total_observations: 0,
                most_seen_count: 0,
                most_seen_observation: None,
            };
        }

        let mut novelty_sum = 0.0;
        let mut importance_sum = 0.0;
        let mut confidence_sum = 0.0;

        let mut total_observations = 0;

        let mut most_seen_count = 0;
        let mut most_seen_observation = None;

        for record in storage.iter() {
            novelty_sum += record.novelty_score;
            importance_sum += record.importance;
            confidence_sum += record.confidence;

            total_observations += record.times_seen;

            if record.times_seen > most_seen_count {
                most_seen_count = record.times_seen;
                most_seen_observation =
                    Some(record.observation.clone());
            }
        }

        Self {
            total_memories,

            average_novelty:
                novelty_sum / total_memories as f32,

            average_importance:
                importance_sum / total_memories as f32,

            average_confidence:
                confidence_sum / total_memories as f32,

            total_observations,

            most_seen_count,

            most_seen_observation,
        }
    }
}