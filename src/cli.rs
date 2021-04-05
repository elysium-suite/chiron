use clap::Clap;

#[derive(Clap)]
pub struct Opts {
	/// The configuration file to read from.
	pub config: String,
}
