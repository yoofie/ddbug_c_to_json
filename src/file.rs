#![allow(non_snake_case)]
#![allow(dead_code)]
#![warn(unused_imports)]
#![allow(unused_parens)]
#![allow(non_camel_case_types)]

use crate::support::{self, value_to_u32};
use serde::{self, Serialize};
use std::ops::Index;
#[derive(Debug, Default, Serialize)]
struct Section {
	name: String,
	size: u32,
	start_address: String,
	end_address: String,
}

#[derive(Default, Serialize, Debug)]
pub struct FileData {
	filename: String,
	address_data: Vec<support::AddressData>,
	total_size: u32,
	var_size: u32,
	fn_size: u32,
	other_size: u32,
	sections: Vec<Section>,
}

pub fn parse_file_data(buffer: &str) -> FileData {
	let mut retval = FileData::default();
	let clean = buffer.replace("\t", "");
	//println!("\nPARSE DATA FILE\n\n\n\n{}", buffer);

	let file_data = &mut clean.split("\n");

	/* ********************************************************
		Get FileName
	******************************************************** */
	{
		if let Some(file_name) = file_data.nth(0) {
			/* let v                                       = file_name
			.split_ascii_whitespace()
			.nth(1)
			.unwrap_or_else(|| "Error 401: failed to parse file name"); */
			if let Some(val) = file_name.split_ascii_whitespace().nth(1) {
				retval.filename = val.to_string();
			} else {
				retval.filename = "Error: failed to parse filename".to_string();
			}
		}
	}
	/* ********************************************************
		Parse Addresses
	******************************************************** */
	let _addresses = file_data.nth(0).unwrap();

	println!("\n----\n\n");

	for item in file_data.into_iter() {
		if item.starts_with("0x") {
			let address_item = support::parse_address(&item);
			retval.address_data.push(address_item);
		} else {
			break;
		}
	}

	/* ********************************************************
		Fields
	******************************************************** */

	for item in file_data.into_iter() {
		let kv_pair: Vec<&str> = item.split(":").collect();
		let key = if kv_pair.len() > 0 {
			Some(kv_pair.index(0).trim())
		} else {
			None
		};
		let value = if kv_pair.len() > 1 {
			Some(kv_pair.index(1).trim()).unwrap_or_default()
		} else {
			""
		};

		if let Some(key) = key {
			match key {
				"size" => {
					retval.total_size = value_to_u32(value);
				}
				"fn size" => {
					retval.fn_size = value_to_u32(value);
				}
				"var size" => {
					retval.var_size = value_to_u32(value);
				}
				"other size" => {
					retval.other_size = value_to_u32(value);
				}
				"sections" => {
					//println!("Stop!");
					break;
				}
				_ => {
					println!("{key}")
				}
			}
		}
	}

	/* ********************************************************
		Parse File Sections
	******************************************************** */
	let mut ss: Vec<String> = Vec::new();
	let mut section_vec: Vec<Section> = Vec::new();
	file_data.next();
	let mut itr = file_data.peekable();

	while let Some(iix) = itr.next() {
		if iix.starts_with(".") {
			let x = parse_sections(&ss);
			section_vec.push(x);
			ss.clear();
			let v = iix.replace(".", "name:");
			ss.push(v);
		} else {
			ss.push(iix.to_string());
			if itr.peek().is_none() {
				let x = parse_sections(&ss);
				section_vec.push(x);
			}
		}
	}

	section_vec.remove(0);
	retval.sections = section_vec;
	retval
}

/* ********************************************************
	Functions
******************************************************** */
fn parse_sections(ss: &Vec<String>) -> Section {
	let mut section = Section::default();

	for z in ss {
		let kv_pair: Vec<&str> = z.split(":").collect();
		let key = if kv_pair.len() > 0 {
			Some(kv_pair.index(0).trim())
		} else {
			None
		};
		let value = if kv_pair.len() > 1 {
			Some(kv_pair.index(1).trim()).unwrap_or_default()
		} else {
			"0x0"
		};

		if let Some(mm) = key {
			//dbg!(mm);
			match mm {
				"name" => {
					section.name = value.to_string();
				}
				"size" => section.size = value_to_u32(value),
				"address" => {
					let address_range = value.to_string();

					(section.start_address, section.end_address) =
						support::parse_address_range(&address_range);
				}
				_ => {}
			}
		}
	}

	section
}
