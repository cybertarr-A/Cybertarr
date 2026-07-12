use crate::heartbeat::heartbeat;
use crate::observation::Observation;
use crate::state::InternalState;
use crate::world::World;

pub struct Brain {
    pub state: InternalState,
    pub world: World,
}

impl Brain {
    pub fn new() -> Self {
        Self {
            state: InternalState::default(),
            world: World::new(),
        }
    }

    pub fn tick(&mut self) {
        heartbeat(&mut self.state);

        let observation: Observation = self.world.observe();

        println!("👁 Observation");
        println!("{:#?}", observation);

        println!("🧠 Internal State");
        println!("{:#?}", self.state);
    }
}