use crate::config::CommandContains;
use anyhow::Result;
use regex::Regex;
use std::{process::Command, str};

pub(crate) trait ScoreableCheck {
	fn score(&self) -> Result<bool>;
}

impl ScoreableCheck for CommandContains {
	fn score(&self) -> Result<bool> {
		let mut args = self.command.split(' ');
		let cmd = args.next().unwrap();

		let stdout = Command::new(cmd).args(args).output()?.stdout;
		let regex = Regex::new(&self.contains)?;

		Ok(regex.is_match(str::from_utf8(&stdout)?))
	}
}
