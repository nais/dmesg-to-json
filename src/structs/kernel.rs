// Third-party imports
use anyhow::Result;
use num_derive::{FromPrimitive, ToPrimitive};
use thiserror::Error;

// Local imports
use crate::utils;

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
	fn fmt(
		&self,
		f: &mut std::fmt::Formatter,
	) -> std::fmt::Result {
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

#[derive(Debug)]
pub struct KernelLine {
	pub log_level: KernelLogLevel,
	pub timestamp: humantime::Timestamp,
	pub message: String,
}

impl KernelLine {
	/// Regex parse log_level, timestamp, and message from log line
	pub fn new(line: &str) -> Result<KernelLine> {
		let (log_level, timestamp, message) = utils::get_log_line_regex_matches(line)?;

		let result = KernelLine {
			timestamp: timestamp,
			message: message,
			log_level: log_level,
		};
		Ok(result)
	}
}

impl std::fmt::Display for KernelLine {
	fn fmt(
		&self,
		f: &mut std::fmt::Formatter,
	) -> std::fmt::Result {
		write!(
			f,
			"{}::[{}] {}",
			self.log_level, self.timestamp, self.message,
		)
	}
}
