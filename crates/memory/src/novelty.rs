/// Computes novelty scores for observations.
///
/// Novelty is normalized between:
///
/// 1.0 -> Completely novel
/// 0.0 -> Completely familiar
///
/// The current implementation uses a simple frequency-based decay.
/// More advanced models (Bayesian, probabilistic, neural) can replace
/// this later without changing the public API.
pub struct Novelty;

impl Novelty {
    /// Compute a novelty score from the number of times
    /// an observation has been seen.
    ///
    /// Examples:
    ///
    /// seen = 1   -> 1.000
    /// seen = 2   -> 0.500
    /// seen = 5   -> 0.200
    /// seen = 10  -> 0.100
    pub fn calculate(times_seen: u64) -> f32 {
        if times_seen == 0 {
            return 1.0;
        }

        1.0 / times_seen as f32
    }

    /// Returns true if the observation is considered novel.
    pub fn is_novel(times_seen: u64, threshold: f32) -> bool {
        Self::calculate(times_seen) >= threshold
    }

    /// Clamp a novelty score into the valid range.
    pub fn normalize(score: f32) -> f32 {
        score.clamp(0.0, 1.0)
    }
}