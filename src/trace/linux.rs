use anyhow::Result;
use procfs::process::Process;

pub(crate) fn check_trace() -> Result<bool> {
	let status = Process::myself()?.status()?;
	status.tracerpid != 0
}
