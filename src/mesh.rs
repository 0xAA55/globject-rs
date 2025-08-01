
#![allow(dead_code)]

use glcore::*;
use crate::glbuffer::*;
use crate::glcmdbuf::*;
use crate::buffervec::*;
use std::{
	fmt::{self, Debug, Formatter}
};

#[derive(Clone)]
pub struct StaticMesh<'a> {
	pub glcore: &'a GLCore,
	pub vertex_buffer: Buffer<'a>,
	pub instance_buffer: Option<Buffer<'a>>,
	pub command_buffer: Option<Buffer<'a>>,
}

#[derive(Clone)]
pub struct EditableMesh<'a> {
	pub glcore: &'a GLCore,
	pub vertex_buffer: BufferVec<'a>,
	pub instance_buffer: Option<BufferVec<'a>>,
	pub command_buffer: Option<BufferVec<'a>>,
}

#[derive(Clone)]
pub struct DynamicMesh<'a, T: BufferVecItem> {
	pub glcore: &'a GLCore,
	pub vertex_buffer: BufferVecDynamic<'a, T>,
	pub instance_buffer: Option<BufferVecDynamic<'a, T>>,
	pub command_buffer: Option<BufferVecDynamic<'a, DrawCommand>>,
}

impl<'a> StaticMesh<'a> {
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
	pub fn new(glcore: &'a GLCore, vertex_buffer: BufferVec<'a>, instance_buffer: Option<BufferVec<'a>>, command_buffer: Option<BufferVec<'a>>) -> Self {
		Self {
			glcore,
			vertex_buffer,
			instance_buffer,
			command_buffer,
		}
	}
}

impl<'a, T: BufferVecItem> DynamicMesh<'a, T> {
	pub fn new(glcore: &'a GLCore, vertex_buffer: BufferVecDynamic<'a, T>, instance_buffer: Option<BufferVecDynamic<'a, T>>, command_buffer: Option<BufferVecDynamic<'a, DrawCommand>>) -> Self {
		Self {
			glcore,
			vertex_buffer,
			instance_buffer,
			command_buffer,
		}
	}
}

impl<'a> From<StaticMesh<'a>> for EditableMesh<'a> {
	fn from(val: StaticMesh<'a>) -> Self {
		EditableMesh {
			glcore: val.glcore,
			vertex_buffer: val.vertex_buffer.into(),
			instance_buffer: val.instance_buffer.map(|b|b.into()),
			command_buffer: val.command_buffer.map(|b|b.into()),
		}
	}
}

impl<'a> From<EditableMesh<'a>> for StaticMesh<'a> {
	fn from(val: EditableMesh<'a>) -> Self {
		StaticMesh {
			glcore: val.glcore,
			vertex_buffer: val.vertex_buffer.into(),
			instance_buffer: val.instance_buffer.map(|b|b.into()),
			command_buffer: val.command_buffer.map(|b|b.into()),
		}
	}
}

impl<'a, T: BufferVecItem> From<StaticMesh<'a>> for DynamicMesh<'a, T> {
	fn from(val: StaticMesh<'a>) -> Self {
		DynamicMesh {
			glcore: val.glcore,
			vertex_buffer: val.vertex_buffer.into(),
			instance_buffer: val.instance_buffer.map(|b|b.into()),
			command_buffer: val.command_buffer.map(|b|b.into()),
		}
	}
}

impl<'a, T: BufferVecItem> From<DynamicMesh<'a, T>> for StaticMesh<'a> {
	fn from(val: DynamicMesh<'a, T>) -> Self {
		StaticMesh {
			glcore: val.glcore,
			vertex_buffer: val.vertex_buffer.into(),
			instance_buffer: val.instance_buffer.map(|b|b.into()),
			command_buffer: val.command_buffer.map(|b|b.into()),
		}
	}
}

