use crate::observation::Observation;

pub struct Universe {
    tick: u64,
}

impl Universe {
    pub fn new() -> Self {
        Self { tick: 0 }
    }

    pub fn next_observation(&mut self) -> Observation {

        self.tick += 1;

        Observation::new(
            self.tick,
            "universe",
            format!("heartbeat_event_{}", self.tick),
        )
    }
}