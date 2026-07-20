/// Returns true if the attention score passes the threshold.
pub fn accept(score: f32, threshold: f32) -> bool {
    score >= threshold
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_score() {
        assert!(accept(0.8, 0.5));
    }

    #[test]
    fn rejects_score() {
        assert!(!accept(0.2, 0.5));
    }
}