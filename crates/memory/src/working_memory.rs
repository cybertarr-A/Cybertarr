use std::collections::VecDeque;

use crate::record::MemoryRecord;

/// Working memory is a short-term buffer that stores the
/// most recent observations.
///
/// When capacity is exceeded, the oldest memory is discarded.
#[derive(Debug)]
pub struct WorkingMemory {
    capacity: usize,
    memories: VecDeque<MemoryRecord>,
}

impl WorkingMemory {
    /// Creates a new working memory with a fixed capacity.
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            memories: VecDeque::with_capacity(capacity),
        }
    }

    /// Returns the maximum capacity.
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Returns the current number of memories.
    pub fn len(&self) -> usize {
        self.memories.len()
    }

    /// Returns true if working memory is empty.
    pub fn is_empty(&self) -> bool {
        self.memories.is_empty()
    }

    /// Returns true if working memory is full.
    pub fn is_full(&self) -> bool {
        self.memories.len() >= self.capacity
    }

    /// Adds a new memory.
    ///
    /// If working memory is full, the oldest memory
    /// is automatically removed.
    pub fn push(&mut self, record: MemoryRecord) {
        if self.is_full() {
            self.memories.pop_front();
        }

        self.memories.push_back(record);
    }

    /// Returns the most recent memory.
    pub fn latest(&self) -> Option<&MemoryRecord> {
        self.memories.back()
    }

    /// Returns the oldest memory.
    pub fn oldest(&self) -> Option<&MemoryRecord> {
        self.memories.front()
    }

    /// Clears working memory.
    pub fn clear(&mut self) {
        self.memories.clear();
    }

    /// Returns an iterator over all memories.
    pub fn iter(&self) -> impl Iterator<Item = &MemoryRecord> {
        self.memories.iter()
    }

    /// Returns the memory at the given index.
    pub fn get(&self, index: usize) -> Option<&MemoryRecord> {
        self.memories.get(index)
    }
}