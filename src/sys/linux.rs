use anyhow::Result;
use procfs::process::Process;
use std::ffi::CString;
use std::fs::File;
use std::os::unix::fs::PermissionsExt;
use std::process::Command;
use std::str;

mod ffi {
	use std::os::raw::{c_char, c_int};

	#[link(name = "libdpkg-wrapper", kind = "static")]
	#[link(name = "dpkg")]
	extern "C" {
		pub fn dpkg_package_installed(name: *const c_char) -> c_int;
	}
}

/// Check if the process is being traced by checking `/proc/self/status` for
/// `tracerpid`
pub fn check_trace() -> Result<bool> {
	let status = Process::myself()?.status()?;
	Ok(status.tracerpid != 0)
}

/// Check if package is installed
pub fn package_installed(package: &str) -> Result<bool> {
	let installed = unsafe {
		let s = CString::new(package).unwrap();
		ffi::dpkg_package_installed(s.as_ptr()) != 0
	};
	Ok(installed)
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
