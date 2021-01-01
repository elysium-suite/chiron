use winapi::um::debugapi;

pub(crate) fn check_trace() -> bool {
	let present = unsafe { debugapi::IsDebuggerPresent() };
	present != 0
}
