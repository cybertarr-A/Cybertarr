use crate::state::InternalState;

pub fn heartbeat(state: &mut InternalState) {
    state.age_ticks += 1;

    println!("❤️ Heartbeat {}", state.age_ticks);
}