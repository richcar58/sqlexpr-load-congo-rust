# Release Notes

## Version 1.0.0 - March 9, 2026

Initial release of sqlexpr-load-congo-rust, a comprehensive load testing harness for the sqlexpr-congo-rust boolean SQL expression evaluation library.

### Features

#### Core Functionality
- **Expression Evaluation**: Loads and evaluates boolean SQL expressions from JSON test data
- **Complexity-Based Grouping**: Automatically groups expressions by complexity (number of AND/OR operators)
- **High-Precision Timing**: Measures execution time using `std::time::Instant` with microsecond precision
- **Configurable Iterations**: Support for repeated evaluations via `--iterations` parameter (default: 1)
- **Error Handling**: Comprehensive error tracking with detailed failure reports

#### Statistical Analysis
- **Per-Complexity-Class Statistics**:
  - Total execution time
  - Minimum execution time
  - Maximum execution time
  - Average execution time
  - Standard deviation (using Bessel's correction)
- **Overall Metrics**:
  - Total expressions evaluated
  - Total evaluations executed
  - Total execution time
  - Failed evaluation count

#### Output & Reporting
- **Markdown Reports**: Human-readable output in `output/` directory
  - `test_timings.md`: Comprehensive statistics by complexity class
  - `failed_tests.md`: Detailed failure information (created only if errors occur)
- **Timestamp Tracking**:
  - Records start time in both local time and UTC
  - Timestamps captured when processing begins
- **Millisecond Precision**: All timing values displayed in milliseconds with 6 decimal places
- **Dynamic Column Widths**: Tables automatically adjust to fit content with proper padding
- **Progress Reporting**: Console feedback every 1000 expressions during long-running tests

#### Command-Line Interface
- `--iterations N`: Specify number of times to evaluate each expression (must be ≥ 1)
- Built with `clap` for robust argument parsing and validation
- Helpful error messages and usage information

**Repository**: https://github.com/richcar58/sqlexpr-load-congo-rust

