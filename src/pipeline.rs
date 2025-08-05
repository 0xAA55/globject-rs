
use crate::prelude::*;
use std::{
	any::Any,
	collections::BTreeMap,
	ffi::{CString, c_void},
	fmt::{self, Debug, Formatter},
	marker::PhantomData,
	mem::size_of,
	ptr::null,
	rc::Rc,
};
use half::f16;
use glm::*;

pub trait VertexType: Copy + Clone + Sized + Default + Debug + Iterable {}
impl<T> VertexType for T where T: Copy + Clone + Sized + Default + Debug + Iterable {}

/// Use this macro to convert your struct into `VertexType`
#[macro_export]
macro_rules! derive_vertex_type {
	($item: item) => {
		#[derive(Iterable, Default, Debug, Clone, Copy)]
		$item
	};
}

pub struct Pipeline<V: VertexType, I: VertexType, M: Mesh, Mat: Material> {
	pub glcore: Rc<GLCore>,
	name: u32,
	pub mesh: Rc<MeshWithMaterial<M, Mat>>,
	pub shader: Rc<Shader>,
	vertex_stride: usize,
	instance_stride: usize,
	_phantom_vertex_type: PhantomData<V>,
	_phantom_instance_type: PhantomData<I>,
}

#[derive(Debug, Clone, Copy)]
struct DataGlType {
	data_type: u32,
	size: u32,
	rows: u32,
}

#[derive(Debug)]
pub struct PipelineBind<'a, V: VertexType, I: VertexType, M: Mesh, Mat: Material> {
	pub pipeline: &'a Pipeline<V, I, M, Mat>,
}

impl DataGlType {
	fn is_integer(&self) -> bool {
		matches!(self.data_type, GL_BYTE | GL_SHORT | GL_INT | GL_UNSIGNED_BYTE | GL_UNSIGNED_SHORT | GL_UNSIGNED_INT)
	}

	fn is_double(&self) -> bool {
		matches!(self.data_type, GL_DOUBLE)
	}

	fn size_in_bytes(&self) -> usize {
		match self.data_type {
			GL_BYTE | GL_UNSIGNED_BYTE => (self.size as usize) * self.rows as usize,
			GL_SHORT | GL_UNSIGNED_SHORT | GL_HALF_FLOAT => 2usize * self.size as usize * self.rows as usize,
			GL_INT | GL_UNSIGNED_INT | GL_FLOAT => 4usize * self.size as usize * self.rows as usize,
			GL_DOUBLE => 8usize * self.size as usize * self.rows as usize,
			other => panic!("Invalid `data_type` ({other})"),
		}
	}
}

impl<V: VertexType, I: VertexType, M: Mesh, Mat: Material> Pipeline<V, I, M, Mat> {
	/// Get the internal name
	pub fn get_name(&self) -> u32 {
		self.name
	}

	pub fn new(glcore: Rc<GLCore>, mesh: Rc<MeshWithMaterial<M, Mat>>, shader: Rc<Shader>) -> Self {
		let mut name: u32 = 0;
		glcore.glGenVertexArrays(1, &mut name as *mut u32);
		let mut ret = Self {
			glcore,
			name,
			mesh,
			shader,
			vertex_stride: size_of::<V>(),
			instance_stride: size_of::<I>(),
			_phantom_vertex_type: PhantomData,
			_phantom_instance_type: PhantomData,
		};
		ret.establish_pipeline();
		ret
	}

	fn establish_pipeline(&mut self) {
		let program = self.shader.use_program();
		let active_attribs = self.shader.get_active_attribs().unwrap();
		let bind = self.bind();

		let vb_bind = self.mesh.get_vertex_buffer().bind();
		self.describe::<V>(&active_attribs, 0);
		vb_bind.unbind();

		if let Some(ib) = self.mesh.get_instance_buffer() {
			let ib_bind = ib.bind();
			self.describe::<I>(&active_attribs, 1);
			ib_bind.unbind();
		}

		bind.unbind();
		program.unuse();
	}

