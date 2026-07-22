use state::Observation;
use state::InternalState;

pub struct Physiology;

impl Physiology {
    pub fn process(
        state: &mut InternalState,
        observation: &Observation,
    ) {
match observation.value.as_str() {

    "Light" => {
        state.curiosity += 0.02;
        state.uncertainty -= 0.01;
    }

    "Dark" => {
        state.curiosity -= 0.01;
        state.uncertainty += 0.03;
    }

    "Sound" => {
        state.uncertainty += 0.02;
    }

    "Movement" => {
        state.curiosity += 0.05;
        state.energy -= 0.002;
    }

    "Temperature" => {
        state.energy -= 0.01;
    }

    "Silence" => {
        state.uncertainty -= 0.02;
    }

    _ => {}
}

        state.energy = state.energy.clamp(0.0, 2.0);
        state.curiosity = state.curiosity.clamp(0.0, 2.0);
        state.uncertainty = state.uncertainty.clamp(0.0, 2.0);
    }
}