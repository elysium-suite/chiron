use anyhow::Result;
use procfs::process::Process;

/// Check if the process is being traced by reading /proc/self/status for
/// tracerpid
pub fn check_trace() -> Result<bool> {
	let status = Process::myself()?.status()?;
	Ok(status.tracerpid != 0)
}