impl<'a, T: BufferVecItem> From<DynamicMesh<'a, T>> for EditableMesh<'a> {
	fn from(val: DynamicMesh<'a, T>) -> Self {
		EditableMesh {
			glcore: val.glcore,
			vertex_buffer: val.vertex_buffer.into(),
			instance_buffer: val.instance_buffer.map(|b|b.into()),
			command_buffer: val.command_buffer.map(|b|b.into()),
		}
	}
}

impl<'a, T: BufferVecItem> From<EditableMesh<'a>> for DynamicMesh<'a, T> {
	fn from(val: EditableMesh<'a>) -> Self {
		DynamicMesh {
			glcore: val.glcore,
			vertex_buffer: val.vertex_buffer.into(),
			instance_buffer: val.instance_buffer.map(|b|b.into()),
			command_buffer: val.command_buffer.map(|b|b.into()),
		}
	}
}

impl<'a> Debug for StaticMesh<'a> {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		f.debug_struct("StaticMesh")
		.field("vertex_buffer", &self.vertex_buffer)
		.field("instance_buffer", &self.instance_buffer)
		.field("command_buffer", &self.command_buffer)
		.finish()
	}
}

impl<'a> Debug for EditableMesh<'a> {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		f.debug_struct("EditableMesh")
		.field("vertex_buffer", &self.vertex_buffer)
		.field("instance_buffer", &self.instance_buffer)
		.field("command_buffer", &self.command_buffer)
		.finish()
	}
}

impl<'a, T: BufferVecItem> Debug for DynamicMesh<'a, T> {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		f.debug_struct("DynamicMesh")
		.field("vertex_buffer", &self.vertex_buffer)
		.field("instance_buffer", &self.instance_buffer)
		.field("command_buffer", &self.command_buffer)
		.finish()
	}
}

pub trait Mesh: Debug {
	fn get_glcore(&self) -> &GLCore;
	fn get_vertex_buffer(&self) -> &Buffer;
	fn get_instance_buffer(&self) -> Option<&Buffer>;
	fn get_command_buffer(&self) -> Option<&Buffer>;
}

impl Mesh for StaticMesh<'_> {
	fn get_glcore(&self) -> &GLCore {
		self.glcore
	}

	fn get_vertex_buffer(&self) -> &Buffer {
		&self.vertex_buffer
	}

	fn get_instance_buffer(&self) -> Option<&Buffer> {
		if let Some(buffer) = &self.instance_buffer {
			Some(buffer)
		} else {
			None
		}
	}

	fn get_command_buffer(&self) -> Option<&Buffer> {
		if let Some(buffer) = &self.command_buffer {
			Some(buffer)
		} else {
			None
		}
	}
}

impl Mesh for EditableMesh<'_> {
	fn get_glcore(&self) -> &GLCore {
		self.glcore
	}

	fn get_vertex_buffer(&self) -> &Buffer {
		self.vertex_buffer.get_buffer()
	}

	fn get_instance_buffer(&self) -> Option<&Buffer> {
		if let Some(buffer) = &self.instance_buffer {
			Some(buffer.get_buffer())
		} else {
			None
		}
	}

	fn get_command_buffer(&self) -> Option<&Buffer> {
		if let Some(buffer) = &self.command_buffer {
			Some(buffer.get_buffer())
		} else {
			None
		}
	}
}

impl<T: BufferVecItem> Mesh for DynamicMesh<'_, T> {
	fn get_glcore(&self) -> &GLCore {
		self.glcore
	}

	fn get_vertex_buffer(&self) -> &Buffer {
		self.vertex_buffer.get_buffer()
	}

	fn get_instance_buffer(&self) -> Option<&Buffer> {
		if let Some(buffer) = &self.instance_buffer {
			Some(buffer.get_buffer())
		} else {
			None
		}
	}

	fn get_command_buffer(&self) -> Option<&Buffer> {
		if let Some(buffer) = &self.command_buffer {
			Some(buffer.get_buffer())
		} else {
			None
		}
	}
}

