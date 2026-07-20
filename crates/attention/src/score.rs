/// Calculates the weighted attention score.
///
/// All inputs are expected to be normalized between 0.0 and 1.0.
pub fn calculate_score(
    novelty: f32,
    importance: f32,
    urgency: f32,
    novelty_weight: f32,
    importance_weight: f32,
    urgency_weight: f32,
) -> f32 {
    let score =
        novelty * novelty_weight
        + importance * importance_weight
        + urgency * urgency_weight;

    score.clamp(0.0, 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculates_score() {
        let score = calculate_score(
            1.0,
            1.0,
            1.0,
            0.3,
            0.4,
            0.3,
        );

        assert_eq!(score, 1.0);
    }
}