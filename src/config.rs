use crate::{checks::ScoreableCheck, each_cond_contains};
use serde::{Deserialize, Serialize};

/// A check to ensure that a system command will contain a certain value.
#[derive(Serialize, Deserialize, Debug)]
pub struct CommandContains {
	/// Command to run
	pub command: String,
	/// Regular expression it must match
	pub contains: String,
}

/// Main check structure; contains check metadata as well as conditions
#[derive(Serialize, Deserialize)]
pub struct Check {
	/// Check message
	pub name: String,
	/// Points to award if the check succeeds
	pub points: usize,
	pass: Option<Vec<Box<dyn ScoreableCheck>>>,
	fail: Option<Vec<Box<dyn ScoreableCheck>>>,
}

/// Generic configuration struct for Chiron. Contains image metadata as well as
/// checks.
#[derive(Serialize, Deserialize)]
pub struct Config {
	title: String,
	user: String,
	os: String,
	remote: String,
	local: bool,
	check: Vec<Check>,
}

impl Config {
	/// Score all checks in the configuration.
	pub fn score(self) -> impl Iterator<Item = Check> {
		self.check.into_iter().filter(|check| {
			each_cond_contains!(check.fail, false)
				&& each_cond_contains!(check.pass, true)
		})
	}
}
