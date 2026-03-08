use crate::timings::ClassTimings;

/// Statistical summary for a single complexity class
#[derive(Debug, Clone)]
pub struct ClassStatistics {
    pub complexity: usize,
    pub total_evaluations: usize,
    pub unique_expressions: usize,
    pub total_time_secs: f64,
    pub min_time_secs: f64,
    pub max_time_secs: f64,
    pub avg_time_secs: f64,
    pub stdev_secs: f64,
}

impl ClassStatistics {
    /// Create statistics from accumulated timing data
    pub fn from_timings(complexity: usize, timings: &ClassTimings) -> Self {
        let avg = if timings.evaluation_count > 0 {
            timings.accumulated_time / timings.evaluation_count as f64
        } else {
            0.0
        };

        let stdev = calculate_stdev(&timings.individual_timings, avg);

        Self {
            complexity,
            total_evaluations: timings.evaluation_count,
            unique_expressions: timings.unique_expr_count,
            total_time_secs: timings.accumulated_time,
            min_time_secs: if timings.min_time == f64::MAX {
                0.0
            } else {
                timings.min_time
            },
            max_time_secs: timings.max_time,
            avg_time_secs: avg,
            stdev_secs: stdev,
        }
    }
}

/// Calculate sample standard deviation using Bessel's correction (n-1)
///
/// Formula: sqrt(Σ(x_i - mean)² / (n - 1))
pub fn calculate_stdev(values: &[f64], mean: f64) -> f64 {
    if values.len() <= 1 {
        return 0.0;
    }

    let variance = values
        .iter()
        .map(|&v| (v - mean).powi(2))
        .sum::<f64>()
        / (values.len() - 1) as f64;

    variance.sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_stdev_empty() {
        assert_eq!(calculate_stdev(&[], 0.0), 0.0);
    }

    #[test]
    fn test_calculate_stdev_single() {
        assert_eq!(calculate_stdev(&[1.0], 1.0), 0.0);
    }

    #[test]
    fn test_calculate_stdev_multiple() {
        let values = vec![2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];
        let mean = values.iter().sum::<f64>() / values.len() as f64;
        let stdev = calculate_stdev(&values, mean);

        // Expected stdev for this sample is approximately 2.138
        assert!((stdev - 2.138).abs() < 0.01);
    }

    #[test]
    fn test_class_statistics_from_timings() {
        let mut timings = ClassTimings::new();
        timings.add_timing(0.001);
        timings.add_timing(0.002);
        timings.add_timing(0.003);
        timings.increment_unique_expr();
        timings.increment_unique_expr();

        let stats = ClassStatistics::from_timings(5, &timings);

        assert_eq!(stats.complexity, 5);
        assert_eq!(stats.total_evaluations, 3);
        assert_eq!(stats.unique_expressions, 2);
        assert_eq!(stats.total_time_secs, 0.006);
        assert_eq!(stats.min_time_secs, 0.001);
        assert_eq!(stats.max_time_secs, 0.003);
        assert!((stats.avg_time_secs - 0.002).abs() < 1e-10);
    }
}
