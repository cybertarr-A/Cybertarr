use crate::heartbeat::heartbeat;
use crate::homeostasis::Homeostasis;
use crate::logger::Logger;
use crate::physiology::Physiology;
use crate::state::InternalState;
use crate::world::World;

use attention::{AttentionConfig, AttentionEngine};
use memory::Memory;

pub struct Brain {
    pub state: InternalState,
    pub world: World,
    pub memory: Memory,
    pub logger: Logger,
    pub attention: AttentionEngine,
}

impl Brain {
    pub fn new() -> Self {
        Self {
            state: InternalState::default(),
            world: World::new(),
            memory: Memory::new(64),
            logger: Logger::new(),

            // Initialize the attention engine
            attention: AttentionEngine::new(
                AttentionConfig::default(),
            ),
        }
    }

    pub fn tick(&mut self) {
        // Advance the organism
        heartbeat(&mut self.state);

        // Observe the environment
        let observation = self.world.observe();

        // ----------------------------------------------------
        // Temporary attention evaluation.
        // These values are placeholders and will later be
        // computed from the Observation.
        // ----------------------------------------------------
        let attention_result = self.attention.process(
            0.8, // novelty
            0.6, // importance
            0.5, // urgency
        );

        // Store only observations that passed attention.
        if attention_result.accepted {
            let _ = self.memory.observe(
                observation.value.clone(),
                self.state.age_ticks,
            );
        }

        // Physiological response
        Physiology::process(
            &mut self.state,
            &observation,
        );

        // Maintain internal stability
        Homeostasis::regulate(
            &mut self.state,
        );

        // Log the tick
        self.logger.record(
            &observation,
            &attention_result,
            &self.state,
        );

        self.logger.save();

        let stats = self.memory.statistics();

        println!("\n==============================");
        println!("Heartbeat {}", self.state.age_ticks);

        println!("\nObservation");
        println!("{:#?}", observation);

        println!("\nAttention");
        println!("{:#?}", attention_result);

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