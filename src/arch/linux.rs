use crate::checks::ScoreableCheck;
use crate::config::PackageInstalled;
use anyhow::Result;
use procfs::process::Process;
use apt_pkg_native::Cache;

/// Check if the process is being traced by checking `/proc/self/status` for
/// `tracerpid`
pub fn check_trace() -> Result<bool> {
	let status = Process::myself()?.status()?;
	Ok(status.tracerpid != 0)
}

#[typetag::serde]
impl ScoreableCheck for PackageInstalled {
	fn score(&self) -> Result<bool> {
		Ok(!matches!(Cache::get_singleton().find_by_name(&self.package).next(), None))
	}
}
