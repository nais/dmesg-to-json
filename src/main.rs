// Std-lib imports
use std::error::Error;
use std::io::{self, BufRead};

// Third-party imports
use clap::Parser;

// Local imports
mod structs;
mod utils;

#[derive(Debug, Parser)]
struct Cli {
	/// Verbosity level (-v, -vv, etc...)
	/// 2+: Print each line as its received to stderr, before attempted json transform
	#[arg(short, action = clap::ArgAction::Count)]
	verbosity_level: u8,

	/// String to filter log line prefix with
	#[arg(short, long, default_value = "naisdevice-fwd: ")]
	filter: String,
}

fn main() -> Result<(), Box<dyn Error>> {
	let args = Cli::parse();
	if 2 <= args.verbosity_level {
		dbg!(&args);
	}

	let stdin = io::stdin();
	for input_line in stdin.lock().lines() {
		let mut line = input_line?;
		if 2 < args.verbosity_level {
			eprintln!("{line}");
		}

		let kernel_line = structs::KernelLine::new(&mut line)?;
		if !kernel_line.message.starts_with(&args.filter) {
			continue;
		}

		// Now we should only print dmesg lines starting with specified prefix as json
		let jsonified_logline =
			structs::IptablesLogLine::new(&kernel_line.message, &kernel_line.timestamp)?;
		println!("{}", serde_json::to_string(&jsonified_logline)?);
	}
	Ok(())
}
