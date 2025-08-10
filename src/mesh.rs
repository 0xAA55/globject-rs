
use crate::prelude::*;
use std::{
	any::type_name,
	fmt::{self, Debug, Formatter},
	marker::PhantomData,
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

/// The mesh, every type of buffer is wrapped in a `BufferVec` can be manipulated just like a `Vec`
#[derive(Debug, Clone)]
pub struct Mesh<BV, V, BE, E, BI, I, BC, C>
where
	BV: BufferVec<V>,
	BE: BufferVec<E>,
	BI: BufferVec<I>,
	BC: BufferVec<C>,
	V: BufferVecItem,
	E: BufferVecItem,
	I: BufferVecItem,
	C: BufferVecItem {
	pub primitive: PrimitiveMode,
	pub vertex_buffer: BV,
	pub element_buffer: Option<BE>,
	pub instance_buffer: Option<BI>,
	pub command_buffer: Option<BC>,
	_vertex_type: PhantomData<V>,
	_element_type: PhantomData<E>,
	_instance_type: PhantomData<I>,
	_command_type: PhantomData<C>,
}

/// The most typical static mesh type: use `BufferVecStatic` for vertices and elements(indices), use `BufferVecDynamic` for instances and draw commands
pub type StaticMesh<V, E, I, C> = Mesh<BufferVecStatic<V>, V, BufferVecStatic<E>, E, BufferVecDynamic<I>, I, BufferVecDynamic<C>, C>;

impl<BV, V, BE, E, BI, I, BC, C> Mesh<BV, V, BE, E, BI, I, BC, C>
where
	BV: BufferVec<V>,
	BE: BufferVec<E>,
	BI: BufferVec<I>,
	BC: BufferVec<C>,
	V: BufferVecItem,
	E: BufferVecItem,
	I: BufferVecItem,
	C: BufferVecItem {
	/// Create a new mesh from the buffers
	pub fn new(primitive: PrimitiveMode, vertex_buffer: BV, element_buffer: Option<BE>, instance_buffer: Option<BI>, command_buffer: Option<BC>) -> Self {
		Self {
			primitive,
			vertex_buffer,
			element_buffer,
			instance_buffer,
			command_buffer,
			_vertex_type: PhantomData,
			_element_type: PhantomData,
			_instance_type: PhantomData,
			_command_type: PhantomData,
		}
	}

	/// Flush all of the buffers' caches to the GPU
	pub fn flush(&mut self) -> Result<(), GLCoreError> {
		self.vertex_buffer.flush()?;
		if let Some(element_buffer) = &mut self.element_buffer {
			element_buffer.flush()?;
		}
		if let Some(instance_buffer) = &mut self.instance_buffer {
			instance_buffer.flush()?;
		}
		if let Some(command_buffer) = &mut self.command_buffer {
			command_buffer.flush()?;
		}
		Ok(())
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

/// The `GenericMesh` trait helps the `Mesh` struct to be able to turn into an object
pub trait GenericMesh: Debug {
	/// Get the primitive mode of the mesh
	fn get_primitive(&self) -> PrimitiveMode;

	/// Get the vertex buffer of the mesh
	fn get_vertex_buffer(&self) -> &Buffer;

	/// Get the element buffer of the mesh
	fn get_element_buffer(&self) -> Option<&Buffer>;

	/// Get the type of the element buffer
	fn get_element_type(&self) -> ElementType;

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
	fn flush(&mut self) -> Result<(), GLCoreError> {Ok(())}

	/// Bind the vertex buffer
	fn bind_vertex_buffer<'a>(&'a self) -> Result<BufferBind<'a>, GLCoreError> {
		self.get_vertex_buffer().bind_to(BufferTarget::ArrayBuffer)
	}

	/// Bind the element buffer
	fn bind_element_buffer<'a>(&'a self) -> Result<Option<BufferBind<'a>>, GLCoreError> {
		if let Some(element_buffer) = self.get_element_buffer() {
			Ok(Some(element_buffer.bind_to(BufferTarget::ElementArrayBuffer)?))
		} else {
			Ok(None)
		}
	}

	/// Bind the instance buffer
	fn bind_instance_buffer<'a>(&'a self) -> Result<Option<BufferBind<'a>>, GLCoreError> {
		if let Some(instance_buffer) = self.get_instance_buffer() {
			Ok(Some(instance_buffer.bind_to(BufferTarget::ArrayBuffer)?))
		} else {
			Ok(None)
		}
	}

	/// Bind the command buffer
	fn bind_command_buffer<'a>(&'a self) -> Result<Option<BufferBind<'a>>, GLCoreError> {
		if let Some(command_buffer) = self.get_command_buffer() {
			Ok(Some(command_buffer.bind_to(BufferTarget::DrawIndirectBuffer)?))
		} else {
			Ok(None)
		}
	}
}

