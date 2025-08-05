
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

pub trait DrawCommand: Default + Clone + Copy + Sized + Debug {}

impl DrawCommand for DrawArrayCommand {}
impl DrawCommand for DrawElementsCommand {}
