// Third-party imports
use anyhow::{anyhow, Result};
use humantime::Timestamp;

// Local imports
use crate::structs::{KernelLineError, KernelLogLevel};

lazy_static! {
	pub static ref KERNEL_LINE_REGEX: regex::Regex =
		match regex::RegexBuilder::new(KERNEL_LINE_REGEX_PATTERN)
			.ignore_whitespace(true)
			.build()
		{
			Err(e) => panic!(
				"Static variable KERNEL_LINE_REGEX unable to initialize!\nError:\n\t{}",
				e
			),
			Ok(regex) => regex,
		};
}
static KERNEL_LINE_REGEX_PATTERN: &'static str = r#"
    ^                       # Start of string _and_ line!
    (<(?P<level>\d)>)?      # Potential first match: The log level
    (?P<timestamp>
        \d{4}-\d{2}-\d{2}   # Timestamp: yyyy-mm-dd
        T                   # Timestamp: seperator
        \d{2}:\d{2}:\d{2}   # Timestamp: hours:minutes:seconds
    )
    ,                       # Timestamp: seperator
    \d+                     # Timestamp: milliseconds
    [+-]\d{2}:?\d{2}        # Timestamp: timezone
    \s                      # A literal "<space>" separating timestampt and message
    (?P<message>.*)         # The actual log message
"#;

fn get_regex_match_by_name<'a>(
	captures: &'a regex::Captures,
	input_string: &'a str,
	pattern: &str,
	group_name: &str,
) -> Result<&'a str> {
	match captures.name(group_name) {
		Some(regex_match) => Ok(regex_match.as_str()),
		None => {
			return Err(anyhow!(KernelLineError::MissingRegexGroupMatch {
				group_name: group_name.to_string(),
				pattern: pattern.to_string(),
				input: input_string.to_string(),
			}))
		}
	}
}

/// Parse `input_string` into a humantime::Timestamp, by leveraging humatime::parse_rfc3339_weak()
/// -> https://docs.rs/humantime/2.0.1/humantime/fn.parse_rfc3339_weak.html
pub fn parse_dmesg_iso_timestamp(input_string: &str) -> Result<Timestamp> {
	let timestamp: std::time::SystemTime = match humantime::parse_rfc3339_weak(input_string) {
		Ok(result) => result,
		Err(e) => {
			return Err(anyhow!(
				"Unexpected error occured when parsing timestamp!\n\t{}",
				e
			))
		}
	};
	Ok(Timestamp::from(timestamp))
}

/// Match input line from dmesg and parse its contents.
/// Expected string structure:
///     <log-level>?<yyyy-mm-ddTHH:MM:SS,<ms><timezone offset> <message>
pub fn get_log_line_regex_matches(input_line: &str) -> Result<(KernelLogLevel, Timestamp, String)> {
	let captures = match KERNEL_LINE_REGEX.captures(input_line) {
		Some(matches) => matches,
		None => {
			return Err(anyhow!(KernelLineError::NoRegexMatches {
				pattern: KERNEL_LINE_REGEX_PATTERN.to_string(),
				input: input_line.to_string(),
			}))
		}
	};

	// Extract named groups into variables
	let (timestamp, message, log_level) = (
		match get_regex_match_by_name(
			&captures,
			&input_line,
			&KERNEL_LINE_REGEX_PATTERN,
			"timestamp",
		) {
			Err(e) => return Err(e),
			Ok(string) => parse_dmesg_iso_timestamp(string)?,
		},
		get_regex_match_by_name(
			&captures,
			&input_line,
			&KERNEL_LINE_REGEX_PATTERN,
			"message",
		)?,
		match get_regex_match_by_name(&captures, &input_line, &KERNEL_LINE_REGEX_PATTERN, "level") {
			Ok(lvl) => match lvl {
				"0" => KernelLogLevel::Emergency,
				"1" => KernelLogLevel::Alert,
				"2" => KernelLogLevel::Critical,
				"3" => KernelLogLevel::Error,
				"4" => KernelLogLevel::Warning,
				"5" => KernelLogLevel::Notice,
				"6" => KernelLogLevel::Info,
				"7" => KernelLogLevel::Debug,
				x => {
					return Err(anyhow!(KernelLineError::UnexpectedKernelLogLevel {
						input: x.to_string()
					}))
				}
			},
			Err(_) => KernelLogLevel::Info,
		},
	);
	Ok((log_level, timestamp, message.to_owned()))
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn check_for_unexpected_kernel_loglevel() {
		let input = "8 2020-07-30T14:02:51,000000+00:00 blabla";
		match get_log_line_regex_matches(&input).map_err(|e| e) {
			Ok(_) => assert_eq!(true, false),
			Err(e) => assert_eq!(true, e.is::<crate::utils::KernelLineError>(),),
		};
	}
}
