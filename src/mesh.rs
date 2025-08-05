
use glcore::*;
use crate::glbuffer::*;
use crate::glcmdbuf::*;
use crate::buffervec::*;
use crate::material::*;
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

#[derive(Debug, Clone)]
pub struct StaticMesh {
	pub primitive: PrimitiveMode,
	pub vertex_buffer: Buffer,
	pub element_buffer: Option<ElementBuffer>,
	pub instance_buffer: Option<Buffer>,
	pub command_buffer: Option<Buffer>,
}

#[derive(Debug, Clone)]
pub struct EditableMesh {
	pub primitive: PrimitiveMode,
	pub vertex_buffer: BufferVec,
	pub element_buffer: Option<ElementBufferVec>,
	pub instance_buffer: Option<BufferVec>,
	pub command_buffer: Option<BufferVec>,
}

#[derive(Debug, Clone)]
pub struct DynamicMesh<T: BufferVecItem, E: BufferVecItem, I: BufferVecItem, C: DrawCommand> {
	pub primitive: PrimitiveMode,
	pub vertex_buffer: BufferVecDynamic<T>,
	pub element_buffer: Option<ElementBufferVecDynamic<E>>,
	pub instance_buffer: Option<BufferVecDynamic<I>>,
	pub command_buffer: Option<BufferVecDynamic<C>>,
}

impl StaticMesh {
	pub fn new(primitive: PrimitiveMode, vertex_buffer: Buffer, element_buffer: Option<ElementBuffer>, instance_buffer: Option<Buffer>, command_buffer: Option<Buffer>) -> Self {
		Self {
			primitive,
			element_buffer,
			vertex_buffer,
			instance_buffer,
			command_buffer,
		}
	}
}

impl EditableMesh {
	pub fn new(primitive: PrimitiveMode, vertex_buffer: BufferVec, element_buffer: Option<ElementBufferVec>, instance_buffer: Option<BufferVec>, command_buffer: Option<BufferVec>) -> Self {
		Self {
			primitive,
			vertex_buffer,
			element_buffer,
			instance_buffer,
			command_buffer,
		}
	}
}

impl<T: BufferVecItem, E: BufferVecItem, I: BufferVecItem, C: DrawCommand> DynamicMesh<T, E, I, C> {
	pub fn new(primitive: PrimitiveMode, vertex_buffer: BufferVecDynamic<T>, element_buffer: Option<ElementBufferVecDynamic<E>>, instance_buffer: Option<BufferVecDynamic<I>>, command_buffer: Option<BufferVecDynamic<C>>) -> Self {
		Self {
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

impl ElementBuffer {
	pub fn get_buffer(&self) -> &Buffer {
		&self.buffer
	}

	pub fn get_type(&self) -> ElementType {
		self.element_type
	}

	pub fn get_num_elements(&self) -> usize {
		self.buffer.size() / self.element_type.get_size()
	}

	pub fn bind<'a>(&'a self) -> BufferBind<'a> {
		self.buffer.bind()
	}
}

impl ElementBufferVec {
	pub fn get_buffer(&self) -> &Buffer {
		self.buffer.get_buffer()
	}

	pub fn get_type(&self) -> ElementType {
		self.element_type
	}

	pub fn get_num_elements(&self) -> usize {
		self.buffer.size_in_bytes() / self.element_type.get_size()
	}

	pub fn bind<'a>(&'a self) -> BufferBind<'a> {
		self.buffer.bind()
	}
}

impl<T: BufferVecItem> ElementBufferVecDynamic<T> {
	pub fn get_buffer(&self) -> &Buffer {
		self.buffer.get_buffer()
	}

	pub fn get_type(&self) -> ElementType {
		self.element_type
	}

	pub fn get_num_elements(&self) -> usize {
		self.buffer.len()
	}

	pub fn bind<'a>(&'a self) -> BufferBind<'a> {
		self.buffer.bind()
	}
}

impl<'a> ElementBufferRef<'a> {
	pub fn new(buffer: &'a Buffer, element_type: ElementType) -> Self {
		Self {
			buffer,
			element_type,
		}
	}

	pub fn get_type(&self) -> ElementType {
		self.element_type
	}

	pub fn get_num_elements(&self) -> usize {
		self.buffer.size() / self.element_type.get_size()
	}

	pub fn bind(&self) -> BufferBind<'a> {
		self.buffer.bind()
	}
}

impl From<ElementBuffer> for ElementBufferVec{
	fn from(val: ElementBuffer) -> Self {
		Self {
			buffer: val.buffer.into(),
			element_type: val.element_type,
		}
	}
}

impl From<ElementBufferVec> for ElementBuffer{
	fn from(val: ElementBufferVec) -> Self {
		Self {
			buffer: val.buffer.into(),
			element_type: val.element_type,
		}
	}
}

impl<T: BufferVecItem> From<ElementBuffer> for ElementBufferVecDynamic<T> {
	fn from(val: ElementBuffer) -> Self {
		Self {
			buffer: val.buffer.into(),
			element_type: val.element_type,
		}
	}
}

