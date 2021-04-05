use anyhow::Result;
use apt_pkg_native::Cache;
use procfs::process::Process;
use std::fs::File;
use std::os::unix::fs::PermissionsExt;
use std::process::Command;
use std::str;

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
	let file_perms = {
		let f = File::open(&file)?;
		let meta = f.metadata()?;
		meta.permissions().mode() & 0o777
	};

	Ok(file_perms == u32::from_str_radix(&perms, 8)?)
}

/// Check if firewall is enabled
pub fn firewall_enabled() -> Result<bool> {
	let stdout = Command::new("ufw").arg("status").output()?.stdout;

	Ok(str::from_utf8(&stdout)?.contains("Status: active"))
}
