
use std::{
	fmt::Debug,
};

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct DrawArrayCommand {
	vertex_count: u32,
	instance_count: u32,
	first_index: u32,
	base_instance: u32,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct DrawElementsCommand {
	element_count: u32,
	instance_count: u32,
	first_index: u32,
	base_vertex: i32,
	base_instance: u32,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct DispatchIndirectCommand {
	num_groups_x: u32,
	num_groups_y: u32,
	num_groups_z: u32,
}

pub trait DrawCommand: Default + Clone + Copy + Sized + Debug {}

impl DrawCommand for DrawArrayCommand {}
impl DrawCommand for DrawElementsCommand {}
impl DrawCommand for DispatchIndirectCommand {}
