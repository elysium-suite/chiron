use crate::checks::ScoreableCheck;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct CommandContains {
	pub(crate) command: String,
	pub(crate) contains: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub(crate) enum Condition {
	CommandContains(CommandContains),
}

// unfortunately, we can't deserialize into the ScoreableCheck trait because
// it's unsized. this is the best solution i can think of for now. see
// Cargo.toml for an example crate that _might_ be able to address our use case
// by deserializing directly into boxed traits.
impl Condition {
	fn score(self) -> Result<bool> {
		match self {
			Condition::CommandContains(cond) => cond.score(),
			// rest of the variants...
		}
	}
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Check {
	name: String,
	points: usize,
	pass: Option<Vec<Condition>>,
	fail: Option<Vec<Condition>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Config {
	title: String,
	user: String,
	os: String,
	remote: String,
	local: bool,
	check: Vec<Check>,
}

impl Config {
	// consume self
	// obviously return something more descriptive than a score but this is just
	// a POC
	pub(crate) fn score(self) -> Result<usize> {
		let mut points = 0usize;

		for check in self.check {
			let mut scored = false;
			score_checks!(check, scored);

			if scored {
				points += check.points;
			}
		}

		Ok(points)
	}
}
