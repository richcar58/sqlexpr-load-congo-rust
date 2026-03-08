use crate::statistics::ClassStatistics;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FailedTest {
    pub expr: String,
    pub value_map: HashMap<String, serde_json::Value>,
    pub error_message: String,
}

/// Overall statistics across all complexity classes
pub struct OverallStats {
    pub input_file_path: String,
    pub start_time_local: chrono::DateTime<chrono::Local>,
    pub start_time_utc: chrono::DateTime<chrono::Utc>,
    pub iterations: usize,
    pub total_expressions: usize,
    pub total_evaluations: usize,
    pub total_time_secs: f64,
    pub total_failures: usize,
}

/// Generate the test_timings.md report
pub fn generate_timings_report(
    mut stats: Vec<ClassStatistics>,
    overall: OverallStats,
) -> String {
    // Sort by complexity class (ascending)
    stats.sort_by_key(|s| s.complexity);

    let mut report = String::new();

    // Header
    report.push_str("# Load Test Results - sqlexpr-rust\n\n");

    // Overall statistics
    report.push_str("## Overall Statistics\n\n");
    report.push_str(&format!(
        "- **Start Time (Local)**: {}\n",
        overall.start_time_local.format("%Y-%m-%d %H:%M:%S %Z")
    ));
    report.push_str(&format!(
        "- **Start Time (UTC)**: {}\n",
        overall.start_time_utc.format("%Y-%m-%d %H:%M:%S %Z")
    ));
    report.push_str(&format!(
        "- **Input File**: {}\n",
        overall.input_file_path
    ));
    report.push_str(&format!(
        "- **Iterations**: {}\n",
        overall.iterations
    ));
    report.push_str(&format!(
        "- **Total Expressions Evaluated**: {}\n",
        overall.total_expressions
    ));
    report.push_str(&format!(
        "- **Total Evaluations Executed**: {}\n",
        overall.total_evaluations
    ));
    report.push_str(&format!(
        "- **Total Execution Time**: {:.3} ms\n",
        overall.total_time_secs * 1000.0
    ));
    report.push_str(&format!(
        "- **Failed Evaluations**: {}\n\n",
        overall.total_failures
    ));

    // Results by complexity class
    report.push_str("## Results by Complexity Class\n\n");

    for stat in stats {
        report.push_str(&format!(
            "### Complexity Class {} ({} expressions)\n\n",
            stat.complexity, stat.unique_expressions
        ));

        // Calculate column widths for this complexity class
        let metrics = vec![
            ("Total Time", stat.total_time_secs * 1000.0),
            ("Min Time", stat.min_time_secs * 1000.0),
            ("Max Time", stat.max_time_secs * 1000.0),
            ("Average Time", stat.avg_time_secs * 1000.0),
            ("Std Deviation", stat.stdev_secs * 1000.0),
        ];

        // Find max widths for each column
        let metric_col_width = metrics
            .iter()
            .map(|(name, _)| name.len())
            .max()
            .unwrap_or(6)
            .max(6); // "Metric" header is 6 chars

        let ms_col_width = metrics
            .iter()
            .map(|(_, value)| format!("{:.6}", value).len())
            .max()
            .unwrap_or(13)
            .max(13); // "Milliseconds" header is 13 chars

        // Add padding (2 spaces on each side)
        let metric_width = metric_col_width + 2;
        let ms_width = ms_col_width + 2;

        // Build table with dynamic widths
        let metric_header = format!("{:^width$}", "Metric", width = metric_width);
        let ms_header = format!("{:^width$}", "Milliseconds", width = ms_width);
        report.push_str(&format!("|{}|{}|\n", metric_header, ms_header));

        let metric_sep = "-".repeat(metric_width);
        let ms_sep = "-".repeat(ms_width);
        report.push_str(&format!("|{}|{}|\n", metric_sep, ms_sep));

        for (metric_name, value) in metrics {
            let metric_cell = format!("{:width$}", format!(" {}", metric_name), width = metric_width);
            let value_cell = format!("{:>width$.6} ", value, width = ms_width - 1);
            report.push_str(&format!("|{}|{}|\n", metric_cell, value_cell));
        }
        report.push('\n');

        // Additional details
        report.push_str(&format!(
            "- **Total Evaluations**: {}\n",
            stat.total_evaluations
        ));
        report.push_str(&format!(
            "- **Unique Expressions**: {}\n\n",
            stat.unique_expressions
        ));
    }

    // Footer
    report.push_str("---\n\n");
    report.push_str(&format!(
        "Generated: {}\n",
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S")
    ));

    report
}

/// Generate the failed_tests.md report
pub fn generate_failed_tests_report(failures: &[FailedTest]) -> String {
    let mut report = String::new();

    report.push_str("# Failed Expression Evaluations\n\n");
    report.push_str(&format!("Total failures: {}\n\n", failures.len()));

    for (i, failure) in failures.iter().enumerate() {
        report.push_str(&format!("## Failure {}\n\n", i + 1));

        report.push_str("**Expression:**\n");
        report.push_str("```\n");
        report.push_str(&failure.expr);
        report.push_str("\n```\n\n");

        report.push_str("**Value Map:**\n");
        report.push_str("```json\n");
        report.push_str(&serde_json::to_string_pretty(&failure.value_map).unwrap());
        report.push_str("\n```\n\n");

        report.push_str("**Error:**\n");
        report.push_str("```\n");
        report.push_str(&failure.error_message);
        report.push_str("\n```\n\n");

        report.push_str("---\n\n");
    }

    report.push_str(&format!(
        "Generated: {}\n",
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S")
    ));

    report
}
