
#![allow(dead_code)]

use glcore::*;
use crate::glbuffer::*;
use crate::glcmdbuf::*;
use crate::buffervec::*;
use std::{
	fmt::{self, Debug, Formatter},
	rc::Rc,
};

#[derive(Clone)]
pub struct StaticMesh {
	pub glcore: Rc<GLCore>,
	pub vertex_buffer: Buffer,
	pub element_buffer: Option<Buffer>,
	pub instance_buffer: Option<Buffer>,
	pub command_buffer: Option<Buffer>,
}

#[derive(Clone)]
pub struct EditableMesh {
	pub glcore: Rc<GLCore>,
	pub vertex_buffer: BufferVec,
	pub element_buffer: Option<BufferVec>,
	pub instance_buffer: Option<BufferVec>,
	pub command_buffer: Option<BufferVec>,
}

#[derive(Clone)]
pub struct DynamicMesh<T: BufferVecItem, E: BufferVecItem, I: BufferVecItem> {
	pub glcore: Rc<GLCore>,
	pub vertex_buffer: BufferVecDynamic<T>,
	pub element_buffer: Option<BufferVecDynamic<E>>,
	pub instance_buffer: Option<BufferVecDynamic<I>>,
	pub command_buffer: Option<BufferVecDynamic<DrawCommand>>,
}

impl StaticMesh {
	pub fn new(glcore: Rc<GLCore>, vertex_buffer: Buffer, element_buffer: Option<Buffer>, instance_buffer: Option<Buffer>, command_buffer: Option<Buffer>) -> Self {
		Self {
			glcore,
			element_buffer,
			vertex_buffer,
			instance_buffer,
			command_buffer,
		}
	}
}

impl EditableMesh {
	pub fn new(glcore: Rc<GLCore>, vertex_buffer: BufferVec, element_buffer: Option<BufferVec>, instance_buffer: Option<BufferVec>, command_buffer: Option<BufferVec>) -> Self {
		Self {
			glcore,
			vertex_buffer,
			element_buffer,
			instance_buffer,
			command_buffer,
		}
	}
}

impl<T: BufferVecItem, E: BufferVecItem, I: BufferVecItem> DynamicMesh<T, E, I> {
	pub fn new(glcore: Rc<GLCore>, vertex_buffer: BufferVecDynamic<T>, element_buffer: Option<BufferVecDynamic<E>>, instance_buffer: Option<BufferVecDynamic<I>>, command_buffer: Option<BufferVecDynamic<DrawCommand>>) -> Self {
		Self {
			glcore,
			vertex_buffer,
			element_buffer,
			instance_buffer,
			command_buffer,
		}
	}
}

impl From<StaticMesh> for EditableMesh {
	fn from(val: StaticMesh) -> Self {
		EditableMesh {
			glcore: val.glcore,
			vertex_buffer: val.vertex_buffer.into(),
			element_buffer: val.element_buffer.map(|b|b.into()),
			instance_buffer: val.instance_buffer.map(|b|b.into()),
			command_buffer: val.command_buffer.map(|b|b.into()),
		}
	}
}

impl From<EditableMesh> for StaticMesh {
	fn from(val: EditableMesh) -> Self {
		StaticMesh {
			glcore: val.glcore,
			vertex_buffer: val.vertex_buffer.into(),
			element_buffer: val.element_buffer.map(|b|b.into()),
			instance_buffer: val.instance_buffer.map(|b|b.into()),
			command_buffer: val.command_buffer.map(|b|b.into()),
		}
	}
}

impl<T: BufferVecItem, E: BufferVecItem, I: BufferVecItem> From<StaticMesh> for DynamicMesh<T, E, I> {
	fn from(val: StaticMesh) -> Self {
		DynamicMesh {
			glcore: val.glcore,
			vertex_buffer: val.vertex_buffer.into(),
			element_buffer: val.element_buffer.map(|b|b.into()),
			instance_buffer: val.instance_buffer.map(|b|b.into()),
			command_buffer: val.command_buffer.map(|b|b.into()),
		}
	}
}