	fn describe<T: VertexType>(&self, active_attribs: &BTreeMap<String, AttribVarType>, v_a_d: u32) {
		let instance = T::default();
		let stride = size_of::<T>();
		let mut cur_offset: usize = 0;
		for (field_name, field_value) in instance.iter() {
			let typename = Self::get_typename_of_vertex_struct_member(field_value);
			let datainfo = Self::get_vertex_struct_member_gltype(typename);
			if let Some(attrib_type) = active_attribs.get(field_name) {
				let (p_size, p_rows) = attrib_type.get_size_and_rows();
				if p_size != datainfo.size || p_rows != datainfo.rows {
					panic!("The size and rows of the shader attrib is {p_size}x{p_rows}, but the given member of the vertex struct is {}x{}", datainfo.size, datainfo.rows);
				}
				let c_field_name = CString::new(field_name).unwrap();
				let location = self.glcore.glGetAttribLocation(self.shader.get_name(), c_field_name.as_ptr());
				if location >= 0 {
					let location = location as u32;
					for row in 0..datainfo.rows {
						let location = location + row;
						let do_normalize = if field_name.contains("normalized") && field_name.contains("_") {
							1
						} else {
							0
						};
						let ptr_param = cur_offset as *const c_void;
						self.glcore.glEnableVertexAttribArray(location);
						if attrib_type.is_float() {
							self.glcore.glVertexAttribPointer(location, p_size as i32, attrib_type.get_base_type() as u32, do_normalize, stride as i32, ptr_param);
						} else if attrib_type.is_integer() {
							self.glcore.glVertexAttribIPointer(location, p_size as i32, attrib_type.get_base_type() as u32, stride as i32, ptr_param);
						} else if attrib_type.is_double() {
							self.glcore.glVertexAttribLPointer(location, p_size as i32, attrib_type.get_base_type() as u32, stride as i32, ptr_param);
						} else {
							panic!("Unknown data type of the attrib `{} {field_name}`", attrib_type.get_type());
						}
						self.glcore.glVertexAttribDivisor(location, v_a_d);
					}
				} else {
					eprintln!("Attrib `{typename} {field_name}` is active, but can't get its location.");
				}
			} else {
				eprintln!("Attrib `{typename} {field_name}` is not active.");
			}
			cur_offset += datainfo.size_in_bytes();
		}
	}

