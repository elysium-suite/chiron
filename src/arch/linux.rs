use anyhow::Result;
use apt_pkg_native::Cache;
use procfs::process::Process;
use std::{fs::File, os::unix::fs::PermissionsExt};

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
	let f = File::open(&file)?;
	let metadata = f.metadata()?;
	let current_perms = metadata.permissions().mode();
	Ok((current_perms & 0o7777) == u32::from_str_radix(&perms, 8)?)
}
