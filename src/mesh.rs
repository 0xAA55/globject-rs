
#![allow(dead_code)]

use glcore::*;
use crate::glbuffer::*;
use crate::glcmdbuf::*;
use crate::buffervec::*;
use std::{
	fmt::{self, Debug, Formatter},
	rc::Rc,
};

#[derive(Clone, Copy, PartialEq)]
pub enum PrimitiveMode {
	Points = GL_POINTS as isize,
	LineStrip = GL_LINE_STRIP as isize,
	LineLoop = GL_LINE_LOOP as isize,
	Lines = GL_LINES as isize,
	LineStripAdjacency = GL_LINE_STRIP_ADJACENCY as isize,
	LinesAdjacency = GL_LINES_ADJACENCY as isize,
	TriangleStrip = GL_TRIANGLE_STRIP as isize,
	TriangleFan = GL_TRIANGLE_FAN as isize,
	Triangles = GL_TRIANGLES as isize,
	TriangleStripAdjacency = GL_TRIANGLE_STRIP_ADJACENCY as isize,
	TrianglesAdjacency = GL_TRIANGLES_ADJACENCY as isize,
	Patches = GL_PATCHES as isize,
}

#[derive(Clone, Copy, PartialEq)]
pub enum ElementType {
	U8 = GL_UNSIGNED_BYTE as isize,
	U16 = GL_UNSIGNED_SHORT as isize,
	U32 = GL_UNSIGNED_INT as isize,
}

#[derive(Debug, Clone)]
pub struct ElementBuffer {
	pub buffer: Buffer,
	pub element_type: ElementType,
}

#[derive(Debug, Clone)]
pub struct ElementBufferVec {
	pub buffer: BufferVec,
	pub element_type: ElementType,
}

#[derive(Debug, Clone)]
pub struct ElementBufferVecDynamic<T: BufferVecItem> {
	pub buffer: BufferVecDynamic<T>,
	pub element_type: ElementType,
}

#[derive(Debug, Clone)]
pub struct ElementBufferRef<'a> {
	pub buffer: &'a Buffer,
	pub element_type: ElementType,
}

#[derive(Clone)]
pub struct StaticMesh {
	pub glcore: Rc<GLCore>,
	pub primitive: PrimitiveMode,
	pub vertex_buffer: Buffer,
	pub element_buffer: Option<ElementBuffer>,
	pub instance_buffer: Option<Buffer>,
	pub command_buffer: Option<Buffer>,
}

#[derive(Clone)]
pub struct EditableMesh {
	pub glcore: Rc<GLCore>,
	pub primitive: PrimitiveMode,
	pub vertex_buffer: BufferVec,
	pub element_buffer: Option<ElementBufferVec>,
	pub instance_buffer: Option<BufferVec>,
	pub command_buffer: Option<BufferVec>,
}

#[derive(Clone)]
pub struct DynamicMesh<T: BufferVecItem, E: BufferVecItem, I: BufferVecItem, C: DrawCommand> {
	pub glcore: Rc<GLCore>,
	pub primitive: PrimitiveMode,
	pub vertex_buffer: BufferVecDynamic<T>,
	pub element_buffer: Option<ElementBufferVecDynamic<E>>,
	pub instance_buffer: Option<BufferVecDynamic<I>>,
	pub command_buffer: Option<BufferVecDynamic<C>>,
}

impl StaticMesh {
	pub fn new(glcore: Rc<GLCore>, primitive: PrimitiveMode, vertex_buffer: Buffer, element_buffer: Option<ElementBuffer>, instance_buffer: Option<Buffer>, command_buffer: Option<Buffer>) -> Self {
		Self {
			glcore,
			primitive,
			element_buffer,
			vertex_buffer,
			instance_buffer,
			command_buffer,
		}
	}
}

impl EditableMesh {
	pub fn new(glcore: Rc<GLCore>, primitive: PrimitiveMode, vertex_buffer: BufferVec, element_buffer: Option<ElementBufferVec>, instance_buffer: Option<BufferVec>, command_buffer: Option<BufferVec>) -> Self {
		Self {
			glcore,
			primitive,
			vertex_buffer,
			element_buffer,
			instance_buffer,
			command_buffer,
		}
	}
}

impl<T: BufferVecItem, E: BufferVecItem, I: BufferVecItem, C: DrawCommand> DynamicMesh<T, E, I, C> {
	pub fn new(glcore: Rc<GLCore>, primitive: PrimitiveMode, vertex_buffer: BufferVecDynamic<T>, element_buffer: Option<ElementBufferVecDynamic<E>>, instance_buffer: Option<BufferVecDynamic<I>>, command_buffer: Option<BufferVecDynamic<C>>) -> Self {
		Self {
			glcore,
			primitive,
			vertex_buffer,
			element_buffer,
			instance_buffer,
			command_buffer,
		}
	}
}

impl ElementType {
	pub fn get_size(&self) -> usize {
		match self {
			Self::U8 => 1,
			Self::U16 => 2,
			Self::U32 => 4,
		}
	}
}

