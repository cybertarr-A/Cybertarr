use crate::heartbeat::heartbeat;
use crate::homeostasis::Homeostasis;
use crate::logger::Logger;
use crate::physiology::Physiology;
use crate::world::World;

use attention::{
    AttentionAnalyzer,
    AttentionConfig,
    AttentionEngine,
};

use memory::Memory;
use state::InternalState;

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
            attention: AttentionEngine::new(
                AttentionConfig::default(),
            ),
        }
    }

    pub fn tick(&mut self) {
        // Advance organism
        heartbeat(&mut self.state);

        // Observe environment
        let observation = self.world.observe();

        // Update physiology
        Physiology::process(
            &mut self.state,
            &observation,
        );

        // Maintain homeostasis
        Homeostasis::regulate(
            &mut self.state,
        );

        // Analyze observation and generate attention metrics
        let input = AttentionAnalyzer::analyze(
            &observation,
            &self.memory,
            &self.state,
        );

        // Evaluate attention
        let attention_result = self
            .attention
            .evaluate(input)
            .expect("Attention evaluation failed");

        // Store accepted observations
        if attention_result.accepted {
            let _ = self.memory.observe(
                observation.value.clone(),
                self.state.age_ticks,
            );
        }

        // Get memory statistics
        let stats = self.memory.statistics();

        // Record experiment
        self.logger.record(
            &observation,
            &attention_result,
            &self.state,
            stats.clone(),
        );

        // Save every 100 ticks
        if self.state.age_ticks % 100 == 0 {
            self.logger.save();
        }

        println!("\n==============================");
        println!("Heartbeat {}", self.state.age_ticks);

        println!("\nObservation");
        println!("{:#?}", observation);

        println!("\nAttention");
        println!("{:#?}", attention_result);

        println!("\nMemory");
        println!(
            "Unique Memories   : {}",
            stats.total_memories
        );
        println!(
            "Observations      : {}",
            stats.total_observations
        );
        println!(
            "Average Novelty   : {:.3}",
            stats.average_novelty
        );
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
            println!(
                "Experiment directory: {:?}",
                self.logger.experiment_directory()
            );

            std::process::exit(0);
        }
    }
}