impl<T: BufferVecItem, E: BufferVecItem, I: BufferVecItem> From<DynamicMesh<T, E, I>> for StaticMesh {
	fn from(val: DynamicMesh<T, E, I>) -> Self {
		StaticMesh {
			glcore: val.glcore,
			vertex_buffer: val.vertex_buffer.into(),
			element_buffer: val.element_buffer.map(|b|b.into()),
			instance_buffer: val.instance_buffer.map(|b|b.into()),
			command_buffer: val.command_buffer.map(|b|b.into()),
		}
	}
}

impl<T: BufferVecItem, E: BufferVecItem, I: BufferVecItem> From<DynamicMesh<T, E, I>> for EditableMesh {
	fn from(val: DynamicMesh<T, E, I>) -> Self {
		EditableMesh {
			glcore: val.glcore,
			vertex_buffer: val.vertex_buffer.into(),
			element_buffer: val.element_buffer.map(|b|b.into()),
			instance_buffer: val.instance_buffer.map(|b|b.into()),
			command_buffer: val.command_buffer.map(|b|b.into()),
		}
	}
}

impl<T: BufferVecItem, E: BufferVecItem, I: BufferVecItem> From<EditableMesh> for DynamicMesh<T, E, I> {
	fn from(val: EditableMesh) -> Self {
		DynamicMesh {
			glcore: val.glcore,
			vertex_buffer: val.vertex_buffer.into(),
			element_buffer: val.element_buffer.map(|b|b.into()),
			instance_buffer: val.instance_buffer.map(|b|b.into()),
			command_buffer: val.command_buffer.map(|b|b.into()),
		}
	}
}

impl Debug for StaticMesh {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		f.debug_struct("StaticMesh")
		.field("vertex_buffer", &self.vertex_buffer)
		.field("element_buffer", &self.element_buffer)
		.field("instance_buffer", &self.instance_buffer)
		.field("command_buffer", &self.command_buffer)
		.finish()
	}
}

impl Debug for EditableMesh {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		f.debug_struct("EditableMesh")
		.field("vertex_buffer", &self.vertex_buffer)
		.field("element_buffer", &self.element_buffer)
		.field("instance_buffer", &self.instance_buffer)
		.field("command_buffer", &self.command_buffer)
		.finish()
	}
}

impl<T: BufferVecItem, E: BufferVecItem, I: BufferVecItem> Debug for DynamicMesh<T, E, I> {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		f.debug_struct("DynamicMesh")
		.field("vertex_buffer", &self.vertex_buffer)
		.field("element_buffer", &self.element_buffer)
		.field("instance_buffer", &self.instance_buffer)
		.field("command_buffer", &self.command_buffer)
		.finish()
	}
}

pub trait Mesh: Debug {
	fn get_glcore(&self) -> &GLCore;
	fn get_vertex_buffer(&self) -> &Buffer;
	fn get_element_buffer(&self) -> Option<&Buffer>;
	fn get_instance_buffer(&self) -> Option<&Buffer>;
	fn get_command_buffer(&self) -> Option<&Buffer>;
}

impl Mesh for StaticMesh {
	fn get_glcore(&self) -> &GLCore {
		self.glcore.as_ref()
	}

	fn get_vertex_buffer(&self) -> &Buffer {
		&self.vertex_buffer
	}

	fn get_element_buffer(&self) -> Option<&Buffer> {
		if let Some(buffer) = &self.element_buffer {
			Some(buffer)
		} else {
			None
		}
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

impl Mesh for EditableMesh {
	fn get_glcore(&self) -> &GLCore {
		self.glcore.as_ref()
	}

	fn get_vertex_buffer(&self) -> &Buffer {
		self.vertex_buffer.get_buffer()
	}
	
	fn get_element_buffer(&self) -> Option<&Buffer> {
		if let Some(buffer) = &self.element_buffer {
			Some(buffer.get_buffer())
		} else {
			None
		}
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

impl<T: BufferVecItem, E: BufferVecItem, I: BufferVecItem> Mesh for DynamicMesh<T, E, I> {
	fn get_glcore(&self) -> &GLCore {
		self.glcore.as_ref()
	}

	fn get_vertex_buffer(&self) -> &Buffer {
		self.vertex_buffer.get_buffer()
	}
	
	fn get_element_buffer(&self) -> Option<&Buffer> {
		if let Some(buffer) = &self.element_buffer {
			Some(buffer.get_buffer())
		} else {
			None
		}
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

