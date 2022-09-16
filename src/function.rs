#![allow(non_snake_case)]
#![allow(dead_code)]
#![warn(unused_imports)]
#![allow(unused_parens)]
#![allow(non_camel_case_types)]

use crate::support::{
	get_line_number_and_column, parse_address_range, value_to_i32, value_to_u32, AddressData,
	ReturnType,
};
use serde::{self, Serialize};
use std::path::PathBuf;
#[derive(Debug, Default, Serialize)]
pub struct FnVariableData {
	fn_var_name: String,
	fn_var_type: Option<String>,
	fn_var_size: u32,
	fn_var_stack_frame: i32,
}
#[derive(Debug, Default, Serialize)]
pub struct FnParamData {
	fn_param_name: String,
	fn_param_type: String,
	fn_param_size: u32,
	fn_param_stack_frame: i32,
}
#[derive(Debug, Default, Serialize)]
pub struct FnStackFrame {
	fn_stack_frame_name: Option<String>,
	fn_stack_frame_type: Option<String>,
	fn_stack_frame_size: u32,
	fn_stack_frame_stack_offset: i32,
}
#[derive(Debug, Default, Serialize)]
pub struct FnInlinedFunctions {
	inlined_fn_name: Option<String>,
	inlined_fn_size: u32,
	inlined_fn_call_source_path: Option<String>,
	line_num: Option<u32>,
	line_col: Option<u32>,
}
#[derive(Debug, Default, Serialize)]
pub struct FnData {
	name: String,
	full_file_path: Option<String>,
	linkage_name: Option<String>,
	line_num: Option<u32>,
	line_col: Option<u32>,
	address: AddressData,
	total_size: u32,
	return_data: Option<ReturnType>,
	fn_params: Option<Vec<FnParamData>>,
	fn_vars: Option<Vec<FnVariableData>>,
	fn_stack_frame: Option<Vec<FnStackFrame>>,
	fn_inlined_functions: Option<Vec<FnInlinedFunctions>>,
}

pub fn parse_fn_data(buffer: &str) -> FnData {
	let line_buffer = buffer.lines();
	let mut function = FnData::default();

	for line in line_buffer {
		let clean_line = line.replace("\t", "");

		if let Some((k, v)) = clean_line.split_once(" ") {
			match k {
				"fn" => {
					function.name = v.to_string();
				}
				_ => {}
			}
		}

		if let Some((key, value)) = clean_line.split_once(":") {
			match key {
				"linkage name" => {
					function.linkage_name = Some(value.trim().to_string());
				}
				"source" => {
					let function_file_path = PathBuf::from(value);

					if let Some(the_file_name) = function_file_path.file_name() {
						let (line, column) = get_line_number_and_column(the_file_name);
						function.line_num = line;
						function.line_col = column;
						function.full_file_path =
							Some(function_file_path.to_string_lossy().trim().to_string());
					}
				}
				"address" => {
					let (addr_start, addr_end) = parse_address_range(&value.to_string());
					let calculated_size =
						value_to_u32((addr_end.as_str())) - value_to_u32((addr_start.as_str()));

					function.address = AddressData {
						start_address: addr_start,
						end_address: addr_end,
						total_size: calculated_size,
					};
				}
				"return type" => {
					let return_type_buffer = if buffer.contains("\treturn type:") {
						let rt_buf = if let Some(rt_buf) = buffer.find("\treturn type:") {
							let mut retval_type_buffr: String =
								buffer.to_string().drain(rt_buf..).collect();
							if retval_type_buffr.contains("parameters:") {
								if let Some(indx) = retval_type_buffr.find("parameters:") {
									retval_type_buffr = retval_type_buffr.drain(..indx).collect();
								};
							}

							Some(retval_type_buffr)
						} else {
							None
						};
						rt_buf
					} else {
						None
					};

					if let Some(ref return_data) = return_type_buffer {
						function.return_data = handle_fn_return_type(return_data);
					}
				}
				"parameters" => {
					let parameters_buffer = if buffer.contains("\tparameters:") {
						let param_buffer = if let Some(param_buffer) = buffer.find("\tparameters:")
						{
							let mut params: String =
								buffer.to_string().drain(param_buffer..).collect();
							if params.contains("variables:") {
								if let Some(indx) = params.find("variables:") {
									params = params.drain(..indx).collect();
									//params = params.replace("\t", "");
								};
							}
							if params.contains("stack frame:\n") {
								if let Some(indx) = params.find("stack frame:\n") {
									params = params.drain(..indx).collect();
								};
							}
							Some(params)
						} else {
							None
						};
						param_buffer
					} else {
						None
					};

					if let Some(ref params) = parameters_buffer {
						function.fn_params = handle_fn_params(params);
					}
				}
				"variables" => {
					let variables_buffer = if buffer.contains("\tvariables:\n") {
						let var_buffer = if let Some(variable_buf) = buffer.find("\tvariables:") {
							let mut variables: String =
								buffer.to_string().drain(variable_buf..).collect();
							if variables.contains("stack frame:\n") {
								if let Some(indx) = variables.find("stack frame:\n") {
									variables = variables.drain(..indx).collect();
								};
							}
							Some(variables)
						} else {
							None
						};
						(var_buffer)
					} else {
						None
					};
					if let Some(ref params) = variables_buffer {
						function.fn_vars = handle_fn_vars(params);
					}
				}
				"stack frame" => {
					let stackframe_buffer = if buffer.contains("\tstack frame:\n") {
						let stack_buffer = if let Some(stack_buf) = buffer.find("\tstack frame:\n")
						{
							let mut stackf: String =
								buffer.to_string().drain(stack_buf..).collect();
							if stackf.contains("\tinlined functions:\n") {
								if let Some(indx) = stackf.find("\tinlined functions:\n") {
									stackf = stackf.drain(..indx).collect();
								};
							}
							Some(stackf)
						} else {
							None
						};
						(stack_buffer)
					} else {
						None
					};
					if let Some(ref params) = stackframe_buffer {
						function.fn_stack_frame = handle_fn_stack_frame(params);
					}
				}
				"inlined functions" => {
					let inlined_fn_buffer = if buffer.contains("\tinlined functions:\n") {
						let inline_functions_buf =
							if let Some(indx) = buffer.find("\tinlined functions:\n") {
								let retval: String = buffer.to_string().drain(indx..).collect();

								Some(retval)
							} else {
								None
							};
						(inline_functions_buf)
					} else {
						None
					};
					if let Some(ref inline_fn_buffr) = inlined_fn_buffer {
						function.fn_inlined_functions =
							handle_fn_inlined_functions(inline_fn_buffr);
					}
				}
				"size" => {
					let the_size = value_to_u32((value));
					function.address.total_size = the_size;
					function.total_size = the_size;
				}
				_ => {}
			}
		}
	}
	function
}

