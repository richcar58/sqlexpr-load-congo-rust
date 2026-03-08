use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "sqlexpr-load-rust")]
#[command(about = "Load testing harness for sqlexpr-rust library")]
pub struct Args {
    #[arg(long, default_value_t = 1)]
    #[arg(help = "Number of times to evaluate each expression (must be >= 1)")]
    pub iterations: usize,

    #[arg(long, default_value = "complex_expressions-max.json")]
    #[arg(help = "Input file path (absolute or relative to resources/ directory)")]
    pub input: String,
}

impl Args {
    pub fn validate(&self) -> Result<(), String> {
        if self.iterations < 1 {
            return Err(format!(
                "Invalid iterations value: {}. Must be >= 1",
                self.iterations
            ));
        }
        Ok(())
    }
}
