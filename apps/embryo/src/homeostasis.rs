use state::InternalState;

pub struct Homeostasis;

impl Homeostasis {
    pub fn regulate(state: &mut InternalState) {

        // -------------------------
        // Basal metabolism
        // -------------------------

        // Every living organism consumes energy.
        state.energy -= 0.001;

        // Every organism slowly becomes fatigued.
        state.fatigue += 0.002;

        // -------------------------
        // Stress dynamics
        // -------------------------

        // Stress naturally decreases over time.
        state.stress *= 0.995;

        // Low energy increases stress.
        state.stress += (1.0 - state.energy).max(0.0) * 0.002;

        // Uncertainty is mentally demanding.
        state.stress += state.uncertainty * 0.001;

        // Curiosity has a cognitive cost.
        state.stress += state.curiosity * 0.0005;

        // Fatigue also contributes to stress.
        state.stress += state.fatigue * 0.001;

        // -------------------------
        // Health dynamics
        // -------------------------

        // Chronic stress slowly reduces health.
        state.health -= state.stress * 0.001;

        // Exhaustion reduces health.
        if state.fatigue > 0.9 {
            state.health -= 0.002;
        }

        // Starvation reduces health.
        if state.energy < 0.2 {
            state.health -= 0.005;
        }

        // -------------------------
        // Clamp everything
        // -------------------------

        state.energy = state.energy.clamp(0.0, 1.0);

        state.curiosity = state.curiosity.clamp(0.0, 2.0);

        state.uncertainty = state.uncertainty.clamp(0.0, 2.0);

        state.fatigue = state.fatigue.clamp(0.0, 1.0);

        state.stress = state.stress.clamp(0.0, 1.0);

        state.health = state.health.clamp(0.0, 1.0);

        // -------------------------
        // Death
        // -------------------------

        if state.health <= 0.0 {
            state.alive = false;
        }
    }
}