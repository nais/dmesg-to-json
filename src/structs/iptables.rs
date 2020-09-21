// Std-lib imports
use std::collections::HashMap;
use std::net::Ipv4Addr;
use std::time::SystemTime;

// Third-party imports
use anyhow::{anyhow, Result};
use humantime::format_rfc3339;
use humantime_serde::Serde;
use itertools::Itertools;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct IptablesLogLine {
	pub time_stamp: Serde<SystemTime>,
	pub source_ip: Ipv4Addr,
	pub destination_ip: Ipv4Addr,
	pub log_type: String,
}

impl IptablesLogLine {
	pub fn new(
		input_string: &str,
		timestamp: &SystemTime,
	) -> Result<IptablesLogLine> {
		let (iptables_prefix, body) = match input_string.splitn(2, ": ").next_tuple() {
			Some(tuple) => tuple,
			None => {
				return Err(anyhow!(
					"Unable to find split dmesg-line by ': ':\n\n{:?}",
					input_string
				))
			}
		};
		let mut parse_map: HashMap<String, String> = body
			.split(" ")
			.flat_map(|pair| pair.split("="))
			.map(|s| s.to_string())
			.tuples()
			.collect();
		// Necessary to duplicate map because rust as a programming language are still waiting IndexMut
		// trait, ref:
		parse_map.insert("log-type".to_string(), iptables_prefix.to_string());
		parse_map.insert(
			"timestamp".to_string(),
			format_rfc3339(*timestamp).to_string(),
		);
		let result: IptablesLogLine = self::IptablesLogLine {
			timestamp: Serde::from(timestamp.to_owned()),
			log_type: iptables_prefix.to_owned(),
			source_ip: parse_map["SRC"].parse()?,
			destination_ip: parse_map["DST"].parse()?,
		};
		Ok(result)
	}
}