	pub fn bind<'a>(&'a self) -> PipelineBind<'a, V, I, M, Mat> {
		PipelineBind::new(self)
	}

	fn get_vertex_struct_member_gltype(member_type: &str) -> DataGlType {
		match member_type {
			"i8" => DataGlType{data_type: GL_BYTE, size: 1, rows: 1},
			"i16" => DataGlType{data_type: GL_SHORT, size: 1, rows: 1},
			"i32" => DataGlType{data_type: GL_INT, size: 1, rows: 1},
			"u8" => DataGlType{data_type: GL_UNSIGNED_BYTE, size: 1, rows: 1},
			"u16" => DataGlType{data_type: GL_UNSIGNED_SHORT, size: 1, rows: 1},
			"u32" => DataGlType{data_type: GL_UNSIGNED_INT, size: 1, rows: 1},
			"f16" => DataGlType{data_type: GL_HALF_FLOAT, size: 1, rows: 1},
			"f32" => DataGlType{data_type: GL_FLOAT, size: 1, rows: 1},
			"f64" => DataGlType{data_type: GL_DOUBLE, size: 1, rows: 1},
			_ => {
				if member_type.contains("vec") {
					let data_type =
					     if member_type.starts_with("u32") {GL_UNSIGNED_INT}
					else if member_type.starts_with("u16") {GL_UNSIGNED_SHORT}
					else if member_type.starts_with("u8")  {GL_UNSIGNED_BYTE}
					else if member_type.starts_with("i32") {GL_INT}
					else if member_type.starts_with("i16") {GL_SHORT}
					else if member_type.starts_with("i8")  {GL_BYTE}
					else {
						match member_type.chars().next().unwrap() {
							'v' => GL_FLOAT,
							'd' => GL_DOUBLE,
							'b' => GL_BYTE,
							'i' => GL_INT,
							'u' => GL_UNSIGNED_INT,
							_ => panic!("Unsupported type of member: `{member_type}`"),
						}
					};
					let size = u32::from(member_type.chars().last().unwrap()) - u32::from('0');
					DataGlType{data_type, size, rows: 1}
				} else if member_type.contains("mat") {
					let data_type = if member_type.starts_with("d") {
						GL_DOUBLE
					} else {
						GL_FLOAT
					};
					let (size, rows) =
					     if member_type.ends_with("2x2") {(2, 2)}
					else if member_type.ends_with("2x3") {(2, 3)}
					else if member_type.ends_with("2x4") {(2, 4)}
					else if member_type.ends_with("3x2") {(3, 2)}
					else if member_type.ends_with("3x3") {(3, 3)}
					else if member_type.ends_with("3x4") {(3, 4)}
					else if member_type.ends_with("4x2") {(4, 2)}
					else if member_type.ends_with("4x3") {(4, 3)}
					else if member_type.ends_with("4x4") {(4, 4)}
					else {
						match member_type.chars().last().unwrap() {
							'2' => (2, 2),
							'3' => (3, 3),
							'4' => (4, 4),
							_ => panic!("Unsupported type of member: `{member_type}`"),
						}
					};
					DataGlType{data_type, size, rows}
				} else if member_type.contains("quat") {
					let data_type = if member_type.starts_with("d") {
						GL_DOUBLE
					} else {
						GL_FLOAT
					};
					DataGlType{data_type, size: 4, rows: 1}
				} else {
					panic!("Unsupported type of member: `{member_type}`")
				}
			}
		}
	}

	pub fn get_typename_of_vertex_struct_member(data: &dyn Any) -> &str {
		     if data.is::<u8>() {"u8"}
		else if data.is::<u16>() {"u16"}
		else if data.is::<u32>() {"u32"}
		else if data.is::<i8>() {"i8"}
		else if data.is::<i16>() {"i16"}
		else if data.is::<i32>() {"i32"}
		else if data.is::<f16>() {"f16"}
		else if data.is::<f32>() {"f32"}
		else if data.is::<f64>() {"f64"}
		else if data.is::<Vec1>() {"vec1"}
		else if data.is::<Vec2>() {"vec2"}
		else if data.is::<Vec3>() {"vec3"}
		else if data.is::<Vec4>() {"vec4"}
		else if data.is::<DVec1>() {"dvec1"}
		else if data.is::<DVec2>() {"dvec2"}
		else if data.is::<DVec3>() {"dvec3"}
		else if data.is::<DVec4>() {"dvec4"}
		else if data.is::<BVec1>() {"bvec1"}
		else if data.is::<BVec2>() {"bvec2"}
		else if data.is::<BVec3>() {"bvec3"}
		else if data.is::<BVec4>() {"bvec4"}
		else if data.is::<IVec1>() {"ivec1"}
		else if data.is::<IVec2>() {"ivec2"}
		else if data.is::<IVec3>() {"ivec3"}
		else if data.is::<IVec4>() {"ivec4"}
		else if data.is::<I8Vec1>() {"i8vec1"}
		else if data.is::<I8Vec2>() {"i8vec2"}
		else if data.is::<I8Vec3>() {"i8vec3"}
		else if data.is::<I8Vec4>() {"i8vec4"}
		else if data.is::<I16Vec1>() {"i16vec1"}
		else if data.is::<I16Vec2>() {"i16vec2"}
		else if data.is::<I16Vec3>() {"i16vec3"}
		else if data.is::<I16Vec4>() {"i16vec4"}
		else if data.is::<I32Vec1>() {"i32vec1"}
		else if data.is::<I32Vec2>() {"i32vec2"}
		else if data.is::<I32Vec3>() {"i32vec3"}
		else if data.is::<I32Vec4>() {"i32vec4"}
		else if data.is::<UVec1>() {"uvec1"}
		else if data.is::<UVec2>() {"uvec2"}
		else if data.is::<UVec3>() {"uvec3"}
		else if data.is::<UVec4>() {"uvec4"}
		else if data.is::<U8Vec1>() {"u8vec1"}
		else if data.is::<U8Vec2>() {"u8vec2"}
		else if data.is::<U8Vec3>() {"u8vec3"}
		else if data.is::<U8Vec4>() {"u8vec4"}
		else if data.is::<U16Vec1>() {"u16vec1"}
		else if data.is::<U16Vec2>() {"u16vec2"}
		else if data.is::<U16Vec3>() {"u16vec3"}
		else if data.is::<U16Vec4>() {"u16vec4"}
		else if data.is::<U32Vec1>() {"u32vec1"}
		else if data.is::<U32Vec2>() {"u32vec2"}
		else if data.is::<U32Vec3>() {"u32vec3"}
		else if data.is::<U32Vec4>() {"u32vec4"}
		else if data.is::<Quat>() {"quat"}
		else if data.is::<DQuat>() {"dquat"}
		else if data.is::<Mat2>() {"mat2"}
		else if data.is::<Mat3>() {"mat3"}
		else if data.is::<Mat4>() {"mat4"}
		else if data.is::<Mat2x2>() {"mat2x2"}
		else if data.is::<Mat2x3>() {"mat2x3"}
		else if data.is::<Mat2x4>() {"mat2x4"}
		else if data.is::<Mat3x2>() {"mat3x2"}
		else if data.is::<Mat3x3>() {"mat3x3"}
		else if data.is::<Mat3x4>() {"mat3x4"}
		else if data.is::<Mat4x2>() {"mat4x2"}
		else if data.is::<Mat4x3>() {"mat4x3"}
		else if data.is::<Mat4x4>() {"mat4x4"}
		else if data.is::<DMat2>() {"dmat2"}
		else if data.is::<DMat3>() {"dmat3"}
		else if data.is::<DMat4>() {"dmat4"}
		else if data.is::<DMat2x2>() {"dmat2x2"}
		else if data.is::<DMat2x3>() {"dmat2x3"}
		else if data.is::<DMat2x4>() {"dmat2x4"}
		else if data.is::<DMat3x2>() {"dmat3x2"}
		else if data.is::<DMat3x3>() {"dmat3x3"}
		else if data.is::<DMat3x4>() {"dmat3x4"}
		else if data.is::<DMat4x2>() {"dmat4x2"}
		else if data.is::<DMat4x3>() {"dmat4x3"}
		else if data.is::<DMat4x4>() {"dmat4x4"}
		else {panic!("Unsupported type of value: {data:?}")}
	}
}

