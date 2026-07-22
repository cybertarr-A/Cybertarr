use state::Observation;

pub struct UrgencyAnalyzer;

impl UrgencyAnalyzer {
    pub fn calculate(
        observation: &Observation,
    ) -> f32 {
        let value = observation.value.to_lowercase();

        if value.contains("danger") {
            return 1.0;
        }

        if value.contains("predator") {
            return 1.0;
        }

        if value.contains("fire") {
            return 1.0;
        }

        if value.contains("movement") {
            return 0.9;
        }

        if value.contains("sound") {
            return 0.8;
        }

        if value.contains("temperature") {
            return 0.6;
        }

        if value.contains("light") {
            return 0.4;
        }

        if value.contains("silence") {
            return 0.2;
        }

        0.5
    }
}