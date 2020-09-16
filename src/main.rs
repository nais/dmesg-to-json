use std::error::Error;

use rmesg::{kernel_log_timestamps_enable, RMesgLinesIterator, SUGGESTED_POLL_INTERVAL};

fn main() -> Result<(), Box<dyn Error>> {
    kernel_log_timestamps_enable(true)?;
    let iterator = RMesgLinesIterator::with_options(false, SUGGESTED_POLL_INTERVAL)?;
    let (max_lines, mut counter) = (50, 0);
    for line in iterator {
        if counter >= max_lines {
            break;
        }
        println!("\nCurrent line:\n{}", &line?);
        counter += 1;
    }
    Ok(())
}
