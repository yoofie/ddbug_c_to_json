use serde::{self, Serialize};
use std::{ffi::OsStr, ops::Range};
#[derive(Debug, Default, Serialize)]

pub struct AddressData {
	pub start_address: String,
	pub end_address: String,
	pub total_size: u32,
}
#[derive(Debug, Default, Serialize)]
pub struct ReturnType {
	pub return_type: String,
	pub return_size: u32,
}

/* ********************************************************
	Parses address range
	like "0x13dc-0x1657"
******************************************************** */
pub fn parse_address_range(input: &String) -> (String, String) {
	let mut split_val = input.split("-");
	let start = if let Some(start_val) = split_val.next() {
		start_val.trim().to_string()
	} else {
		"Invalid".to_string()
	};
	let end = if let Some(end_val) = split_val.next() {
		end_val.trim().to_string()
	} else {
		"Invalid".to_string()
	};
	(start, end)
}

/* ********************************************************
	Parse address range
	Example: 0x3cc-0x5c1 (502)
******************************************************** */
pub fn parse_address(input_buffer: &str) -> AddressData {
	let mut retval = AddressData::default();

	let txt_value = input_buffer.split_ascii_whitespace().nth(1).unwrap();
	let remove_prefix = txt_value.strip_prefix("(").unwrap();
	let int_value = remove_prefix.strip_suffix(")").unwrap();
	retval.total_size = int_value.parse::<u32>().unwrap_or_default();

	let mut start_end_addr = input_buffer.split("-");
	retval.start_address = start_end_addr.next().unwrap().to_string();
	retval.end_address = start_end_addr
		.next()
		.unwrap()
		.strip_suffix(txt_value)
		.unwrap()
		.trim_end()
		.to_string();

	//println!("{:?}", retval);
	retval
}

pub fn value_to_u32(input: &str) -> u32 {
	let retval = match input.trim().parse::<u32>() {
		Ok(v) => v,

		_ => 0xFD,
	};
	retval
}
pub fn value_to_i32(input: Option<&str>) -> i32 {
	if let Some(val) = input {
		return match val.trim().parse::<i32>() {
			Ok(v) => v,

			_ => 0xFD,
		};
	} else {
		return 0xFF;
	}
}

fn return_range(s: usize, e: usize) -> Range<usize> {
	Range { start: s, end: e }
}

pub fn get_line_number_and_column(input: &OsStr) -> (Option<u32>, Option<u32>) {
	let string = input.to_str().to_owned().unwrap();
	let mut split_string = string.trim().split(":");

	let row = if let Some(x) = split_string.nth(1) {
		Some(value_to_u32((x)))
	} else {
		None
	};
	//split_string.next();
	let column = if let Some(x) = split_string.last() {
		Some(value_to_u32((x)))
	} else {
		None
	};
	//dbg!(split_string);
	(row, column)
}
