// Third-party imports
use anyhow::{anyhow, Result};
use num_derive::{FromPrimitive, ToPrimitive};
use regex::RegexBuilder;
use thiserror::Error;

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum KernelLogLevel {
    Emergency = 0,
    Alert = 1,
    Critical = 2,
    Error = 3,
    Warning = 4,
    Notice = 5,
    Info = 6,
    Debug = 7,
}

impl std::fmt::Display for KernelLogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            KernelLogLevel::Emergency => write!(f, "DebugEmergency"),
            KernelLogLevel::Alert => write!(f, "Alert"),
            KernelLogLevel::Critical => write!(f, "Critical"),
            KernelLogLevel::Error => write!(f, "Error"),
            KernelLogLevel::Warning => write!(f, "Warning"),
            KernelLogLevel::Notice => write!(f, "Notice"),
            KernelLogLevel::Info => write!(f, "Info"),
            KernelLogLevel::Debug => write!(f, "Debug"),
        }
    }
}

#[derive(Error, Debug)]
pub enum KernelLineError {
    #[error("Regex matched zero groups\nInput:\n\t{input:?}\nPattern\n\t{pattern:?}")]
    NoRegexMatches { input: String, pattern: String },
    #[error("Missing expected regex group\nInput:\n\t{input:?}\nPattern\n\tr'{pattern:?}'\nGroup name:\n\t'{group_name:?}'")]
    MissingRegexGroupMatch {
        input: String,
        pattern: String,
        group_name: String,
    },
    #[error("Unexpected kernel log-level: {input:?}")]
    UnexpectedKernelLogLevel { input: String },
}

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
fn parse_dmesg_iso_timestamp(input_string: &str) -> Result<humantime::Timestamp> {
    let timestamp: std::time::SystemTime = match humantime::parse_rfc3339_weak(input_string) {
        Ok(result) => result,
        Err(e) => {
            return Err(anyhow!(
                "Unexpected error occured when parsing timestamp!\n\t{}",
                e
            ))
        }
    };
    Ok(humantime::Timestamp::from(timestamp))
}

/// Match input line from dmesg and parse its contents.
/// Expected string structure:
///     <log-level>?<yyyy-mm-ddTHH:MM:SS,<ms><timezone offset> <message>
fn get_log_line_regex_matches(
    input_line: &str,
) -> Result<(KernelLogLevel, humantime::Timestamp, String)> {
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

#[derive(Debug)]
pub struct KernelLine {
    pub log_level: KernelLogLevel,
    pub timestamp: humantime::Timestamp,
    pub message: String,
}

impl KernelLine {
    /// Regex parse log_level, timestamp, and message from log line
    pub fn new(line: &str) -> Result<KernelLine> {
        let (log_level, timestamp, message) = get_log_line_regex_matches(line)?;

        let result = KernelLine {
            timestamp: timestamp,
            message: message,
            log_level: log_level,
        };
        Ok(result)
    }
}

impl std::fmt::Display for KernelLine {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}::[{}] {}",
            self.log_level, self.timestamp, self.message,
        )
    }
}
