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
	#[arg(short, action = clap::ArgAction::Count)]
	verbosity_level: u8,

	/// String to filter log line prefix with
	#[arg(short, long)]
	print_all: bool,
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
			eprintln!("Received line: {}", &line);
		}
		let kernel_line = structs::KernelLine::new(&mut line)?;
		if args.print_all {
			println!("{}", &kernel_line.message);
			continue;
		}
		if !args.print_all && !kernel_line.message.starts_with("naisdevice-fwd: ") {
			continue;
		}

		// Now we should only print dmesg lines starting with specified prefix as json
		let jsonified_logline =
			structs::IptablesLogLine::new(&kernel_line.message, &kernel_line.timestamp)?;
		println!("{}", serde_json::to_string(&jsonified_logline)?);
	}
	Ok(())
}