impl<BV, V, BE, E, BI, I, BC, C> GenericMesh for Mesh<BV, V, BE, E, BI, I, BC, C>
where
	BV: BufferVec<V>,
	BE: BufferVec<E>,
	BI: BufferVec<I>,
	BC: BufferVec<C>,
	V: BufferVecItem,
	E: BufferVecItem,
	I: BufferVecItem,
	C: BufferVecItem {
	fn get_primitive(&self) -> PrimitiveMode {
		self.primitive
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

	fn get_element_type(&self) -> ElementType {
		match size_of::<E>() {
			1 => ElementType::U8,
			2 => ElementType::U16,
			4 => ElementType::U32,
			_ => panic!("Unsupported element type: {}", type_name::<E>()),
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

	fn get_vertex_stride(&self) -> usize {
		size_of::<V>()
	}

	fn get_instance_stride(&self) -> usize {
		size_of::<I>()
	}

	fn get_vertex_count(&self) -> usize {
		self.vertex_buffer.len()
	}

	fn get_element_count(&self) -> usize {
		if let Some(element_buffer) = &self.element_buffer {
			element_buffer.len()
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

	fn flush(&mut self) -> Result<(), GLCoreError> {
		Mesh::flush(self)
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
pub struct MeshWithMaterial<M: GenericMesh, Mat: Material> {
	material: Rc<Mat>,
	mesh: M,
}

impl<M: GenericMesh, Mat: Material> MeshWithMaterial<M, Mat> {
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

impl<M: GenericMesh, Mat: Material> GenericMesh for MeshWithMaterial<M, Mat> {
	fn get_primitive(&self) -> PrimitiveMode {
		self.mesh.get_primitive()
	}

	fn get_vertex_buffer(&self) -> &Buffer {
		self.mesh.get_vertex_buffer()
	}

	fn get_element_buffer(&self) -> Option<&Buffer> {
		self.mesh.get_element_buffer()
	}

	fn get_element_type(&self) -> ElementType {
		self.mesh.get_element_type()
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

/// The `GenericMeshWithMaterial` trait helps the `MeshWithMaterial` struct to be able to turn into an object
pub trait GenericMeshWithMaterial: GenericMesh {
	fn get_material(&self) -> Option<&dyn Material>;
}

impl<BV, V, BE, E, BI, I, BC, C> GenericMeshWithMaterial for Mesh<BV, V, BE, E, BI, I, BC, C>
where
	BV: BufferVec<V>,
	BE: BufferVec<E>,
	BI: BufferVec<I>,
	BC: BufferVec<C>,
	V: BufferVecItem,
	E: BufferVecItem,
	I: BufferVecItem,
	C: BufferVecItem {
	fn get_material(&self) -> Option<&dyn Material> {
		None
	}
}

impl<M: GenericMesh, Mat: Material> GenericMeshWithMaterial for MeshWithMaterial<M, Mat> {
	fn get_material(&self) -> Option<&dyn Material> {
		Some(&*self.material)
	}
}
