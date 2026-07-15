use crate::{
    error::MemoryError,
    record::MemoryRecord,
    storage::MemoryStorage,
};

/// Long-term memory stores persistent memories that survive
/// beyond the limited working memory.
///
/// In Version 0.1 this is simply a wrapper around MemoryStorage.
/// Future versions will implement:
/// - Memory consolidation
/// - Forgetting
/// - Importance weighting
/// - Episodic memory
/// - Semantic memory
#[derive(Debug, Default)]
pub struct LongTermMemory {
    storage: MemoryStorage,
}

impl LongTermMemory {
    /// Creates an empty long-term memory.
    pub fn new() -> Self {
        Self {
            storage: MemoryStorage::new(),
        }
    }

    /// Stores a memory.
    pub fn store(
        &mut self,
        record: MemoryRecord,
    ) -> Result<(), MemoryError> {
        self.storage.insert(record)
    }

    /// Recalls a memory.
    pub fn recall(
        &self,
        observation: &str,
    ) -> Result<&MemoryRecord, MemoryError> {
        self.storage.get(observation)
    }

    /// Returns true if the memory exists.
    pub fn contains(
        &self,
        observation: &str,
    ) -> bool {
        self.storage.contains(observation)
    }

    /// Number of stored memories.
    pub fn len(&self) -> usize {
        self.storage.len()
    }

    /// Returns true if empty.
    pub fn is_empty(&self) -> bool {
        self.storage.is_empty()
    }

    /// Removes every stored memory.
    pub fn clear(&mut self) {
        self.storage.clear();
    }

    /// Returns an iterator over all memories.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = &MemoryRecord> {
        self.storage.iter()
    }
}