
use crate::prelude::*;
use std::{
	fmt::{self, Debug, Formatter},
	rc::Rc,
};

/// The primitive mode of the mesh, indicating how to draw the vertices to which type of the shapes
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

/// The type of the element buffer of the mesh, indicating the vertices were indexed by which type of index
#[derive(Clone, Copy, PartialEq)]
pub enum ElementType {
	U8 = GL_UNSIGNED_BYTE as isize,
	U16 = GL_UNSIGNED_SHORT as isize,
	U32 = GL_UNSIGNED_INT as isize,
}

/// The element buffer for the mesh has `element_type` to tell the format of the indices
#[derive(Debug, Clone)]
pub struct ElementBuffer {
	pub buffer: Buffer,
	pub element_type: ElementType,
}

/// The `BufferVec` variant of the `ElementBuffer`
#[derive(Debug, Clone)]
pub struct ElementBufferVec {
	pub buffer: BufferVec,
	pub element_type: ElementType,
}

/// The `BufferVecDynamic` variant of the `ElementBuffer`
#[derive(Debug, Clone)]
pub struct ElementBufferVecDynamic<T: BufferVecItem> {
	pub buffer: BufferVecDynamic<T>,
	pub element_type: ElementType,
}

/// A reference to a buffer with the format of the indices
#[derive(Debug, Clone)]
pub struct ElementBufferRef<'a> {
	pub buffer: &'a Buffer,
	pub element_type: ElementType,
}

/// The static mesh, to manipulate the data of the mesh, must explicitly manipulate the `Buffer` data in bytes of each type of the buffer
/// The mesh is considered not to be changed frequently
/// Data was only stored in the GPU; every update to the buffer caused data to be transferred to the GPU
#[derive(Debug, Clone)]
pub struct StaticMesh {
	pub primitive: PrimitiveMode,
	pub vertex_buffer: Buffer,
	pub element_buffer: Option<ElementBuffer>,
	pub instance_buffer: Option<Buffer>,
	pub command_buffer: Option<Buffer>,
	pub vertex_stride: usize,
	pub instance_stride: usize,
}

/// The editable mesh, every type of buffer is wrapped in a `BufferVec`, can be manipulated slightly easier than the static mesh
/// The mesh is considered not to be changed frequently
/// Data was only stored in the GPU; every update to the buffer caused data to be transferred to the GPU
#[derive(Debug, Clone)]
pub struct EditableMesh {
	pub primitive: PrimitiveMode,
	pub vertex_buffer: BufferVec,
	pub element_buffer: Option<ElementBufferVec>,
	pub instance_buffer: Option<BufferVec>,
	pub command_buffer: Option<BufferVec>,
	pub vertex_stride: usize,
	pub instance_stride: usize,
}

/// The dynamic mesh, every type of buffer is wrapped in a `BufferVecDynamic` can be manipulated just like a `Vec`
/// There are caches in the system memory to be updated by indexing/slicing, while the buffers remember every item's updated flags using a bitmap
/// When the `flush()` method is called, every buffer flushes its cache to the GPU. Only the changed part was flushed
#[derive(Debug, Clone)]
pub struct DynamicMesh<T: BufferVecItem, E: BufferVecItem, I: BufferVecItem, C: DrawCommand> {
	pub primitive: PrimitiveMode,
	pub vertex_buffer: BufferVecDynamic<T>,
	pub element_buffer: Option<ElementBufferVecDynamic<E>>,
	pub instance_buffer: Option<BufferVecDynamic<I>>,
	pub command_buffer: Option<BufferVecDynamic<C>>,
}

impl StaticMesh {
	/// Create a new static mesh from the buffers
	pub fn new(primitive: PrimitiveMode, vertex_buffer: Buffer, vertex_stride: usize, element_buffer: Option<ElementBuffer>, instance_buffer: Option<Buffer>, instance_stride: usize, command_buffer: Option<Buffer>) -> Self {
		Self {
			primitive,
			element_buffer,
			vertex_buffer,
			instance_buffer,
			command_buffer,
			vertex_stride,
			instance_stride,
		}
	}
}

impl EditableMesh {
	/// Create a new editable mesh from the buffers
	pub fn new(primitive: PrimitiveMode, vertex_buffer: BufferVec, vertex_stride: usize, element_buffer: Option<ElementBufferVec>, instance_buffer: Option<BufferVec>, instance_stride: usize, command_buffer: Option<BufferVec>) -> Self {
		Self {
			primitive,
			vertex_buffer,
			element_buffer,
			instance_buffer,
			command_buffer,
			vertex_stride,
			instance_stride,
		}
	}
}

