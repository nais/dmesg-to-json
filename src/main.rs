use std::error::Error;

use rmesg::{kernel_log_timestamps_enable, RMesgLinesIterator, SUGGESTED_POLL_INTERVAL};
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

    /// Number of kernel log-lines to print
    #[structopt(short, long, default_value = "25", value_name = "lines_count")]
    max_log_lines: u32,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::from_args();
    if 2 <= args.verbosity_level {
        dbg!(&args);
    }
    kernel_log_timestamps_enable(true)?;
    let iterator = RMesgLinesIterator::with_options(false, SUGGESTED_POLL_INTERVAL)?;
    let mut counter = 0;
    for line in iterator {
        if counter >= args.max_log_lines {
            break;
        }
        println!("\nCurrent line:\n{}", &line?);
        counter += 1;
    }
    Ok(())
}
