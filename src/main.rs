#![allow(non_snake_case)]
#![allow(dead_code)]
#![warn(unused_imports)]
#![allow(unused_parens)]
#![allow(non_camel_case_types)]

use std::{
	fs::{self, File},
	io::{self, BufRead},
	ops::{Index, Range},
	path::{Path, PathBuf},
};

mod base;
mod enums;
mod file;
mod ftype;
mod function;
mod gvar;
mod structure;
mod support;
mod unit;
use crate::{
	structure::StructData,
	unit::{parse_unit, TranslationUnitData},
};
use base::BaseData;
use enums::EnumData;
use ftype::FtypeData;
use function::FnData;
use gvar::GlobalVarData;
use serde::Serialize;
use unit::UnitData;

use clap::{arg, value_parser, Command};
fn main() {
	let cmd_line = Command::new("DDbug C to JSON")
		.version("0.1")
		.author("Yoofie <yoofie@gmail.com>")
		.about("This project exists to take ddbug output text files (based on C source code) and convert them to Rust structures so that you can create your own custom programs based on DWARF file data.

With the `ddbug` parser output files serialized to Rust Structs, you can also use serde to serialize the data to machine parsable JSON format.")
		.arg(arg!(-i --input <INPUT_FILE> "The input file into this tool. This file should have been generated from the ddbug tool").value_parser(value_parser!(PathBuf)))
		.arg(arg!(-o --output [OUTPUT_FILE] "The name of the generated output file. ").value_parser(value_parser!(PathBuf)).default_value("output.json"))

		.arg(arg!(-d --rdbg "Prints out the internal Rust structures").takes_value(false))
		.get_matches();

	let input_file = cmd_line.get_one::<PathBuf>("input").expect("Required");
	let output_file = cmd_line.get_one::<PathBuf>("output").expect("Required");
	/* ********************************************************
		Welcome Message
	******************************************************** */
	println!(
		"Hello!\nUsing \"{}\" as input.\n\"{}\" as output file.",
		input_file.to_string_lossy(),
		output_file.to_string_lossy(),
	);

	/* ********************************************************
		So it begins()
	******************************************************** */
	let mut translation_units: Vec<TranslationUnitData> = Vec::new();
	start_parsing(&mut translation_units, &input_file);

	/* ********************************************************
		Ouput based on cmd line options
	******************************************************** */

	if cmd_line.is_present("rdbg") {
		println!("{:#?}", translation_units);
	}

	/* ********************************************************
		JSON output
	******************************************************** */
	// Normal "two spaces" indentation string. This blashphemy is painful to the eyes
	//let serialized = serde_json::to_string_pretty(&translation_units).unwrap();

	let output_json_buffer = Vec::new();

	// Define normal tabs as our indentation, bringing sanity back to the world
	let formatter = serde_json::ser::PrettyFormatter::with_indent(b"\t");
	let mut ser = serde_json::Serializer::with_formatter(output_json_buffer, formatter);
	translation_units.serialize(&mut ser).unwrap();
	//let output_pretty_json = String::from_utf8(ser.into_inner()).unwrap();

	if let Ok(output_pretty_json) = String::from_utf8(ser.into_inner()) {
		fs::write(output_file, output_pretty_json).expect("Error: Unable to write file.");
		println!(
			"\"{}\" written successfully!",
			output_file.to_string_lossy()
		);
	} else {
		"Failed to print UTF8 pretty JSON :(".to_string();
	}
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
	P: AsRef<Path>,
{
	let file = File::open(filename)?;
	Ok(io::BufReader::new(file).lines())
}

fn get_group_range(input: &[usize]) -> Range<usize> {
	let start = *input.index(0);
	let end = if input.len() > 1 {
		*input.index(1)
	} else {
		input.len() - 1
	};

	Range {
		start: start,
		end: end,
	}
}

fn start_parsing(translation_units: &mut Vec<TranslationUnitData>, input_file: &PathBuf) {
	let lines = match read_lines(&input_file.as_os_str()) {
		Ok(it) => {
			//println!("{:?}", it);
			it.into_iter()
		}
		_ => return,
	};

	let mut buffr = String::new();
	let mut vBuffer: Vec<String> = Vec::new();
	vBuffer.reserve(2048);
	for (indx, i) in lines.enumerate() {
		if let Ok(the_line) = i {
			if the_line.is_empty() {
				//println!("New Line starts {indx}");
				vBuffer.push(buffr.clone());
				buffr.clear();
			} else {
				let newLineBuffr = the_line + "\n";
				buffr.push_str(newLineBuffr.as_str())
			}
		} else {
			println!("Failed to parse line {indx}")
		}
	}

	/* ********************************************************
		Create Unit Index
	******************************************************** */
	let mut uuIndex: Vec<usize> = Vec::new();
	for (indx, item) in vBuffer.iter().enumerate() {
		if let Some((key, _val)) = item.split_once(" ") {
			match key {
				"unit" => uuIndex.push(indx),
				_ => {}
			}
		} else {
		}
	}

	/* ********************************************************
		Group the string buffer into groups of translation units
	******************************************************** */
	let mut compilation_unit_group: Vec<usize> = Vec::new();

	/* Create the actual groups, saved the buffer indexes */
	for (indx, item) in vBuffer.iter().enumerate() {
		if item.starts_with("unit") || item.starts_with("unit<") {
			compilation_unit_group.push(indx);
		}
	}

	/* Now that we have indexes for each translation unit, we can parse the data */
	for x in compilation_unit_group.windows(2) {
		/* Seperate the actual translation unit data (unit <filename>) */
		let (translation_unit_data) = get_group_range(x);

		// Grab the first entry "unit <filename>"
		// Note: This can be made more efficient. I am fully aware that Im doign heap allocations in a hot loop
		let mut fn_data_vec: Vec<FnData> = Vec::new();
		let mut g_var_data_vec: Vec<GlobalVarData> = Vec::new();
		let mut base_data_vec: Vec<BaseData> = Vec::new();
		let mut ftype_data_vec: Vec<FtypeData> = Vec::new();
		let mut unit_data = UnitData::default();
		let mut struct_data_vec: Vec<StructData> = Vec::new();
		let mut enum_data_vector: Vec<EnumData> = Vec::new();
		/* Now iterate through each translation_unit */
		for n in translation_unit_data {
			if n < vBuffer.len() {
				let buffer = vBuffer.index(n);

				let target = buffer.split_ascii_whitespace();
				for data_chunk in target {
					match data_chunk {
						"file" => {
							let _file_data = file::parse_file_data(buffer);
							break;
						}
						"fn" => {
							let fn_data = function::parse_fn_data(buffer);
							fn_data_vec.push(fn_data);
							break;
						}
						"base" => {
							let parsed_base_data = base::parse_base_data(buffer);
							base_data_vec.push(parsed_base_data);
							break;
						}
						"type" => {
							let parsed_ftype_data = ftype::parse_type_data(buffer);
							ftype_data_vec.push(parsed_ftype_data);
							break;
						}
						"struct" => {
							let parsed_struct_data = structure::parse_struct_data(buffer);
							struct_data_vec.push(parsed_struct_data);
							break;
						}
						"enum" => {
							let enum_data = enums::parse_enum_data(buffer);
							enum_data_vector.push(enum_data);
							break;
						}
						"var" => {
							let g_var_data = gvar::parse_g_var_data(buffer);
							g_var_data_vec.push(g_var_data);
							//println!("function");
							break;
						}
						"unit" => {
							unit_data = parse_unit(buffer);
							break;
						}
						_ => {
							break;
						}
					}
				}
			}
		}
		let translation_unitx = TranslationUnitData {
			unit_data: unit_data,
			fn_data: fn_data_vec,
			base_data: base_data_vec,
			ftype_data: ftype_data_vec,
			struct_data: struct_data_vec,
			g_var_data: g_var_data_vec,
			enum_data_v: enum_data_vector,
		};
		translation_units.push(translation_unitx);
	}
}
