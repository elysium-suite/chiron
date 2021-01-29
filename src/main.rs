use anyhow::{ensure, Result};
use libchiron::{arch, config::Config};
use libchiron::scoringreport::write_to_scoring_report;
use std::fs;

fn main() -> Result<()> {
	ensure!(
		matches!(arch::check_trace(), Ok(false)),
		"Detected engine tracing, killing engine!"
	);

	let raw = fs::read_to_string("examples/scoring.toml")?;
  let config: Config = toml::from_str(&raw)?;
  
  write_to_scoring_report(&String::from("/opt/chiron/scoringreport.html"), config.score());
  
	Ok(())
}
