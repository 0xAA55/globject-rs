
use std::{
	fmt::Debug,
};

/// The data for `glMultiDrawArraysIndirect` to submit multiple draw array commands at once with instancing
/// Must be binded to the `BufferTarget::DrawIndirectBuffer`
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct DrawArrayCommand {
	vertex_count: u32,
	instance_count: u32,
	first_index: u32,
	base_instance: u32,
}

/// The data for `glMultiDrawElementsIndirect` to submit multiple draw element commands at once with instancing
/// Must be binded to the `BufferTarget::DrawIndirectBuffer`
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct DrawElementsCommand {
	element_count: u32,
	instance_count: u32,
	first_index: u32,
	base_vertex: i32,
	base_instance: u32,
}

/// The data for `glDispatchComputeIndirect` to submit multiple compute commands at once
/// Must be binded to the `BufferTarget::DispatchIndirectBuffer`
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct DispatchIndirectCommand {
	num_groups_x: u32,
	num_groups_y: u32,
	num_groups_z: u32,
}

/// The trait for all of the commands
pub trait DrawCommand: Default + Clone + Copy + Sized + Debug {}

impl DrawCommand for DrawArrayCommand {}
impl DrawCommand for DrawElementsCommand {}
impl DrawCommand for DispatchIndirectCommand {}