impl<'a, V: VertexType, I: VertexType, M: Mesh, Mat: Material> PipelineBind<'a, V, I, M, Mat> {
	fn new(pipeline: &'a Pipeline<V, I, M, Mat>) -> Self {
		pipeline.glcore.glBindVertexArray(pipeline.name);
		Self {
			pipeline,
		}
	}

	/// Run the pipeline
	pub fn draw(&self, fbo: Option<&Framebuffer>) {
		let glcore = &self.pipeline.glcore;
		let program = self.pipeline.shader.use_program();
		let fbo_bind = fbo.map_or_else(
		|| {
			glcore.glBindFramebuffer(GL_DRAW_FRAMEBUFFER, 0);
			None
		},
		|fbo| {
			let bind = fbo.bind();
			bind.setup(&self.pipeline.shader);
			Some(bind)
		});

		let mesh = &self.pipeline.mesh;
		let vertex_buffer = mesh.get_vertex_buffer();
		let element_buffer = mesh.get_element_buffer();
		let e_bind = element_buffer.as_ref().map(|b| {
			assert_eq!(b.buffer.get_target(), BufferTarget::ElementArrayBuffer);
			b.bind()
		});

		if let Some(command_buffer) = mesh.get_command_buffer() {
			assert_eq!(command_buffer.get_target(), BufferTarget::DrawIndirectBuffer);
			let c_bind = command_buffer.bind();
			if let Some(element_buffer) = element_buffer {
				let num_commands = command_buffer.size() / size_of::<DrawElementsCommand>();
				glcore.glMultiDrawElementsIndirect(mesh.get_primitive() as u32, element_buffer.get_type() as u32, null(), num_commands as i32, size_of::<DrawElementsCommand>() as i32);
			} else {
				let num_commands = command_buffer.size() / size_of::<DrawArrayCommand>();
				glcore.glMultiDrawArraysIndirect(mesh.get_primitive() as u32, null(), num_commands as i32, size_of::<DrawArrayCommand>() as i32);
			}
			c_bind.unbind();
		} else {
			let num_vertices = vertex_buffer.size() / self.pipeline.vertex_stride;
			if let Some(instance_buffer) = mesh.get_instance_buffer() {
				let num_instances = instance_buffer.size() / self.pipeline.instance_stride;
				if let Some(element_buffer) = element_buffer {
					glcore.glDrawElementsInstanced(mesh.get_primitive() as u32, element_buffer.get_num_elements() as i32, element_buffer.get_type() as u32, null(), num_instances as i32);
				} else {
					glcore.glDrawArraysInstanced(mesh.get_primitive() as u32, 0, num_vertices as i32, num_instances as i32);
				}
			} else if let Some(element_buffer) = element_buffer {
				glcore.glDrawElements(mesh.get_primitive() as u32, element_buffer.get_num_elements() as i32, element_buffer.get_type() as u32, null());
			} else {
				glcore.glDrawArrays(mesh.get_primitive() as u32, 0, num_vertices as i32);
			}
		}

		if let Some(b) = e_bind { b.unbind() }
		program.unuse();
		if let Some(b) = fbo_bind { b.unbind() }
	}

	/// Unbind the VAO by utilizing the RAII rules.
	pub fn unbind(self) {}
}

impl<'a, V: VertexType, I: VertexType, M: Mesh, Mat: Material> Drop for PipelineBind<'a, V, I, M, Mat> {
	fn drop(&mut self) {
		self.pipeline.glcore.glBindVertexArray(0);
	}
}

impl<V: VertexType, I: VertexType, M: Mesh, Mat: Material> Drop for Pipeline<V, I, M, Mat> {
	fn drop(&mut self) {
		self.glcore.glDeleteVertexArrays(1, &self.name as *const u32);
	}
}

impl<V: VertexType, I: VertexType, M: Mesh, Mat: Material> Debug for Pipeline<V, I, M, Mat> {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		f.debug_struct("Pipeline")
		.field("name", &self.name)
		.field("mesh", &self.mesh)
		.field("shader", &self.shader)
		.finish()
	}
}

derive_vertex_type! {
	pub struct UnusedType {}
}
