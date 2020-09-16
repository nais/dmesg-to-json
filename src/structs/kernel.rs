use std::error::Error;
use std::fmt;

use num_derive::{FromPrimitive, ToPrimitive};
use regex::Regex;

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

impl fmt::Display for KernelLogLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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

#[derive(Debug)]
pub struct KernelLine {
    pub log_level: KernelLogLevel,
    pub timestamp: String,
    pub message: String,
}

impl KernelLine {
    pub fn new(line: &str) -> Result<KernelLine, Box<dyn Error>> {
        // regex parse log_level, timestamp, and message from line
        // TODO: Replace with some lazy_static solution to avoid re-compiling regex each time
        let regex = Regex::new(r"^<(?P<level>\d)>\[\s+(?P<timestamp>\d+\.\d+)\] (?P<message>.*)$")?;
        let mut captures: Vec<&str> = vec![];
        if let Some(capture_groups) = regex.captures(line) {
            captures = capture_groups
                .iter()
                .skip(1) // First element is always whole string matched, not specific groups
                .filter(|s| s.is_some())
                .map(|s| s.unwrap().as_str())
                .collect();
        }
        if captures.len() != 3 {
            panic!(
                "Syslog/kernel log line does not match expected regex!{}",
                format!("\nExpected:\n\t{}\n\nReceived:\n\t{}", regex, line)
            );
        }

        let result = KernelLine {
            timestamp: captures[1].to_string(),
            message: captures[2].to_string(),
            log_level: match captures[0] {
                "0" => KernelLogLevel::Emergency,
                "1" => KernelLogLevel::Alert,
                "2" => KernelLogLevel::Critical,
                "3" => KernelLogLevel::Error,
                "4" => KernelLogLevel::Warning,
                "5" => KernelLogLevel::Notice,
                "6" => KernelLogLevel::Info,
                "7" => KernelLogLevel::Debug,
                _ => panic!(
                    "Unable to match syslog/kernel log-level with int from 0 to (and including) 7{}",
                    format!("\nExpected:\n\t{}\n\nReceived:\n\t{}", regex, line)
                ),
            },
        };
        Ok(result)
    }
}

impl fmt::Display for KernelLine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "KernelLine {{ log_level: {}, timestamp: {}, message: \"\"\"{}\"\"\" }}",
            self.log_level, self.timestamp, self.message,
        )
    }
}
