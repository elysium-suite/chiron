use anyhow::Result;
use winapi::um::debugapi;

pub(crate) fn check_trace() -> Result<bool> {
	let present = unsafe { debugapi::IsDebuggerPresent() };
	Ok(present != 0)
}
