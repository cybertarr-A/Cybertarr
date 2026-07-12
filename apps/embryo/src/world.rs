use crate::observation::Observation;

pub struct World {
    tick: u64,
}

impl World {
    pub fn new() -> Self {
        Self { tick: 0 }
    }

    pub fn observe(&mut self) -> Observation {
        self.tick += 1;

        let value = match self.tick % 6 {
            0 => "Light",
            1 => "Dark",
            2 => "Sound",
            3 => "Movement",
            4 => "Temperature",
            _ => "Silence",
        };

        Observation::new(
            self.tick,
            "World",
            value,
        )
    }
}