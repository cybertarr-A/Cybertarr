#[cfg(test)]
mod tests {
    use crate::{
        memory::Memory,
        novelty::Novelty,
        record::MemoryRecord,
        working_memory::WorkingMemory,
    };

    #[test]
    fn creates_new_memory() {
        let memory = Memory::new(10);

        assert_eq!(memory.total_memories(), 0);
    }

    #[test]
    fn stores_new_observation() {
        let mut memory = Memory::new(10);

        memory.observe("Light", 1).unwrap();

        assert!(memory.contains("Light"));

        assert_eq!(memory.total_memories(), 1);
    }

    #[test]
    fn repeated_observation_updates_record() {
        let mut memory = Memory::new(10);

        memory.observe("Light", 1).unwrap();
        memory.observe("Light", 2).unwrap();

        let record = memory.recall("Light").unwrap();

        assert_eq!(record.times_seen, 2);

        assert_eq!(record.last_seen_tick, 2);
    }

    #[test]
    fn novelty_decreases() {
        assert_eq!(Novelty::calculate(1), 1.0);

        assert_eq!(Novelty::calculate(2), 0.5);

        assert_eq!(Novelty::calculate(4), 0.25);
    }

    #[test]
    fn working_memory_respects_capacity() {
        let mut wm = WorkingMemory::new(2);

        wm.push(MemoryRecord::new("Light", 1));

        wm.push(MemoryRecord::new("Sound", 2));

        wm.push(MemoryRecord::new("Dark", 3));

        assert_eq!(wm.len(), 2);

        assert_eq!(
            wm.oldest().unwrap().observation,
            "Sound"
        );

        assert_eq!(
            wm.latest().unwrap().observation,
            "Dark"
        );
    }

    #[test]
    fn recall_existing_memory() {
        let mut memory = Memory::new(10);

        memory.observe("Temperature", 8).unwrap();

        let record = memory.recall("Temperature").unwrap();

        assert_eq!(record.observation, "Temperature");
    }

    #[test]
    fn statistics_are_generated() {
        let mut memory = Memory::new(10);

        memory.observe("Light", 1).unwrap();

        memory.observe("Sound", 2).unwrap();

        memory.observe("Light", 3).unwrap();

        let stats = memory.statistics();

        assert_eq!(stats.total_memories, 2);

        assert_eq!(stats.total_observations, 3);

        assert_eq!(
            stats.most_seen_observation.unwrap(),
            "Light"
        );
    }

    #[test]
    fn clearing_memory_removes_everything() {
        let mut memory = Memory::new(10);

        memory.observe("Light", 1).unwrap();

        memory.clear();

        assert_eq!(memory.total_memories(), 0);

        assert!(!memory.contains("Light"));
    }
}