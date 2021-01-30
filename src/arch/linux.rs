use anyhow::Result;
use procfs::process::Process;
use apt_pkg_native::Cache;

/// Check if the process is being traced by checking `/proc/self/status` for
/// `tracerpid`
pub fn check_trace() -> Result<bool> {
	let status = Process::myself()?.status()?;
	Ok(status.tracerpid != 0)
}

/// Check if package is installed
pub fn package_installed(package: &str) -> Result<bool> {
	Ok(!matches!(Cache::get_singleton().find_by_name(&package).next(), None))
}
