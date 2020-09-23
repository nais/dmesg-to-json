// Third-party imports
use anyhow::{anyhow, Result};
use humantime::Timestamp;
use regex::RegexBuilder;

// Local imports
use crate::structs::{KernelLineError, KernelLogLevel};

fn get_regex_match_by_name(
	captures: &regex::Captures,
	input_string: &str,
	pattern: &str,
	group_name: &str,
) -> Result<String> {
	match captures.name(group_name) {
		Some(regex_match) => Ok(regex_match.as_str().to_string()),
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
	let regex_pattern = r#"
            ^                       # Start of string _and_ line!
            (<(?P<level>\d)>)?      # Potential first match: The log level
            (?P<timestamp>
                \d{4}-\d{2}-\d{2}   # Timestamp: yyyy-mm-dd
                T                   # Timestamp: seperator
                \d{2}:\d{2}:\d{2}   # Timestamp: hours:minutes:seconds
            )
            ,                       # Timestamp: seperator
            \d+                     # Timestamp: milliseconds
            [+-]\d{4}               # Timestamp: timezone
            \s                      # A literal "<space>" separating timestampt and message
            (?P<message>.*)         # The actual log message
        "#;

	// Match regex pattern with something
	let regex = match RegexBuilder::new(regex_pattern)
        .ignore_whitespace(true) // This allows the multi-line regex pattern string
        .build()
	{
		Ok(regex) => regex,
		Err(e) => return Err(anyhow!("Error bubbling from regex::RegexBuilder:\n{}", e)),
	};
	let captures = match regex.captures(input_line) {
		Some(matches) => matches,
		None => {
			return Err(anyhow!(KernelLineError::NoRegexMatches {
				pattern: regex_pattern.to_string(),
				input: input_line.to_string(),
			}))
		}
	};

	// Extract named groups into variables
	let (level, timestamp, message) = (
		match get_regex_match_by_name(&captures, &input_line, &regex_pattern, "level") {
			Ok(lvl) => lvl,
			Err(_) => "6".to_string(),
		},
		get_regex_match_by_name(&captures, &input_line, &regex_pattern, "timestamp")?,
		get_regex_match_by_name(&captures, &input_line, &regex_pattern, "message")?,
	);
	let log_level = match level.as_str() {
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
	};
	let timing = parse_dmesg_iso_timestamp(&timestamp)?;
	Ok((log_level, timing, message))
}