impl From<StaticMesh> for EditableMesh {
	fn from(val: StaticMesh) -> Self {
		EditableMesh {
			glcore: val.glcore,
			primitive: val.primitive,
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
			primitive: val.primitive,
			vertex_buffer: val.vertex_buffer.into(),
			element_buffer: val.element_buffer.map(|b|b.into()),
			instance_buffer: val.instance_buffer.map(|b|b.into()),
			command_buffer: val.command_buffer.map(|b|b.into()),
		}
	}
}

impl<T: BufferVecItem, E: BufferVecItem, I: BufferVecItem, C: DrawCommand> From<StaticMesh> for DynamicMesh<T, E, I, C> {
	fn from(val: StaticMesh) -> Self {
		DynamicMesh {
			glcore: val.glcore,
			primitive: val.primitive,
			vertex_buffer: val.vertex_buffer.into(),
			element_buffer: val.element_buffer.map(|b|b.into()),
			instance_buffer: val.instance_buffer.map(|b|b.into()),
			command_buffer: val.command_buffer.map(|b|b.into()),
		}
	}
}

impl<T: BufferVecItem, E: BufferVecItem, I: BufferVecItem, C: DrawCommand> From<DynamicMesh<T, E, I, C>> for StaticMesh {
	fn from(val: DynamicMesh<T, E, I, C>) -> Self {
		StaticMesh {
			glcore: val.glcore,
			primitive: val.primitive,
			vertex_buffer: val.vertex_buffer.into(),
			element_buffer: val.element_buffer.map(|b|b.into()),
			instance_buffer: val.instance_buffer.map(|b|b.into()),
			command_buffer: val.command_buffer.map(|b|b.into()),
		}
	}
}

impl<T: BufferVecItem, E: BufferVecItem, I: BufferVecItem, C: DrawCommand> From<DynamicMesh<T, E, I, C>> for EditableMesh {
	fn from(val: DynamicMesh<T, E, I, C>) -> Self {
		EditableMesh {
			glcore: val.glcore,
			primitive: val.primitive,
			vertex_buffer: val.vertex_buffer.into(),
			element_buffer: val.element_buffer.map(|b|b.into()),
			instance_buffer: val.instance_buffer.map(|b|b.into()),
			command_buffer: val.command_buffer.map(|b|b.into()),
		}
	}
}

impl<T: BufferVecItem, E: BufferVecItem, I: BufferVecItem, C: DrawCommand> From<EditableMesh> for DynamicMesh<T, E, I, C> {
	fn from(val: EditableMesh) -> Self {
		DynamicMesh {
			glcore: val.glcore,
			primitive: val.primitive,
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
		.field("primitive", &self.primitive)
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
		.field("primitive", &self.primitive)
		.field("vertex_buffer", &self.vertex_buffer)
		.field("element_buffer", &self.element_buffer)
		.field("instance_buffer", &self.instance_buffer)
		.field("command_buffer", &self.command_buffer)
		.finish()
	}
}

impl<T: BufferVecItem, E: BufferVecItem, I: BufferVecItem, C: DrawCommand> Debug for DynamicMesh<T, E, I, C> {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		f.debug_struct("DynamicMesh")
		.field("primitive", &self.primitive)
		.field("vertex_buffer", &self.vertex_buffer)
		.field("element_buffer", &self.element_buffer)
		.field("instance_buffer", &self.instance_buffer)
		.field("command_buffer", &self.command_buffer)
		.finish()
	}
}

pub trait Mesh: Debug {
	fn get_glcore(&self) -> &GLCore;
	fn get_primitive(&self) -> PrimitiveMode;
	fn get_vertex_buffer(&self) -> &Buffer;
	fn get_element_buffer(&self) -> Option<ElementBufferRef>;
	fn get_instance_buffer(&self) -> Option<&Buffer>;
	fn get_command_buffer(&self) -> Option<&Buffer>;
}

impl Mesh for StaticMesh {
	fn get_glcore(&self) -> &GLCore {
		self.glcore.as_ref()
	}

	fn get_primitive(&self) -> PrimitiveMode {
		self.primitive
	}

	fn get_vertex_buffer(&self) -> &Buffer {
		&self.vertex_buffer
	}

	fn get_element_buffer(&self) -> Option<ElementBufferRef> {
		if let Some(buffer) = &self.element_buffer {
			Some(ElementBufferRef::new(buffer.get_buffer(), buffer.element_type))
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

	fn get_primitive(&self) -> PrimitiveMode {
		self.primitive
	}

	fn get_vertex_buffer(&self) -> &Buffer {
		self.vertex_buffer.get_buffer()
	}
	
	fn get_element_buffer(&self) -> Option<ElementBufferRef> {
		if let Some(buffer) = &self.element_buffer {
			Some(ElementBufferRef::new(buffer.get_buffer(), buffer.element_type))
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

impl<T: BufferVecItem, E: BufferVecItem, I: BufferVecItem, C: DrawCommand> Mesh for DynamicMesh<T, E, I, C> {
	fn get_glcore(&self) -> &GLCore {
		self.glcore.as_ref()
	}

	fn get_primitive(&self) -> PrimitiveMode {
		self.primitive
	}

	fn get_vertex_buffer(&self) -> &Buffer {
		self.vertex_buffer.get_buffer()
	}
	
	fn get_element_buffer(&self) -> Option<ElementBufferRef> {
		if let Some(buffer) = &self.element_buffer {
			Some(ElementBufferRef::new(buffer.get_buffer(), buffer.element_type))
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

