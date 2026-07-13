use crate::heartbeat::heartbeat;
use crate::homeostasis::Homeostasis;
use crate::logger::Logger;
use crate::physiology::Physiology;
use crate::state::InternalState;
use crate::world::World;

pub struct Brain {
    pub state: InternalState,
    pub world: World,
    pub logger: Logger,
}

impl Brain {
    pub fn new() -> Self {
        Self {
            state: InternalState::default(),
            world: World::new(),
            logger: Logger::new(),
        }
    }

    pub fn tick(&mut self) {
        // Advance the organism
        heartbeat(&mut self.state);

        // Observe the world
        let observation = self.world.observe();

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

        // Save the current log
        // (Temporary: we'll optimize this later)
        self.logger.save();

        println!("\n==============================");
        println!("❤️ Heartbeat {}", self.state.age_ticks);

        println!("\n👁 Observation");
        println!("{:#?}", observation);

        println!("\n🫀 Homeostasis Updated");

        println!("\n🧠 Internal State");
        println!("{:#?}", self.state);

        println!("==============================");

        if !self.state.alive {
            println!("\n💀 Embryo has died.");

            // Final save before exit
            self.logger.save();
            
            println!("💾 Experiment saved.");

            std::process::exit(0);
        }
    }
}