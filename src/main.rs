mod cli;
mod complexity;
mod evaluator;
mod input;
mod reporter;
mod statistics;
mod timings;

use clap::Parser;
use cli::Args;
use complexity::calculate_complexity;
use evaluator::evaluate_timed;
use input::load_tests;
use reporter::{generate_failed_tests_report, generate_timings_report, FailedTest, OverallStats};
use statistics::ClassStatistics;
use std::collections::{HashMap, HashSet};
use std::fs;
use timings::ClassTimings;

fn main() -> Result<(), anyhow::Error> {
    // Parse and validate command-line arguments
    let args = Args::parse();
    args.validate()
        .map_err(|e| anyhow::anyhow!(e))?;

    println!(
        "Starting load test with {} iteration(s) per expression...",
        args.iterations
    );

    // Resolve input file path
    let input_path = if std::path::Path::new(&args.input).is_absolute() {
        std::path::PathBuf::from(&args.input)
    } else {
        std::path::PathBuf::from("resources").join(&args.input)
    };

    // Validate file exists
    if !input_path.exists() {
        return Err(anyhow::anyhow!(
            "Input file not found: {}\nSpecified: {}",
            input_path.display(),
            args.input
        ));
    }

    // Canonicalize for reporting
    let canonical_input_path = std::fs::canonicalize(&input_path)?;
    let input_path_str = canonical_input_path.to_string_lossy().to_string();

    // Load test expressions from JSON
    println!("Loading test expressions from {}...", input_path.display());
    let tests = load_tests(input_path.to_str().unwrap())?;
    println!("Loaded {} test expressions", tests.len());

    // Create output directory if needed
    fs::create_dir_all("output")?;

    // Initialize tracking data structures
    let mut class_timings: HashMap<usize, ClassTimings> = HashMap::new();
    let mut error_list: Vec<FailedTest> = Vec::new();
    let mut failed_expr_indices: HashSet<usize> = HashSet::new();

    // Capture start time when processing begins
    let start_time_local = chrono::Local::now();
    let start_time_utc = chrono::Utc::now();

    // Main evaluation loop
    println!("Evaluating expressions...");
    for (idx, test) in tests.iter().enumerate() {
        // Skip expressions that have already failed
        if failed_expr_indices.contains(&idx) {
            continue;
        }

        // Calculate complexity and convert value map
        let complexity = calculate_complexity(&test.expr);
        let runtime_map = match test.to_runtime_map() {
            Ok(map) => map,
            Err(e) => {
                error_list.push(FailedTest {
                    expr: test.expr.clone(),
                    value_map: test.value_map.clone(),
                    error_message: format!("Value map conversion error: {}", e),
                });
                failed_expr_indices.insert(idx);
                continue;
            }
        };

        // Get or create complexity class entry
        let timings = class_timings
            .entry(complexity)
            .or_default();

        // Increment unique expression count (once per expression)
        timings.increment_unique_expr();

        // Evaluate expression N times
        for _ in 0..args.iterations {
            match evaluate_timed(&test.expr, &runtime_map) {
                Ok(duration) => {
                    timings.add_timing(duration);
                }
                Err(e) => {
                    error_list.push(FailedTest {
                        expr: test.expr.clone(),
                        value_map: test.value_map.clone(),
                        error_message: e,
                    });
                    failed_expr_indices.insert(idx);
                    break; // Don't retry this expression
                }
            }
        }

        // Progress update every 1000 expressions
        if (idx + 1) % 1000 == 0 {
            println!("Processed {} expressions...", idx + 1);
        }
    }

    println!("Evaluation complete!");
    println!("Calculating statistics...");

    // Calculate statistics for each complexity class
    let all_stats: Vec<ClassStatistics> = class_timings
        .iter()
        .map(|(complexity, timings)| ClassStatistics::from_timings(*complexity, timings))
        .collect();

    // Calculate overall statistics
    let total_expressions = tests.len() - failed_expr_indices.len();
    let total_evaluations: usize = all_stats.iter().map(|s| s.total_evaluations).sum();
    let total_time_secs: f64 = all_stats.iter().map(|s| s.total_time_secs).sum();
    let total_failures = error_list.len();

    let overall = OverallStats {
        input_file_path: input_path_str,
        start_time_local,
        start_time_utc,
        iterations: args.iterations,
        total_expressions,
        total_evaluations,
        total_time_secs,
        total_failures,
    };

    // Generate and write reports
    println!("Generating reports...");

    // Generate test_timings.md
    let timings_report = generate_timings_report(all_stats, overall);
    fs::write("output/test_timings.md", timings_report)?;
    println!("✓ Written output/test_timings.md");

    // Generate failed_tests.md if there were failures
    if !error_list.is_empty() {
        let failed_report = generate_failed_tests_report(&error_list);
        fs::write("output/failed_tests.md", failed_report)?;
        println!("✓ Written output/failed_tests.md");
        println!("  {} failed evaluations recorded", total_failures);
    }

    // Summary
    println!("\nLoad Test Summary:");
    println!("==================");
    println!("Total expressions: {}", total_expressions);
    println!("Total evaluations: {}", total_evaluations);
    println!(
        "Total time: {:.3} ms",
        total_time_secs * 1000.0
    );
    println!("Failed evaluations: {}", total_failures);
    println!("\nReports generated in output/ directory");

    Ok(())
}
