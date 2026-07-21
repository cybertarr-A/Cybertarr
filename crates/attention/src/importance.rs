//! Importance analysis.
//!
//! Importance measures how relevant an observation is to the
//! organism's current internal state.

use crate::types::AttentionMetrics;
use state::InternalState;
use world::Observation;

pub struct ImportanceAnalyzer;

impl ImportanceAnalyzer {
    /// Returns an importance score between 0.0 and 1.0.
    pub fn analyze(
        observation: &Observation,
        state: &InternalState,
    ) -> f32 {
        let mut importance = 0.3;

        match observation.value.as_str() {
            "Food" => {
                importance = 1.0 - state.energy;
            }

            "Water" => {
                importance = 1.0 - state.hydration;
            }

            "Temperature" => {
                importance = if state.temperature < 35.5
                    || state.temperature > 38.0
                {
                    0.9
                } else {
                    0.4
                };
            }

            "Movement" => {
                importance = 0.7;
            }

            "Sound" => {
                importance = 0.5;
            }

            "Light" => {
                importance = 0.3;
            }

            _ => {}
        }

        importance.clamp(0.0, 1.0)
    }
}