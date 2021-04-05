use crate::config::{
	CommandContains, FilePermissions, FirewallEnabled, PackageInstalled,
};
use crate::sys::{file_permissions, firewall_enabled, package_installed};
use anyhow::Result;
use regex::bytes::Regex;
use std::process::Command;
use std::str;

/// Wrapper trait for all scorable checks
#[typetag::serde(tag = "type")]
pub trait Scorable {
	/// Function that returns whether the check has passed or not
	fn score(&self) -> Result<bool>;
}

#[typetag::serde]
impl Scorable for CommandContains {
	fn score(&self) -> Result<bool> {
		let mut args = self.command.split(' ');
		let cmd = args.next().unwrap();

		let stdout = Command::new(cmd).args(args).output()?.stdout;
		let regex = Regex::new(&self.contains)?;

		Ok(regex.is_match(&stdout))
	}
}

#[typetag::serde]
impl Scorable for PackageInstalled {
	fn score(&self) -> Result<bool> { package_installed(&self.package) }
}

#[typetag::serde]
impl Scorable for FilePermissions {
	fn score(&self) -> Result<bool> {
		file_permissions(&self.file, &self.perms)
	}
}

#[typetag::serde]
impl Scorable for FirewallEnabled {
	fn score(&self) -> Result<bool> { firewall_enabled() }
}
