//! Attention filtering.
//!
//! Determines whether an observation should pass
//! through the attention gate.

/// Returns `true` if the score meets or exceeds the threshold.
///
/// Both values are expected to be in the range [0.0, 1.0].
#[inline]
pub fn accept(
    score: f32,
    threshold: f32,
) -> bool {
    score >= threshold
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_above_threshold() {
        assert!(accept(0.80, 0.50));
    }

    #[test]
    fn accepts_equal_threshold() {
        assert!(accept(0.50, 0.50));
    }

    #[test]
    fn rejects_below_threshold() {
        assert!(!accept(0.49, 0.50));
    }

    #[test]
    fn accepts_maximum_score() {
        assert!(accept(1.00, 0.50));
    }

    #[test]
    fn rejects_zero_score() {
        assert!(!accept(0.00, 0.50));
    }

    #[test]
    fn accepts_zero_threshold() {
        assert!(accept(0.00, 0.00));
    }

    #[test]
    fn rejects_when_threshold_is_one() {
        assert!(!accept(0.99, 1.00));
    }

    #[test]
    fn accepts_when_both_are_one() {
        assert!(accept(1.00, 1.00));
    }
}