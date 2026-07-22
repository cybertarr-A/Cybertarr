//! Weighted score calculation for the Attention Engine.

use crate::types::{
    AttentionConfig,
    AttentionInput,
};

/// Calculates the final weighted attention score.
///
/// Formula:
///
/// score =
///     novelty   × novelty_weight
///   + importance × importance_weight
///   + urgency    × urgency_weight
///
/// Returns a normalized score in the range [0.0, 1.0].
pub fn calculate_score(
    input: &AttentionInput,
    config: &AttentionConfig,
) -> f32 {
    let score =
        input.novelty * config.novelty_weight
        + input.importance * config.importance_weight
        + input.urgency * config.urgency_weight;

    score.clamp(0.0, 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_weighted_score() {
        let input = AttentionInput::new(
            0.8,
            0.6,
            0.5,
        );

        let config = AttentionConfig::default();

        let score = calculate_score(
            &input,
            &config,
        );

        let expected =
            0.8 * 0.30
            + 0.6 * 0.40
            + 0.5 * 0.30;

        assert!((score - expected).abs() < 0.0001);
    }

    #[test]
    fn score_is_clamped() {
        let input = AttentionInput::new(
            2.0,
            2.0,
            2.0,
        );

        let config = AttentionConfig {
            novelty_weight: 1.0,
            importance_weight: 1.0,
            urgency_weight: 1.0,
            acceptance_threshold: 0.5,
        };

        let score = calculate_score(
            &input,
            &config,
        );

        assert_eq!(score, 1.0);
    }

    #[test]
    fn zero_score() {
        let input = AttentionInput::new(
            0.0,
            0.0,
            0.0,
        );

        let config = AttentionConfig::default();

        let score = calculate_score(
            &input,
            &config,
        );

        assert_eq!(score, 0.0);
    }
}