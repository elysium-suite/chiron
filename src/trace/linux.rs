use procfs::process::Process;

pub(crate) fn check_trace() -> bool {
	let status = Process::myself()?.status()?;
	status.tracerpid != 0
}
