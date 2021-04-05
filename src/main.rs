mod cli;

use anyhow::{ensure, Context, Result};
use clap::Clap;
use libchiron::config::Config;
use libchiron::scoring::report;
use libchiron::sys;
use std::fs;

fn main() -> Result<()> {
	ensure!(
		matches!(sys::check_trace(), Ok(false)),
		"Detected engine tracing, killing engine!"
	);

	let opts = cli::Opts::parse();
	let raw = fs::read_to_string(&opts.config)
		.context("Failed to read scoring config!")?;
	let config = toml::from_str::<Config>(&raw)
		.context("Failed to parse scoring config!")?;

	report::generate(config.score())
		.context("Failed to generate scoring report!")
}
