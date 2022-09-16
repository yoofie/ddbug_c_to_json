#![allow(non_snake_case)]
#![allow(dead_code)]
#![warn(unused_imports)]
#![allow(unused_parens)]
#![allow(non_camel_case_types)]
use std::path::PathBuf;

use serde::{self, Serialize};

use crate::support::{get_line_number_and_column, value_to_u32};

#[derive(Debug, Default, Serialize)]
pub struct GlobalVarData {
	g_var_name: String,
	g_var_type: String,
	g_var_full_file_path: String,
	g_var_line_num: u32,
	g_var_line_col: u32,
	g_var_size: u32,
	g_var_decl: bool,
}

pub fn parse_g_var_data(buffer: &str) -> GlobalVarData {
	let mut line_buffer = buffer.lines();
	let mut retval = GlobalVarData::default();

	if let Some(name) = line_buffer.nth(0) {
		if let Some((k, v)) = name.split_once(":") {
			retval.g_var_name = k.replace("var ", "").to_string();
			retval.g_var_type = v.trim().to_string();
		} else {
			retval.g_var_name = "Error: failed to global/static variable".to_string();
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
								retval.g_var_line_num = line;
								retval.g_var_line_col = column
							}
							_ => {}
						}
						retval.g_var_full_file_path =
							file_path.to_string_lossy().trim().to_string();
					}
				}

				"declaration" => {
					retval.g_var_decl = true;
				}
				"size" => {
					let the_size = value_to_u32((value));
					retval.g_var_size = the_size;
				}
				_ => {}
			}
		}
	}

	retval
}