fn handle_fn_return_type(input: &str) -> Option<ReturnType> {
	let mut return_data = ReturnType::default();

	for current_line in input.lines().skip(1) {
		if let Some((rt_size, rt_type)) = current_line.split_once("]") {
			let cleaned = rt_size.replace(&['[', ']', '\t'], "");
			return_data.return_size = value_to_u32(cleaned.trim());
			return_data.return_type = rt_type.trim().to_string();
		} else {
			"ERROR 322: Failed to parse function return value".to_string();
		}
	}
	Some(return_data)
}

fn handle_fn_params(input: &String) -> Option<Vec<FnParamData>> {
	let mut vector: Vec<FnParamData> = Vec::new();
	let mut param_data = FnParamData::default();

	let formatted = input.replace("\t", "");

	for current_line in formatted.lines().skip(1) {
		if current_line.starts_with("[") {
			if let Some((k, v)) = current_line.split_once(":") {
				param_data.fn_param_type = v.trim().to_string();
				if let Some((p_size, p_name)) = k.split_once("]") {
					param_data.fn_param_name = p_name.trim().to_string();
					let cleaned_size = p_size.replace(&['[', ']'], "");
					param_data.fn_param_size = value_to_u32(cleaned_size.as_str());
				} else {
					"ERROR 234: Failed to split size and name".to_string();
				}
			} else {
				"ERROR 233: Failed to parse param members".to_string();
			}
		} else {
			if let Some((_k, stack_frame_size)) = current_line.split_once(":") {
				param_data.fn_param_stack_frame = value_to_i32(Some(stack_frame_size));
			} else {
				"ERROR 345: Failed to parse stack frame size".to_string();
			}
			vector.push(param_data);
			param_data = FnParamData::default();
		}
	}
	if vector.len() > 0 {
		Some(vector)
	} else {
		None
	}
}