impl<T: BufferVecItem, E: BufferVecItem, I: BufferVecItem, C: DrawCommand> DynamicMesh<T, E, I, C> {
	/// Create a new dynamic mesh from the buffers
	pub fn new(primitive: PrimitiveMode, vertex_buffer: BufferVecDynamic<T>, element_buffer: Option<ElementBufferVecDynamic<E>>, instance_buffer: Option<BufferVecDynamic<I>>, command_buffer: Option<BufferVecDynamic<C>>) -> Self {
		Self {
			primitive,
			vertex_buffer,
			element_buffer,
			instance_buffer,
			command_buffer,
		}
	}

	/// Flush all of the buffers' caches to the GPU
	pub fn flush(&mut self) {
		self.vertex_buffer.flush();
		self.element_buffer.as_mut().map(|b|b.flush());
		self.instance_buffer.as_mut().map(|b|b.flush());
		self.command_buffer.as_mut().map(|b|b.flush());
	}
}

impl ElementType {
	/// Get the size of each index
	pub fn get_size(&self) -> usize {
		match self {
			Self::U8 => 1,
			Self::U16 => 2,
			Self::U32 => 4,
		}
	}
}

pub trait ElementBufferCommon: Debug {
	/// Retrieve the underlying buffer
	fn get_buffer(&self) -> &Buffer;

	/// Get the type of the elements
	fn get_type(&self) -> ElementType;

	/// Get how many of elements in the buffer
	fn get_num_elements(&self) -> usize;

	/// Bind the buffer, create a binding guard to manage the binding state
	fn bind<'a>(&'a self) -> BufferBind<'a> {
		self.get_buffer().bind()
	}

	/// Bind the buffer to a specified target, create a binding guard to manage the binding state
	fn bind_to<'a>(&'a self, target: BufferTarget) -> BufferBind<'a> {
		self.get_buffer().bind_to(target)
	}

	/// Flush the cache if the element buffer has a caching system
	fn flush(&mut self) {}
}

impl ElementBufferCommon for ElementBuffer {
	fn get_buffer(&self) -> &Buffer {
		&self.buffer
	}

	fn get_type(&self) -> ElementType {
		self.element_type
	}

	fn get_num_elements(&self) -> usize {
		self.buffer.size() / self.element_type.get_size()
	}
}

impl ElementBufferCommon for ElementBufferVec {
	fn get_buffer(&self) -> &Buffer {
		self.buffer.get_buffer()
	}

	fn get_type(&self) -> ElementType {
		self.element_type
	}

	fn get_num_elements(&self) -> usize {
		self.buffer.size_in_bytes() / self.element_type.get_size()
	}
}

impl<T: BufferVecItem> ElementBufferCommon for ElementBufferVecDynamic<T> {
	fn get_buffer(&self) -> &Buffer {
		self.buffer.get_buffer()
	}

	fn get_type(&self) -> ElementType {
		self.element_type
	}

	fn get_num_elements(&self) -> usize {
		self.buffer.len()
	}

	fn flush(&mut self) {
		self.buffer.flush()
	}
}

impl<'a> ElementBufferRef<'a>  {
	/// Create a new `ElementBufferRef`
	pub fn new(buffer: &'a Buffer, element_type: ElementType) -> Self {
		Self {
			buffer,
			element_type,
		}
	}
}

impl<'a> ElementBufferCommon for ElementBufferRef<'a> {
	fn get_buffer(&self) -> &Buffer {
		&self.buffer
	}

	fn get_type(&self) -> ElementType {
		self.element_type
	}

	fn get_num_elements(&self) -> usize {
		self.buffer.size() / self.element_type.get_size()
	}
}

impl From<ElementBuffer> for ElementBufferVec {
	fn from(val: ElementBuffer) -> Self {
		Self {
			buffer: val.buffer.into(),
			element_type: val.element_type,
		}
	}
}

impl From<ElementBufferVec> for ElementBuffer {
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
			vertex_stride: val.vertex_stride,
			instance_stride: val.instance_stride,
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
			vertex_stride: val.vertex_stride,
			instance_stride: val.instance_stride,
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
			vertex_stride: size_of::<T>(),
			instance_stride: size_of::<I>(),
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
			vertex_stride: size_of::<T>(),
			instance_stride: size_of::<I>(),
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
	/// Get the primitive mode of the mesh
	fn get_primitive(&self) -> PrimitiveMode;

	/// Get the vertex buffer of the mesh
	fn get_vertex_buffer(&self) -> &Buffer;

	/// Get the element buffer of the mesh
	fn get_element_buffer(&self) -> Option<ElementBufferRef>;

	/// Get the instance buffer of the mesh
	fn get_instance_buffer(&self) -> Option<&Buffer>;

	/// Get the draw command buffer of the mesh
	fn get_command_buffer(&self) -> Option<&Buffer>;

	/// Get the size of each vertex
	fn get_vertex_stride(&self) -> usize;

	/// Get the size of each instance
	fn get_instance_stride(&self) -> usize;

