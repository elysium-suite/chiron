#[macro_use]
mod macros;
mod checks;
mod config;

use anyhow::Result;
use config::Config;
use std::fs;

fn main() -> Result<()> {
	// TODO: read w clap (index = 1)
	let raw = fs::read_to_string("examples/scoring.toml")?;
	let config: Config = toml::from_str(&raw)?;

	println!("{:?}", config.score());
	Ok(())
}
