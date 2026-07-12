#[derive(Debug, Clone)]
pub struct InternalState {
    pub alive: bool,
    pub age_ticks: u64,

    pub energy: f32,
    pub curiosity: f32,
    pub uncertainty: f32,
}

impl Default for InternalState {
    fn default() -> Self {
        Self {
            alive: true,
            age_ticks: 0,
            energy: 1.0,
            curiosity: 1.0,
            uncertainty: 1.0,
        }
    }
}