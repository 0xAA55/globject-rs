#![allow(dead_code)]

use std::default::Default;

#[repr(packed)]
#[derive(Debug, Clone, Copy)]
pub struct DrawCommand {
	element_count: u32,
	instance_count: u32,
	first_index: u32,
	base_vertex: i32,
	base_instance: u32,
}

impl Default for DrawCommand {
	fn default() -> Self {
		Self {
			element_count: 0,
			instance_count: 0,
			first_index: 0,
			base_vertex: 0,
			base_instance: 0,
		}
	}
}
