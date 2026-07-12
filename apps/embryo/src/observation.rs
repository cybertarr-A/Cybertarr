#[derive(Debug, Clone)]
pub struct Observation {
    pub tick: u64,
    pub source: String,
    pub value: String,
}

impl Observation {
    pub fn new(
        tick: u64,
        source: impl Into<String>,
        value: impl Into<String>,
    ) -> Self {
        Self {
            tick,
            source: source.into(),
            value: value.into(),
        }
    }
}