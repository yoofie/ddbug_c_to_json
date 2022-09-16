#![allow(non_snake_case)]
#![allow(dead_code)]
#![warn(unused_imports)]
#![allow(unused_parens)]
#![allow(non_camel_case_types)]

use std::path::PathBuf;

use crate::support::{get_line_number_and_column, value_to_u32};
use serde::{self, Serialize};
#[derive(Debug, Default, Serialize, PartialEq)]
pub struct StructData {
	pub s_name: String,
	pub s_members: Vec<StructMemberData>,
	pub s_full_file_path: String,
	pub s_line_num: u32,
	pub s_line_col: u32,
	pub s_size: u32,
}

#[derive(Debug, PartialEq, Default, Serialize)]
pub struct StructMemberData {
	pub sm_name: String,
	pub sm_type: String,
	pub sm_running_total: u32,
	pub sm_size: u32,
}

pub fn parse_struct_data(buffer: &str) -> StructData {
	let mut line_buffer = buffer.lines();
	let mut retval = StructData::default();
	//let clean_line = line_buffer.replace("\t", "");

	if let Some(struct_name) = line_buffer.nth(0) {
		if let Some(val) = struct_name.split_ascii_whitespace().nth(1) {
			retval.s_name = val.to_string();
		} else {
			retval.s_name = "Error: failed to parse struct name".to_string();
		}
	}

	for line in line_buffer {
		let clean_line = line.replace("\t", "");
		if let Some((key, value)) = clean_line.split_once(":") {
			match key {
				"source" => {
					let file_path = PathBuf::from(value);

					if let Some(the_file_name) = file_path.file_name() {
						//let (Some(line), Some(column)): (Option<u32>, Option<u32>) =
						//get_line_number_and_column(the_file_name);

						match get_line_number_and_column(the_file_name) {
							(Some(line), Some(column)) => {
								retval.s_line_num = line;
								retval.s_line_col = column
							}
							_ => {}
						}

						retval.s_full_file_path = file_path.to_string_lossy().trim().to_string();
					}
				}

				"members" => {
					let bfr = buffer.clone();
					retval.s_members = handle_members(&bfr);
				}
				"size" => {
					let the_size = value_to_u32((value));
					retval.s_size = the_size;
				}
				_ => {}
			}
		}
	}

	retval
}

fn handle_members(input: &str) -> Vec<StructMemberData> {
	//let mut line_buffer = input.lines();
	let mut sm_data: Vec<StructMemberData> = Vec::new();

	let mut member_data = input.replace("]", "|").replace("[", "|").replace("\t", "");
	if let Some(omg) = input.find("s:") {
		let fields: String = member_data.drain(omg..).collect();
		let mut bb = fields.lines();
		bb.next();
		for x in bb {
			let mut data = StructMemberData::default();
			let zz: Vec<_> = x.split(&['|', ':']).collect();
			if !zz.is_empty() {
				data.sm_running_total = if let Some(txt_value) = zz.iter().nth(0) {
					value_to_u32((txt_value))
				} else {
					0
				};

				data.sm_size = if let Some(txt_value) = zz.iter().nth(1) {
					value_to_u32((txt_value))
				} else {
					0
				};

				data.sm_name = if let Some(txt_value) = zz.iter().nth(2) {
					txt_value.to_string()
				} else {
					"[ERROR 545] Failed to parse field member name \"".to_string()
				};

				/* data.sm_name = if let Some((_k, v)) = x.split_once(" ") {
					v.trim().to_string()
				} else {
					dbg!(x);
					"[ERROR 444] Failed to parse struct name".to_string()
				}; */

				data.sm_type = if let Some(txt_value) = zz.iter().nth(3) {
					if txt_value.trim_start().to_string().contains("<padding>") {
						"<padding>".to_string()
					} else {
						txt_value.trim_start().to_string()
					}
				} else {
					"".to_string()
				};
				sm_data.push(data);
			}
		}
	}

	sm_data
}
