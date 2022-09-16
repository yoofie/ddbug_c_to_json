#![allow(non_snake_case)]
#![allow(dead_code)]
#![warn(unused_imports)]
#![allow(unused_parens)]
#![allow(non_camel_case_types)]

use crate::support::value_to_u32;
use serde::{self, Serialize};
#[derive(Debug, Default, Serialize)]
pub struct BaseData {
	pub base_name: String,
	pub base_size: u32,
	pub base_encoding: String,
}

pub fn parse_base_data(buffer: &str) -> BaseData {
	let line_buffer = buffer.lines();
	let mut base_data = BaseData::default();

	for (indx, line) in line_buffer.enumerate() {
		let clean_line = line.replace("\t", "");

		if indx == 0 {
			if let Some((_key, value)) = clean_line.split_once(" ") {
				base_data.base_name = value.to_string()
			} else {
				base_data.base_name = "unparsed name".to_string()
			};
		} else {
			if let Some((key, value)) = clean_line.split_once(":") {
				match key {
					"base" => {
						base_data.base_name = value.trim().to_string();
					}
					"size" => {
						base_data.base_size = value_to_u32((value));
					}
					"encoding" => {
						base_data.base_encoding = value.trim().to_string();
					}

					_ => {}
				}
			}
		}

		//println!("{}", line);
	}
	base_data
}
