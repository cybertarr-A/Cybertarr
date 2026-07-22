//! Maps an attention score to a cognitive priority.

use crate::types::Priority;

/// Converts a normalized attention score into a priority level.
///
/// Thresholds:
///
/// 0.90..=1.00 -> Critical
/// 0.70..0.90  -> High
/// 0.40..0.70  -> Medium
/// 0.00..0.40  -> Low
pub fn determine_priority(score: f32) -> Priority {
    match score {
        s if s >= 0.90 => Priority::Critical,
        s if s >= 0.70 => Priority::High,
        s if s >= 0.40 => Priority::Medium,
        _ => Priority::Low,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_critical() {
        assert_eq!(
            determine_priority(0.95),
            Priority::Critical
        );
    }

    #[test]
    fn returns_high() {
        assert_eq!(
            determine_priority(0.75),
            Priority::High
        );
    }

    #[test]
    fn returns_medium() {
        assert_eq!(
            determine_priority(0.55),
            Priority::Medium
        );
    }

    #[test]
    fn returns_low() {
        assert_eq!(
            determine_priority(0.20),
            Priority::Low
        );
    }

    #[test]
    fn boundary_values() {
        assert_eq!(
            determine_priority(0.90),
            Priority::Critical
        );

        assert_eq!(
            determine_priority(0.70),
            Priority::High
        );

        assert_eq!(
            determine_priority(0.40),
            Priority::Medium
        );

        assert_eq!(
            determine_priority(0.39),
            Priority::Low
        );
    }
}