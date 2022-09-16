#![allow(non_snake_case)]
#![allow(dead_code)]
#![warn(unused_imports)]
#![allow(unused_parens)]
#![allow(non_camel_case_types)]

use crate::support::{get_line_number_and_column, value_to_i32, value_to_u32};
use serde::{self, Serialize};

use std::{collections::HashMap, path::PathBuf};

#[derive(Debug, PartialEq, Default, Serialize)]
pub struct EnumData {
	pub enum_name: String,
	pub enum_full_file_path: String,
	pub enum_line_num: u32,
	pub enum_line_col: u32,
	pub enum_size: u32,
	pub enumerators: HashMap<String, i32>,
}

pub fn parse_enum_data(buffer: &str) -> EnumData {
	let mut line_buffer = buffer.lines();
	let mut retval = EnumData::default();

	//let clean_line = line_buffer.replace("\t", "");

	if let Some(struct_name) = line_buffer.nth(0) {
		if let Some(val) = struct_name.split_ascii_whitespace().nth(1) {
			retval.enum_name = val.to_string();
		} else {
			retval.enum_name = "Error: failed to parse struct name".to_string();
		}
	}
	for line in line_buffer {
		let clean_line = line.replace("\t", "");
		if let Some((key, value)) = clean_line.split_once(":") {
			match key {
				"source" => {
					let file_path = PathBuf::from(value);

					if let Some(the_file_name) = file_path.file_name() {
						match get_line_number_and_column(the_file_name) {
							(Some(line), Some(column)) => {
								retval.enum_line_num = line;
								retval.enum_line_col = column
							}
							_ => {}
						}
						retval.enum_full_file_path = file_path.to_string_lossy().trim().to_string();
					}
				}

				"enumerators" => {
					let bfr = buffer.clone();
					retval.enumerators = handle_enums(&bfr);
				}
				"size" => {
					let the_size = value_to_u32((value));
					retval.enum_size = the_size;
				}
				_ => {}
			}
		}
	}

	retval
}

fn handle_enums(input: &str) -> HashMap<String, i32> {
	let mut enumerators: HashMap<String, i32> = HashMap::new();
	let mut member_data = input.replace("\t", "");
	if let Some(omg) = input.find("s:") {
		let enum_line: String = member_data.drain(omg..).collect();
		let bb = enum_line.lines();

		for x in bb {
			let current_clean_line = x.replace(")", "").replace("(", " ");
			if let Some((key, value)) = current_clean_line.split_once(" ") {
				let number = value_to_i32(Some(value));

				enumerators.insert(key.to_string(), number);
			}
		}
	}
	enumerators
}
