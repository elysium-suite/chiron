use super::SCORING_REPORT_FILE;
use crate::config::Check;
use anyhow::{Context, Result};
use std::fs;

/// Generate the scoring report given an iterator over checks
pub fn generate(checks: impl Iterator<Item = Check>) -> Result<()> {
	let to_write = format!(
		"<ol>{}</ol>",
		checks
			.map(|c| format!("{} - {} points", c.name, c.points))
			.collect::<Vec<_>>()
			.join("\n")
	);

	fs::write(SCORING_REPORT_FILE, to_write)
		.context("Failed to write to scoring report!")
}
