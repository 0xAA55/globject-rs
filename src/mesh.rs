#![allow(dead_code)]

use glcore::*;
use crate::glbuffer::*;
use crate::arraybuffer::*;
use std::{
	fmt::{self, Debug, Formatter}
};

#[derive(Clone)]
pub struct Mesh<'a> {
	pub glcore: &'a GLCore,
	pub vertex_buffer: Buffer<'a>,
	pub instance_buffer: Option<Buffer<'a>>,
	pub command_buffer: Option<Buffer<'a>>,
}

#[derive(Clone)]
pub struct EditableMesh<'a> {
	pub glcore: &'a GLCore,
	pub vertex_buffer: ArrayBuffer<'a>,
	pub instance_buffer: Option<ArrayBuffer<'a>>,
	pub command_buffer: Option<ArrayBuffer<'a>>,
}

#[derive(Clone)]
pub struct DynamicMesh<'a, T: ArrayBufferItem> {
	pub glcore: &'a GLCore,
	pub vertex_buffer: ArrayBufferDynamic<'a, T>,
	pub instance_buffer: Option<ArrayBufferDynamic<'a, T>>,
	pub command_buffer: Option<ArrayBufferDynamic<'a, T>>,
}

impl<'a> Mesh<'a> {
	pub fn new(glcore: &'a GLCore, vertex_buffer: Buffer<'a>, instance_buffer: Option<Buffer<'a>>, command_buffer: Option<Buffer<'a>>) -> Self {
		Self {
			glcore,
			vertex_buffer,
			instance_buffer,
			command_buffer,
		}
	}
}

impl<'a> EditableMesh<'a> {
	pub fn new(glcore: &'a GLCore, vertex_buffer: ArrayBuffer<'a>, instance_buffer: Option<ArrayBuffer<'a>>, command_buffer: Option<ArrayBuffer<'a>>) -> Self {
		Self {
			glcore,
			vertex_buffer,
			instance_buffer,
			command_buffer,
		}
	}
}

impl<'a, T: ArrayBufferItem> DynamicMesh<'a, T> {
	pub fn new(glcore: &'a GLCore, vertex_buffer: ArrayBufferDynamic<'a, T>, instance_buffer: Option<ArrayBufferDynamic<'a, T>>, command_buffer: Option<ArrayBufferDynamic<'a, T>>) -> Self {
		Self {
			glcore,
			vertex_buffer,
			instance_buffer,
			command_buffer,
		}
	}
}

impl<'a> Into<EditableMesh<'a>> for Mesh<'a> {
	fn into(self) -> EditableMesh<'a> {
		EditableMesh {
			glcore: self.glcore,
			vertex_buffer: self.vertex_buffer.into(),
			instance_buffer: self.instance_buffer.map(|b|b.into()),
			command_buffer: self.command_buffer.map(|b|b.into()),
		}
	}
}

impl<'a> Into<Mesh<'a>> for EditableMesh<'a> {
	fn into(self) -> Mesh<'a> {
		Mesh {
			glcore: self.glcore,
			vertex_buffer: self.vertex_buffer.into(),
			instance_buffer: self.instance_buffer.map(|b|b.into()),
			command_buffer: self.command_buffer.map(|b|b.into()),
		}
	}
}

impl<'a, T: ArrayBufferItem> Into<DynamicMesh<'a, T>> for Mesh<'a> {
	fn into(self) -> DynamicMesh<'a, T> {
		DynamicMesh {
			glcore: self.glcore,
			vertex_buffer: self.vertex_buffer.into(),
			instance_buffer: self.instance_buffer.map(|b|b.into()),
			command_buffer: self.command_buffer.map(|b|b.into()),
		}
	}
}

impl<'a, T: ArrayBufferItem> Into<Mesh<'a>> for DynamicMesh<'a, T> {
	fn into(self) -> Mesh<'a> {
		Mesh {
			glcore: self.glcore,
			vertex_buffer: self.vertex_buffer.into(),
			instance_buffer: self.instance_buffer.map(|b|b.into()),
			command_buffer: self.command_buffer.map(|b|b.into()),
		}
	}
}

impl<'a, T: ArrayBufferItem> Into<EditableMesh<'a>> for DynamicMesh<'a, T> {
	fn into(self) -> EditableMesh<'a> {
		EditableMesh {
			glcore: self.glcore,
			vertex_buffer: self.vertex_buffer.into(),
			instance_buffer: self.instance_buffer.map(|b|b.into()),
			command_buffer: self.command_buffer.map(|b|b.into()),
		}
	}
}

impl<'a, T: ArrayBufferItem> Into<DynamicMesh<'a, T>> for EditableMesh<'a> {
	fn into(self) -> DynamicMesh<'a, T> {
		DynamicMesh {
			glcore: self.glcore,
			vertex_buffer: self.vertex_buffer.into(),
			instance_buffer: self.instance_buffer.map(|b|b.into()),
			command_buffer: self.command_buffer.map(|b|b.into()),
		}
	}
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