	/// Get the number of vertices
	fn get_vertex_count(&self) -> usize;

	/// Get the number of the elements
	fn get_element_count(&self) -> usize;

	/// Get the number of the instances
	fn get_instance_count(&self) -> usize;

	/// Get the number of the draw commands
	fn get_command_count(&self) -> usize;

	/// Flush the cache if the mesh has a caching system
	fn flush(&mut self) {}

	/// Bind the vertex buffer
	fn bind_vertex_buffer<'a>(&'a self) -> BufferBind<'a> {
		self.get_vertex_buffer().bind_to(BufferTarget::ArrayBuffer)
	}

	/// Bind the element buffer
	fn bind_element_buffer<'a>(&'a self) -> Option<BufferBind<'a>> {
		self.get_element_buffer().map(|b|b.buffer.bind_to(BufferTarget::ElementArrayBuffer))
	}

	/// Bind the instance buffer
	fn bind_instance_buffer<'a>(&'a self) -> Option<BufferBind<'a>> {
		self.get_instance_buffer().map(|b|b.bind_to(BufferTarget::ArrayBuffer))
	}

	/// Bind the command buffer
	fn bind_command_buffer<'a>(&'a self) -> Option<BufferBind<'a>> {
		self.get_command_buffer().map(|b|b.bind_to(BufferTarget::DrawIndirectBuffer))
	}
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

	fn get_vertex_stride(&self) -> usize {
		self.vertex_stride
	}

	fn get_instance_stride(&self) -> usize {
		self.instance_stride
	}

	fn get_vertex_count(&self) -> usize {
		self.vertex_buffer.size() / self.vertex_stride
	}

	fn get_element_count(&self) -> usize {
		if let Some(element_buffer) = &self.element_buffer {
			element_buffer.get_num_elements()
		} else {
			0
		}
	}

	fn get_instance_count(&self) -> usize {
		if let Some(buffer) = &self.instance_buffer {
			buffer.size() / self.instance_stride
		} else {
			0
		}
	}

	fn get_command_count(&self) -> usize {
		if let Some(buffer) = &self.command_buffer {
			if let Some(_) = &self.element_buffer {
				buffer.size() / size_of::<DrawElementsCommand>()
			} else {
				buffer.size() / size_of::<DrawArrayCommand>()
			}
		} else {
			0
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

	fn get_vertex_stride(&self) -> usize {
		self.vertex_stride
	}

	fn get_instance_stride(&self) -> usize {
		self.instance_stride
	}

	fn get_vertex_count(&self) -> usize {
		self.vertex_buffer.size_in_bytes() / self.vertex_stride
	}

	fn get_element_count(&self) -> usize {
		if let Some(element_buffer) = &self.element_buffer {
			element_buffer.get_num_elements()
		} else {
			0
		}
	}

	fn get_instance_count(&self) -> usize {
		if let Some(buffer) = &self.instance_buffer {
			buffer.size_in_bytes() / self.instance_stride
		} else {
			0
		}
	}

	fn get_command_count(&self) -> usize {
		if let Some(buffer) = &self.command_buffer {
			if let Some(_) = &self.element_buffer {
				buffer.size_in_bytes() / size_of::<DrawElementsCommand>()
			} else {
				buffer.size_in_bytes() / size_of::<DrawArrayCommand>()
			}
		} else {
			0
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

	fn get_vertex_stride(&self) -> usize {
		size_of::<T>()
	}

	fn get_instance_stride(&self) -> usize {
		size_of::<I>()
	}

	fn get_vertex_count(&self) -> usize {
		self.vertex_buffer.len()
	}

	fn get_element_count(&self) -> usize {
		if let Some(element_buffer) = &self.element_buffer {
			element_buffer.get_num_elements()
		} else {
			0
		}
	}

	fn get_instance_count(&self) -> usize {
		if let Some(buffer) = &self.instance_buffer {
			buffer.len()
		} else {
			0
		}
	}

	fn get_command_count(&self) -> usize {
		if let Some(buffer) = &self.command_buffer {
			buffer.len()
		} else {
			0
		}
	}

	fn flush(&mut self) {
		DynamicMesh::<T, E, I, C>::flush(self);
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

	pub fn get_material(&self) -> &Mat {
		&self.material
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

	fn get_vertex_stride(&self) -> usize {
		self.mesh.get_vertex_stride()
	}

	fn get_instance_stride(&self) -> usize {
		self.mesh.get_instance_stride()
	}

	fn get_vertex_count(&self) -> usize {
		self.mesh.get_vertex_count()
	}

	fn get_element_count(&self) -> usize {
		self.mesh.get_element_count()
	}

	fn get_instance_count(&self) -> usize {
		self.mesh.get_instance_count()
	}

	fn get_command_count(&self) -> usize {
		self.mesh.get_command_count()
	}
}
