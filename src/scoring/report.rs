use crate::config::Check;
use std::fs;

/// Writes to scoring report given an iterator of scored checks
pub fn write_to_scoring_report(
	file: &str,
	scored_vector: impl Iterator<Item = Check>,
) -> anyhow::Result<()> {
	let mut text: Vec<String> = Vec::new();
	text.push(String::from("<ul>"));
	text.append(
		&mut scored_vector
			.map(|scored| {
				format!(
					"<li>Check passed: {} - {} points</li>",
					scored.name, scored.points
				)
			})
			.collect(),
	);
	text.push(String::from("</ul>"));
	fs::write(file, text.join("\n"))?;
	Ok(())
}
