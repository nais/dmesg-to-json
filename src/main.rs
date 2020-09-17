// Std-lib imports
use std::error::Error;
use std::io::{self, BufRead};

// Local imports
mod structs;

// Third-party imports
use structopt::{
    clap::AppSettings::{ColorAuto, ColoredHelp},
    StructOpt,
};

#[derive(Debug, StructOpt)]
#[structopt(setting(ColorAuto), setting(ColoredHelp), about)]
struct Cli {
    /// Verbosity level (-v, -vv, etc...)
    #[structopt(short, parse(from_occurrences))]
    verbosity_level: usize,

    /// String to filter log line prefix with
    #[structopt(short, long)]
    include_filter: Option<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::from_args();
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
        if let Some(prefix_filter) = &args.include_filter {
            if !kernel_line.message.starts_with(prefix_filter) {
                continue;
            }
        }
        println!("{}", &kernel_line.message);
    }
    Ok(())
}
