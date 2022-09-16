#![allow(non_snake_case)]
#![allow(dead_code)]
#![warn(unused_imports)]
#![allow(unused_parens)]
#![allow(non_camel_case_types)]

use std::path::PathBuf;

use crate::support::{get_line_number_and_column, value_to_u32};
use serde::{self, Serialize};

#[derive(Debug, Default, Serialize)]
pub struct FtypeData {
	pub type_name: String,
	pub type_base_type: String,
	pub type_full_file_path: String,
	pub type_line_num: u32,
	pub type_line_col: u32,
	pub type_size: u32,
	pub type_members: Option<Vec<StructMemberData>>,
}

#[derive(Debug, Default, Serialize)]
pub struct StructMemberData {
	pub sm_name: String,
	pub sm_type: String,
	pub sm_running_total: u32,
	pub sm_size: u32,
}

pub fn parse_type_data(buffer: &str) -> FtypeData {
	let line_buffer = buffer.lines();
	let mut retval = FtypeData::default();

	for (indx, line) in line_buffer.enumerate() {
		let clean_line = line.replace("\t", "");

		if indx == 0 {
			if let Some((_key, value)) = clean_line.split_once(" ") {
				retval.type_base_type = if let Some((the_clean_name, v)) = value.split_once("=") {
					retval.type_name = the_clean_name.trim().to_string();
					v.trim().to_string()
				} else {
					retval.type_name = value.to_string();
					"Failed to parse the real underlying type".to_string()
				};
			} else {
				retval.type_name = "unparsed name".to_string()
			};
		} else {
			if let Some((key, value)) = clean_line.split_once(":") {
				match key {
					"source" => {
						let ftype_file_path = PathBuf::from(value);

						if let Some(the_file_name) = ftype_file_path.file_name() {
							match get_line_number_and_column(the_file_name) {
								(Some(line), Some(column)) => {
									retval.type_line_num = line;
									retval.type_line_col = column
								}
								// If we dont have a line number, then we dont have a column number either
								_ => {}
							}

							retval.type_full_file_path =
								ftype_file_path.to_string_lossy().trim().to_string();
						}
					}
					"size" => {
						retval.type_size = value_to_u32((value));
					}
					"members" => {
						let bfr = buffer.clone();
						retval.type_members = handle_members(&bfr);
					}

					_ => {}
				}
			}
		}

		//println!("{}", line);
	}
	retval
}

fn handle_members(input: &str) -> Option<Vec<StructMemberData>> {
	let mut sm_data: Vec<StructMemberData> = Vec::new();

	let mut member_data = input.replace("\t", "");
	if let Some(omg) = input.find("s:") {
		let fields: String = member_data.drain(omg..).collect();
		let member_fields = fields.lines();

		for current_member_field in member_fields {
			let mut data = StructMemberData::default();
			//let zz: Vec<_> = current_member_field.split(&['|', ':']).collect();

			// Split member into a KV pair
			if let Some((member_sizes, member_type)) = current_member_field.split_once(":") {
				// Grab the type right away
				data.sm_type = member_type.trim().to_string();

				// Decode running total and current member data size
				(data.sm_running_total, data.sm_size) =
					if let Some((member_size_buffer, member_name)) = member_sizes.split_once("]") {
						data.sm_name = member_name.to_string();

						// extract running total and current size as tuple
						let retval_tuple = if let Some((running_total, current_member_size)) =
							member_size_buffer.split_once("[")
						{
							(
								value_to_u32((running_total)),
								value_to_u32((current_member_size)),
							)
						} else {
							// Return zeroes if we failed to get a tuple
							(0, 0)
						};
						retval_tuple
					} else {
						(0, 0)
					};
			};

			sm_data.push(data);
			//println!("{}\n\n", z.);
		}
	}

	if !sm_data.is_empty() {
		Some(sm_data)
	} else {
		None
	}
}
