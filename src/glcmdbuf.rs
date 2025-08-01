
#![allow(dead_code)]

#[repr(C, packed)]
#[derive(Default, Debug, Clone, Copy)]
pub struct DrawCommand {
	element_count: u32,
	instance_count: u32,
	first_index: u32,
	base_vertex: i32,
	base_instance: u32,
}
