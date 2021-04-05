use crate::config::Check;
use crate::CHIRON_DIR;
use anyhow::Result;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

/// Generate the scoring report given an iterator over checks
pub fn generate(checks: impl Iterator<Item = Check>) -> Result<()> {
	let to_write = format!(
		"<ol>{}</ol>",
		checks
			.map(|c| format!("<li>{} - {} points</li>", c.name, c.points))
			.collect::<Vec<_>>()
			.join("\n")
	);

	let mut handle = OpenOptions::new()
		.write(true)
		.create(true)
		.open(PathBuf::from(CHIRON_DIR).join("scoring.html"))?;
	handle.write_all(to_write.as_bytes()).map_err(Into::into)
}
