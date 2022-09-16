#![allow(non_snake_case)]
#![allow(dead_code)]
#![warn(unused_imports)]
#![allow(unused_parens)]
#![allow(non_camel_case_types)]

use crate::{
	base::*,
	enums::EnumData,
	ftype::*,
	function::FnData,
	gvar::GlobalVarData,
	structure::StructData,
	support::{self, value_to_u32},
};
use serde::{self, Serialize};
use std::path::PathBuf;

#[derive(Debug, Default, Serialize)]
pub struct UnitData {
	pub file_name: String,
	pub file_path: String,
	pub addresses: Vec<support::AddressData>,
	pub unknown_addresses: Option<Vec<support::AddressData>>,
	pub function_size: u32,
	pub var_size: u32,
	pub unknown_size: u32,
}
#[derive(Debug, Default, Serialize)]
pub struct TranslationUnitData {
	pub unit_data: UnitData,
	pub fn_data: Vec<FnData>,
	pub base_data: Vec<BaseData>,
	pub ftype_data: Vec<FtypeData>,
	pub struct_data: Vec<StructData>,
	pub g_var_data: Vec<GlobalVarData>,
	pub enum_data_v: Vec<EnumData>,
}

pub fn parse_unit(buffer: &str) -> UnitData {
	let mut retval = UnitData::default();

	let clean = buffer.replace("\t", "");
	//println!("\nPARSE Unit Data\n\n{}", buffer);

	let unit_data = &mut clean.split("\n");

	/* Get unit Name
		******************************************************** */
	if let Some(kv) = unit_data.next() {
		if let Some(val) = kv.split_ascii_whitespace().nth(1) {
			retval.file_path = val.to_string();
			let pathh = PathBuf::from(val);

			if let Some(the_file_name) = pathh.file_name() {
				retval.file_name = the_file_name.to_string_lossy().to_string();
			}
		}
	}
	unit_data.next();

	/* Address Ranges
		******************************************************** */
	for item in unit_data.into_iter() {
		if item.starts_with("0x") {
			let address_item = support::parse_address(&item);
			retval.addresses.push(address_item);
		} else {
			break;
		}
	}

	/* Unknown Address ranges
		******************************************************** */
	let mut unknown_addr: Vec<support::AddressData> = Vec::new();
	for item in unit_data.into_iter() {
		if item.starts_with("0x") {
			let address_item = support::parse_address(&item);

			unknown_addr.push(address_item);
		} else {
			break;
		}
	}

	retval.unknown_addresses = if !unknown_addr.is_empty() {
		Some(unknown_addr)
	} else {
		None
	};

	//dbg!(&unit_data);
	/* Size fields
		******************************************************** */
	//TODO: We skip 1 unintentionally
	for item in unit_data.into_iter() {
		if let Some((k, v)) = item.split_once(":") {
			match k {
				"fn size" => {
					retval.function_size = value_to_u32(v);
				}
				"var size" => {
					retval.var_size = value_to_u32(v);
				}
				"unknown size" => {
					retval.unknown_size = value_to_u32(v);
				}
				_ => {}
			}
		}
	}

	retval
}
