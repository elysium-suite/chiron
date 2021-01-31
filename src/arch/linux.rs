use anyhow::Result;
use apt_pkg_native::Cache;
use procfs::process::Process;
use regex::bytes::Regex;
use std::{process::Command, str};

/// Check if the process is being traced by checking `/proc/self/status` for
/// `tracerpid`
pub fn check_trace() -> Result<bool> {
	let status = Process::myself()?.status()?;
	Ok(status.tracerpid != 0)
}

/// Check if package is installed
pub fn package_installed(package: &str) -> Result<bool> {
	Ok(!matches!(
		Cache::get_singleton().find_by_name(&package).next(),
		None
	))
}

/// Check if file permissions are secured
pub fn file_permissions(file: &str, perms: &str) -> Result<bool> {
	let stdout = Command::new("stat")
		.arg("-c")
		.arg("'%a'")
		.arg(&file)
		.output()?
		.stdout;
	let regex = Regex::new(&perms)?;

	Ok(regex.is_match(&stdout))
}
