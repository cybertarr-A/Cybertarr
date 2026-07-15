use crate::heartbeat::heartbeat;
use crate::homeostasis::Homeostasis;
use crate::logger::Logger;
use crate::physiology::Physiology;
use crate::state::InternalState;
use crate::world::World;

use memory::Memory;

pub struct Brain {
    pub state: InternalState,
    pub world: World,
    pub memory: Memory,
    pub logger: Logger,
}

impl Brain {
    pub fn new() -> Self {
        Self {
            state: InternalState::default(),
            world: World::new(),
            memory: Memory::new(64),
            logger: Logger::new(),
        }
    }

    pub fn tick(&mut self) {
        // Advance the organism
        heartbeat(&mut self.state);

        // Observe the world
        let observation = self.world.observe();

        // Store observation in memory
        //
        // NOTE:
        // We temporarily convert the observation to a string.
        // Later we'll replace this with a proper Observation type.
        let _ = self.memory.observe(
            format!("{:?}", observation),
            self.state.age_ticks,
        );

        // Physiological response
        Physiology::process(
            &mut self.state,
            &observation,
        );

        // Maintain internal stability
        Homeostasis::regulate(
            &mut self.state,
        );

        // Record this tick
        self.logger.record(
            &observation,
            &self.state,
        );

        self.logger.save();

        let stats = self.memory.statistics();

        println!("\n==============================");
        println!("Heartbeat {}", self.state.age_ticks);

        println!("\nObservation");
        println!("{:#?}", observation);

        println!("\nMemory");
        println!("Unique Memories   : {}", stats.total_memories);
        println!("Observations      : {}", stats.total_observations);
        println!("Average Novelty   : {:.3}", stats.average_novelty);
        println!(
            "Most Seen         : {:?}",
            stats.most_seen_observation
        );

        println!("\nInternal State");
        println!("{:#?}", self.state);

        println!("==============================");

        if !self.state.alive {
            println!("\nEmbryo has died.");

            self.logger.save();

            println!("Experiment saved.");

            std::process::exit(0);
        }
    }
}