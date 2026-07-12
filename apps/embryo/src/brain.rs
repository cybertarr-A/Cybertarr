use crate::heartbeat::heartbeat;
use crate::state::InternalState;

pub struct Brain {
    pub state: InternalState,
}

impl Brain {
    pub fn new() -> Self {
        Self {
            state: InternalState::default(),
        }
    }

    pub fn tick(&mut self) {
        heartbeat(&mut self.state);

        println!("{:#?}", self.state);
    }
}