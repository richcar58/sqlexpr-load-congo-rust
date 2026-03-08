use sqlexpr_congo_rust::{evaluate, RuntimeValue};
use std::collections::HashMap;
use std::time::Instant;

/// Evaluate an expression with timing measurement
///
/// Returns the duration in seconds (as f64) on success,
/// or an error message on failure.
pub fn evaluate_timed(
    expr: &str,
    map: &HashMap<String, RuntimeValue>,
) -> Result<f64, String> {
    let start = Instant::now();

    match evaluate(expr, map) {
        Ok(_result) => {
            let duration = start.elapsed();
            Ok(duration.as_secs_f64())
        }
        Err(e) => Err(format!("{:?}", e)),
    }
}
