use crate::{
    error::MemoryError,
    novelty::Novelty,
    record::MemoryRecord,
    statistics::MemoryStatistics,
    storage::MemoryStorage,
    working_memory::WorkingMemory,
};

/// Central Memory subsystem.
///
/// This is the public interface used by the rest of Cybertarr.
/// Internally it coordinates storage, working memory,
/// novelty calculation and statistics.
#[derive(Debug)]
pub struct Memory {
    storage: MemoryStorage,
    working_memory: WorkingMemory,
}

impl Memory {
    /// Creates a new memory system.
    pub fn new(working_memory_capacity: usize) -> Self {
        Self {
            storage: MemoryStorage::new(),
            working_memory: WorkingMemory::new(
                working_memory_capacity,
            ),
        }
    }

    /// Observe a stimulus.
    ///
    /// If the observation already exists, it is updated.
    /// Otherwise a new memory record is created.
    pub fn observe(
        &mut self,
        observation: impl Into<String>,
        tick: u64,
    ) -> Result<(), MemoryError> {
        let observation = observation.into();

        if observation.trim().is_empty() {
            return Err(
                MemoryError::InvalidObservation(
                    "Observation cannot be empty".into(),
                ),
            );
        }

        if self.storage.contains(&observation) {
            let record =
                self.storage.get_mut(&observation)?;

            record.observe_again(tick);

            record.novelty_score =
                Novelty::calculate(record.times_seen);

            self.working_memory.push(record.clone());

            return Ok(());
        }

        let record =
            MemoryRecord::new(observation, tick);

        self.storage.insert(record.clone())?;

        self.working_memory.push(record);

        Ok(())
    }

    /// Returns an immutable memory record.
    pub fn recall(
        &self,
        observation: &str,
    ) -> Result<&MemoryRecord, MemoryError> {
        self.storage.get(observation)
    }

    /// Returns true if the observation exists.
    pub fn contains(
        &self,
        observation: &str,
    ) -> bool {
        self.storage.contains(observation)
    }

    /// Number of unique memories.
    pub fn total_memories(&self) -> usize {
        self.storage.len()
    }

    /// Returns current statistics.
    pub fn statistics(
        &self,
    ) -> MemoryStatistics {
        MemoryStatistics::from_storage(
            &self.storage,
        )
    }

    /// Returns the working memory.
    pub fn working_memory(
        &self,
    ) -> &WorkingMemory {
        &self.working_memory
    }

    /// Clears all memories.
    pub fn clear(&mut self) {
        self.storage.clear();
        self.working_memory.clear();
    }
}