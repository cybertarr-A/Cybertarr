use std::collections::HashMap;

use crate::{
    error::MemoryError,
    record::MemoryRecord,
};

/// Primary storage engine for memory records.
///
/// Internally uses a HashMap keyed by the observation string.
/// Other modules should interact through this API rather than
/// accessing the underlying storage directly.
#[derive(Debug, Default)]
pub struct MemoryStorage {
    records: HashMap<String, MemoryRecord>,
}

impl MemoryStorage {
    /// Creates an empty memory storage.
    pub fn new() -> Self {
        Self {
            records: HashMap::new(),
        }
    }

    /// Returns the number of stored memories.
    pub fn len(&self) -> usize {
        self.records.len()
    }

    /// Returns true if no memories are stored.
    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }

    /// Inserts a new memory.
    ///
    /// Returns an error if the observation already exists.
    pub fn insert(
        &mut self,
        record: MemoryRecord,
    ) -> Result<(), MemoryError> {
        if self.records.contains_key(&record.observation) {
            return Err(MemoryError::DuplicateRecord(
                record.observation.clone(),
            ));
        }

        self.records
            .insert(record.observation.clone(), record);

        Ok(())
    }

    /// Retrieves an immutable memory reference.
    pub fn get(
        &self,
        observation: &str,
    ) -> Result<&MemoryRecord, MemoryError> {
        self.records
            .get(observation)
            .ok_or_else(|| {
                MemoryError::RecordNotFound(observation.to_string())
            })
    }

    /// Retrieves a mutable memory reference.
    pub fn get_mut(
        &mut self,
        observation: &str,
    ) -> Result<&mut MemoryRecord, MemoryError> {
        self.records
            .get_mut(observation)
            .ok_or_else(|| {
                MemoryError::RecordNotFound(observation.to_string())
            })
    }

    /// Removes a memory.
    pub fn remove(
        &mut self,
        observation: &str,
    ) -> Result<MemoryRecord, MemoryError> {
        self.records
            .remove(observation)
            .ok_or_else(|| {
                MemoryError::RecordNotFound(observation.to_string())
            })
    }

    /// Returns true if the observation already exists.
    pub fn contains(&self, observation: &str) -> bool {
        self.records.contains_key(observation)
    }

    /// Clears all stored memories.
    pub fn clear(&mut self) {
        self.records.clear();
    }

    /// Immutable iterator over all memories.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = &MemoryRecord> {
        self.records.values()
    }

    /// Mutable iterator over all memories.
    pub fn iter_mut(
        &mut self,
    ) -> impl Iterator<Item = &mut MemoryRecord> {
        self.records.values_mut()
    }
}