impl<T: BufferVecItem> From<ElementBufferVec> for ElementBufferVecDynamic<T> {
	fn from(val: ElementBufferVec) -> Self {
		Self {
			buffer: val.buffer.into(),
			element_type: val.element_type,
		}
	}
}

impl<T: BufferVecItem> From<ElementBufferVecDynamic<T>> for ElementBuffer {
	fn from(val: ElementBufferVecDynamic<T>) -> Self {
		Self {
			buffer: val.buffer.into(),
			element_type: val.element_type,
		}
	}
}

impl<T: BufferVecItem> From<ElementBufferVecDynamic<T>> for ElementBufferVec {
	fn from(val: ElementBufferVecDynamic<T>) -> Self {
		Self {
			buffer: val.buffer.into(),
			element_type: val.element_type,
		}
	}
}

impl From<StaticMesh> for EditableMesh {
	fn from(val: StaticMesh) -> Self {
		EditableMesh {
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
			primitive: val.primitive,
			vertex_buffer: val.vertex_buffer.into(),
			element_buffer: val.element_buffer.map(|b|b.into()),
			instance_buffer: val.instance_buffer.map(|b|b.into()),
			command_buffer: val.command_buffer.map(|b|b.into()),
		}
	}
}

pub trait Mesh: Debug {
	fn get_primitive(&self) -> PrimitiveMode;
	fn get_vertex_buffer(&self) -> &Buffer;
	fn get_element_buffer(&self) -> Option<ElementBufferRef>;
	fn get_instance_buffer(&self) -> Option<&Buffer>;
	fn get_command_buffer(&self) -> Option<&Buffer>;
}

impl Mesh for StaticMesh {
	fn get_primitive(&self) -> PrimitiveMode {
		self.primitive
	}

	fn get_vertex_buffer(&self) -> &Buffer {
		&self.vertex_buffer
	}

	fn get_element_buffer(&self) -> Option<ElementBufferRef> {
		self.element_buffer.as_ref().map(|buffer| ElementBufferRef::new(buffer.get_buffer(), buffer.element_type))
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
	fn get_primitive(&self) -> PrimitiveMode {
		self.primitive
	}

	fn get_vertex_buffer(&self) -> &Buffer {
		self.vertex_buffer.get_buffer()
	}
	
	fn get_element_buffer(&self) -> Option<ElementBufferRef> {
		self.element_buffer.as_ref().map(|buffer| ElementBufferRef::new(buffer.get_buffer(), buffer.element_type))
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
	fn get_primitive(&self) -> PrimitiveMode {
		self.primitive
	}

	fn get_vertex_buffer(&self) -> &Buffer {
		self.vertex_buffer.get_buffer()
	}
	
	fn get_element_buffer(&self) -> Option<ElementBufferRef> {
		self.element_buffer.as_ref().map(|buffer| ElementBufferRef::new(buffer.get_buffer(), buffer.element_type))
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

impl Debug for PrimitiveMode {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match self {
			Self::Points => write!(f, "Points"),
			Self::LineStrip => write!(f, "Line Strip"),
			Self::LineLoop => write!(f, "Line Loop"),
			Self::Lines => write!(f, "Lines"),
			Self::LineStripAdjacency => write!(f, "Line Strip Adjacency"),
			Self::LinesAdjacency => write!(f, "Lines Adjacency"),
			Self::TriangleStrip => write!(f, "Triangle Strip"),
			Self::TriangleFan => write!(f, "Triangle Fan"),
			Self::Triangles => write!(f, "Triangles"),
			Self::TriangleStripAdjacency => write!(f, "Triangle Strip Adjacency"),
			Self::TrianglesAdjacency => write!(f, "Triangles Adjacency"),
			Self::Patches => write!(f, "Patches"),
		}
	}
}

impl Debug for ElementType {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match self {
			Self::U8 =>  write!(f, "U8"),
			Self::U16 => write!(f, "U16"),
			Self::U32 => write!(f, "U32"),
		}
	}
}

#[derive(Debug, Clone)]
pub struct MeshWithMaterial<M: Mesh, Mat: Material> {
	material: Rc<Mat>,
	mesh: M,
}

impl<M: Mesh, Mat: Material> MeshWithMaterial<M, Mat> {
	pub fn new(mesh: M, material: Rc<Mat>) -> Self {
		Self {
			material,
			mesh,
		}
	}
}

impl<M: Mesh, Mat: Material> Mesh for MeshWithMaterial<M, Mat> {
	fn get_primitive(&self) -> PrimitiveMode {
		self.mesh.get_primitive()
	}

	fn get_vertex_buffer(&self) -> &Buffer {
		self.mesh.get_vertex_buffer()
	}

	fn get_element_buffer(&self) -> Option<ElementBufferRef> {
		self.mesh.get_element_buffer()
	}

	fn get_instance_buffer(&self) -> Option<&Buffer> {
		self.mesh.get_instance_buffer()
	}

	fn get_command_buffer(&self) -> Option<&Buffer> {
		self.mesh.get_command_buffer()
	}
}
