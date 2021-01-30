use crate::checks::ScoreableCheck;
use crate::config::PackageInstalled;
use std::{process::Command, str};
use anyhow::Result;
use procfs::process::Process;
use regex::bytes::Regex;

/// Check if the process is being traced by checking `/proc/self/status` for
/// `tracerpid`
pub fn check_trace() -> Result<bool> {
	let status = Process::myself()?.status()?;
	Ok(status.tracerpid != 0)
}

#[typetag::serde]
impl ScoreableCheck for PackageInstalled {
	fn score(&self) -> Result<bool> {
		let args = self.package.split(' ');

		let stdout = Command::new("/usr/bin/dpkg").arg("-s").args(args).output()?.stdout;
		let regex = Regex::new("Status: install ok installed")?;

		Ok(regex.is_match(&stdout))
	}
}