fn handle_fn_vars(input: &String) -> Option<Vec<FnVariableData>> {
	let mut vector: Vec<FnVariableData> = Vec::new();
	let mut var_data = FnVariableData::default();

	let formatted = input.replace("\t", "");

	for current_line in formatted.lines().skip(1) {
		if current_line.starts_with("[") {
			if let Some((k, v)) = current_line.split_once(":") {
				var_data.fn_var_type = Some(v.trim().to_string());
				if let Some((p_size, p_name)) = k.split_once("]") {
					var_data.fn_var_name = p_name.trim().to_string();
					let cleaned_size = p_size.replace(&['[', ']'], "");
					var_data.fn_var_size = value_to_u32(cleaned_size.as_str());
				} else {
					"ERROR 237: Failed to split size and name".to_string();
				}
			} else {
				"ERROR 231: Failed to parse vars members".to_string();
			}
		} else {
			if let Some((_k, stack_frame_size)) = current_line.split_once(":") {
				var_data.fn_var_stack_frame = value_to_i32(Some(stack_frame_size));
			} else {
				"ERROR 347: Failed to parse vars size".to_string();
			}
			vector.push(var_data);
			var_data = FnVariableData::default();
		}
	}

	if vector.len() > 0 {
		Some(vector)
	} else {
		None
	}
}

fn handle_fn_stack_frame(input: &String) -> Option<Vec<FnStackFrame>> {
	let mut vector: Vec<FnStackFrame> = Vec::new();
	let mut stack_frame_data = FnStackFrame::default();

	let formatted = input.replace("\t", "");

	for current_line in formatted.lines().skip(1) {
		if let Some((k, v)) = current_line.split_once(":") {
			stack_frame_data.fn_stack_frame_type = Some(v.trim().to_string());
			if let Some((p_size, p_name)) = k.split_once("]") {
				stack_frame_data.fn_stack_frame_name = Some(p_name.trim().to_string());

				if let Some((frame_offset, stack_frame_size)) = p_size.split_once("[") {
					stack_frame_data.fn_stack_frame_stack_offset = value_to_i32(Some(frame_offset));
					stack_frame_data.fn_stack_frame_size = value_to_u32(stack_frame_size);
				} else {
					"ERROR 348: Failed to parse stack frame size".to_string();
				}
			} else {
				//stack_frame_data.fn_stack_frame_name = None;
				"ERROR 238: Failed to split size and name".to_string();
			}
		} else {
			// Handle situation where there is frame gaps
			if current_line.contains("<unknown>") {
				if let Some((k, v)) = current_line.split_once("]") {
					if let Some((frame_offset, frame_size)) = k.split_once("[") {
						stack_frame_data.fn_stack_frame_stack_offset =
							value_to_i32(Some(frame_offset));
						stack_frame_data.fn_stack_frame_size = value_to_u32(frame_size);
						stack_frame_data.fn_stack_frame_name = Some(v.to_string());
						stack_frame_data.fn_stack_frame_type = None;
					}
				}
			} else {
				//stack_frame_data.fn_stack_frame_type = None;
				"ERROR 238: Failed to parse tack frame type".to_string();
			}
		}

		vector.push(stack_frame_data);
		stack_frame_data = FnStackFrame::default();
	}

	if vector.len() > 0 {
		Some(vector)
	} else {
		None
	}
}
fn handle_fn_inlined_functions(input: &String) -> Option<Vec<FnInlinedFunctions>> {
	let mut vector: Vec<FnInlinedFunctions> = Vec::new();
	let mut inline_fn_data = FnInlinedFunctions::default();

	let formatted = input.replace("\t", "");

	for current_line in formatted.lines().skip(1) {
		if current_line.contains(":") {
			if let Some((k, v)) = current_line.split_once(":") {
				if k.contains("call source") {
					let file_path = PathBuf::from(v.trim());
					inline_fn_data.inlined_fn_call_source_path = Some(v.trim().to_string());
					if let Some(the_file_name) = file_path.file_name() {
						match get_line_number_and_column(the_file_name) {
							(Some(line), Some(column)) => {
								inline_fn_data.line_num = Some(line);
								inline_fn_data.line_col = Some(column)
							}
							_ => {}
						}
					}
				}
			} else {
				//stack_frame_data.fn_stack_frame_type = None;
				"ERROR 238: Failed to parse tack frame type".to_string();
			}
			vector.push(inline_fn_data);
			inline_fn_data = FnInlinedFunctions::default();
		} else {
			if let Some((k, v)) = current_line.split_once("]") {
				inline_fn_data.inlined_fn_name = Some(v.trim().to_string());
				inline_fn_data.inlined_fn_size = value_to_u32(k.replace("[", "").as_str());
			} else {
				//stack_frame_data.fn_stack_frame_type = None;
				"ERROR 238: Failed to parse tack frame type".to_string();
			}
		}
	}

	if vector.len() > 0 {
		Some(vector)
	} else {
		None
	}
}
