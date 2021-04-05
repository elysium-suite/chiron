use crate::check::Scorable;
use serde::{Deserialize, Serialize};

/// A check to ensure that a system command will contain a certain value.
#[derive(Serialize, Deserialize, Debug)]
pub struct CommandContains {
	/// Command to run
	pub command: String,
	/// Regular expression it must match
	pub contains: String,
}

/// A check to ensure a package is installed.
#[derive(Serialize, Deserialize, Debug)]
pub struct PackageInstalled {
	/// Package that should be installed
	pub package: String,
}

/// A check for file permissions.
#[derive(Serialize, Deserialize, Debug)]
pub struct FilePermissions {
	/// File to check
	pub file: String,
	/// Secured permissions
	pub perms: String,
}

/// A check to ensure that the firewall is enabled.
#[derive(Serialize, Deserialize, Debug)]
pub struct FirewallEnabled {}

/// Main check structure; contains check metadata as well as conditions
#[derive(Serialize, Deserialize)]
pub struct Check {
	/// Check message
	pub name: String,
	/// Points to award if the check succeeds
	pub points: usize,
	#[serde(default)]
	pass: Vec<Box<dyn Scorable>>,
	#[serde(default)]
	fail: Vec<Box<dyn Scorable>>,
}

impl Check {
	fn met(&self) -> bool {
		self.pass.iter().all(|c| matches!(c.score(), Ok(true)))
			&& self.fail.iter().all(|c| matches!(c.score(), Ok(false)))
	}
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
		self.check.into_iter().filter(Check::met)
	}
}
