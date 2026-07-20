use crate::types::Priority;

/// Maps a normalized score to a priority level.
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
    fn critical_priority() {
        assert_eq!(
            determine_priority(0.95),
            Priority::Critical
        );
    }
}