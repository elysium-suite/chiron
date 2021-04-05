use anyhow::Result;
use winapi::um::debugapi;

/// Returns true if the engine is being traced. Currently uses the Windows API's
/// IsDebuggerPresent method.
pub fn check_trace() -> Result<bool> {
	let present = unsafe { debugapi::IsDebuggerPresent() };
	Ok(present != 0)
}

/// Check if package is installed
pub fn package_installed(package: &str) -> Result<bool> { todo!() }

/// Check if file permissions are secured
pub fn file_permissions(file: &str, perms: &str) -> Result<bool> { todo!() }

/// Check if firewall is enabled
pub fn firewall_enabled() -> Result<bool> { todo!() }
