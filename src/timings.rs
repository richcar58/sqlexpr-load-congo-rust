
/// Tracks timing data for all expressions in a single complexity class
#[derive(Debug, Clone)]
pub struct ClassTimings {
    pub accumulated_time: f64,
    pub min_time: f64,
    pub max_time: f64,
    pub evaluation_count: usize,
    pub unique_expr_count: usize,
    pub individual_timings: Vec<f64>,
}

impl ClassTimings {
    pub fn new() -> Self {
        Self {
            accumulated_time: 0.0,
            min_time: f64::MAX,
            max_time: 0.0,
            evaluation_count: 0,
            unique_expr_count: 0,
            individual_timings: Vec::new(),
        }
    }

    /// Add a timing measurement for a single evaluation
    pub fn add_timing(&mut self, duration_secs: f64) {
        self.accumulated_time += duration_secs;
        self.min_time = self.min_time.min(duration_secs);
        self.max_time = self.max_time.max(duration_secs);
        self.evaluation_count += 1;
        self.individual_timings.push(duration_secs);
    }

    /// Increment the count of unique expressions evaluated
    pub fn increment_unique_expr(&mut self) {
        self.unique_expr_count += 1;
    }
}

impl Default for ClassTimings {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_class_timings() {
        let timings = ClassTimings::new();
        assert_eq!(timings.accumulated_time, 0.0);
        assert_eq!(timings.min_time, f64::MAX);
        assert_eq!(timings.max_time, 0.0);
        assert_eq!(timings.evaluation_count, 0);
        assert_eq!(timings.unique_expr_count, 0);
        assert_eq!(timings.individual_timings.len(), 0);
    }

    #[test]
    fn test_add_timing() {
        let mut timings = ClassTimings::new();

        timings.add_timing(0.001);
        assert_eq!(timings.evaluation_count, 1);
        assert_eq!(timings.accumulated_time, 0.001);
        assert_eq!(timings.min_time, 0.001);
        assert_eq!(timings.max_time, 0.001);

        timings.add_timing(0.003);
        assert_eq!(timings.evaluation_count, 2);
        assert_eq!(timings.accumulated_time, 0.004);
        assert_eq!(timings.min_time, 0.001);
        assert_eq!(timings.max_time, 0.003);
    }

    #[test]
    fn test_increment_unique_expr() {
        let mut timings = ClassTimings::new();
        assert_eq!(timings.unique_expr_count, 0);

        timings.increment_unique_expr();
        assert_eq!(timings.unique_expr_count, 1);

        timings.increment_unique_expr();
        assert_eq!(timings.unique_expr_count, 2);
    }
}
