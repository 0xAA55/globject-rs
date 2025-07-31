#![allow(dead_code)]

use glcore::*;
use crate::glbuffer::*;
use crate::arraybuffer::*;
use std::{
	fmt::{self, Debug, Formatter}
};

#[derive(Clone)]
pub struct Mesh<'a> {
	glcore: &'a GLCore,
	vertex_buffer: Buffer<'a>,
	instance_buffer: Option<Buffer<'a>>,
	command_buffer: Option<Buffer<'a>>
}

#[derive(Clone)]
pub struct EditableMesh<'a> {
	glcore: &'a GLCore,
	vertex_buffer: ArrayBuffer<'a>,
	instance_buffer: Option<ArrayBuffer<'a>>,
	command_buffer: Option<ArrayBuffer<'a>>
}

#[derive(Clone)]
pub struct DynamicMesh<'a, T: ArrayBufferItem> {
	glcore: &'a GLCore,
	vertex_buffer: ArrayBufferDynamic<'a, T>,
	instance_buffer: Option<ArrayBufferDynamic<'a, T>>,
	command_buffer: Option<ArrayBufferDynamic<'a, T>>
}

impl<'a> Debug for Mesh<'a> {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		f.debug_struct("Mesh")
		.field("vertex_buffer", &self.vertex_buffer)
		.field("instance_buffer", &self.instance_buffer)
		.field("command_buffer", &self.command_buffer)
		.finish()
	}
}

impl<'a> Debug for EditableMesh<'a> {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		f.debug_struct("Mesh")
		.field("vertex_buffer", &self.vertex_buffer)
		.field("instance_buffer", &self.instance_buffer)
		.field("command_buffer", &self.command_buffer)
		.finish()
	}
}

impl<'a, T: ArrayBufferItem> Debug for DynamicMesh<'a, T> {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		f.debug_struct("Mesh")
		.field("vertex_buffer", &self.vertex_buffer)
		.field("instance_buffer", &self.instance_buffer)
		.field("command_buffer", &self.command_buffer)
		.finish()
	}
}
