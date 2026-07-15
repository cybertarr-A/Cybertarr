use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Represents a single memory stored by the Digital Embryo.
///
/// A MemoryRecord captures an observed event along with metadata that
/// allows higher cognitive systems (Brain, Learning, Attention) to
/// reason about familiarity, novelty, and importance.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryRecord {
    /// Unique identifier for this memory.
    pub id: Uuid,

    /// The observed stimulus.
    pub observation: String,

    /// Tick when the memory was first created.
    pub first_seen_tick: u64,

    /// Tick when the memory was last observed.
    pub last_seen_tick: u64,

    /// Number of times this observation has occurred.
    pub times_seen: u64,

    /// Novelty score (0.0 = completely familiar, 1.0 = completely novel).
    pub novelty_score: f32,

    /// Importance assigned by future cognitive systems.
    pub importance: f32,

    /// Confidence that this memory is correct.
    pub confidence: f32,

    /// Real-world timestamp when the memory was created.
    pub created_at: DateTime<Utc>,

    /// Last update timestamp.
    pub updated_at: DateTime<Utc>,
}

impl MemoryRecord {
    /// Creates a new memory record from an observation.
    pub fn new(observation: impl Into<String>, tick: u64) -> Self {
        let now = Utc::now();

        Self {
            id: Uuid::new_v4(),
            observation: observation.into(),

            first_seen_tick: tick,
            last_seen_tick: tick,

            times_seen: 1,

            novelty_score: 1.0,
            importance: 0.5,
            confidence: 1.0,

            created_at: now,
            updated_at: now,
        }
    }

    /// Updates the memory after observing the same stimulus again.
    pub fn observe_again(&mut self, tick: u64) {
        self.last_seen_tick = tick;
        self.times_seen += 1;
        self.updated_at = Utc::now();
    }
}