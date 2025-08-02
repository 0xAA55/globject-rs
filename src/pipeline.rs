
#![allow(dead_code)]

use glcore::*;
use crate::glshader::*;
use crate::glframebuffer::*;
use crate::mesh::*;
use struct_iterable::Iterable;
use std::{
	any::Any,
	fmt::{self, Debug, Formatter},
	rc::Rc,
};
use glm::*;

pub trait VertexType: Copy + Clone + Sized + Default + Debug + Iterable {}
impl<T> VertexType for T where T: Copy + Clone + Sized + Default + Debug + Iterable {}

pub struct Pipeline<M: Mesh> {
	pub glcore: Rc<GLCore>,
	name: u32,
	pub mesh: Rc<M>,
	pub framebuffer: Option<Rc<Framebuffer>>,
	pub shader: Rc<Shader>,
}

#[derive(Debug, Clone, Copy)]
struct DataGlType {
	data_type: u32,
	size: u32,
	rows: u32,
}

#[derive(Debug)]
pub struct PipelineBind<'a, M: Mesh> {
	pub pipeline: &'a Pipeline<M>,
}

impl DataGlType {
	fn is_integer(&self) -> bool {
		match self.data_type {
			GL_BYTE | GL_SHORT | GL_INT | GL_UNSIGNED_BYTE | GL_UNSIGNED_SHORT | GL_UNSIGNED_INT => true,
			_ => false,
		}
	}

	fn is_double(&self) -> bool {
		match self.data_type {
			GL_DOUBLE => true,
			_ => false,
		}
	}

	fn size_in_bytes(&self) -> usize {
		match self.data_type {
			GL_BYTE | GL_UNSIGNED_BYTE => 1usize * self.size as usize * self.rows as usize,
			GL_SHORT | GL_UNSIGNED_SHORT | GL_HALF_FLOAT => 2usize * self.size as usize * self.rows as usize,
			GL_INT | GL_UNSIGNED_INT | GL_FLOAT => 4usize * self.size as usize * self.rows as usize,
			GL_DOUBLE => 8usize * self.size as usize * self.rows as usize,
			other => panic!("Invalid `data_type` ({other})"),
		}
	}
}

impl<M: Mesh> Pipeline<M> {
	/// Get the internal name
	pub fn get_name(&self) -> u32 {
		self.name
	}

	pub fn new<T: VertexType>(glcore: Rc<GLCore>, mesh: Rc<M>, framebuffer: Option<Rc<Framebuffer>>, shader: Rc<Shader>) -> Self {
		let mut name: u32 = 0;
		glcore.glGenVertexArrays(1, &mut name as *mut u32);
		let mut ret = Self {
			glcore,
			name,
			mesh,
			framebuffer,
			shader,
		};
		ret.establish_pipeline::<T>();
		ret
	}

	fn establish_pipeline<T: VertexType>(&mut self) {
		let instance = T::default();
		for (field_name, field_value) in instance.iter() {
			let typename = Self::get_typename_of_vertex_struct_member(field_value);
			println!("{field_name}: {typename}");
		}
		let active_attribs = self.shader.get_active_attribs().unwrap();
		for (attrib_name, attrib_type) in active_attribs.iter() {
			println!("{attrib_name}: {} {}", attrib_type.type_, attrib_type.size);

	pub fn bind<'a>(&'a self) -> PipelineBind<'a, M> {
		PipelineBind::new(self)
	}

	fn get_vertex_struct_member_gltype(member_type: &str) -> DataGlType {
		match member_type {
			"i8" => return DataGlType{data_type: GL_BYTE, size: 1, rows: 1},
			"i16" => return DataGlType{data_type: GL_SHORT, size: 1, rows: 1},
			"i32" => return DataGlType{data_type: GL_INT, size: 1, rows: 1},
			"u8" => return DataGlType{data_type: GL_UNSIGNED_BYTE, size: 1, rows: 1},
			"u16" => return DataGlType{data_type: GL_UNSIGNED_SHORT, size: 1, rows: 1},
			"u32" => return DataGlType{data_type: GL_UNSIGNED_INT, size: 1, rows: 1},
			"f16" => return DataGlType{data_type: GL_HALF_FLOAT, size: 1, rows: 1},
			"f32" => return DataGlType{data_type: GL_FLOAT, size: 1, rows: 1},
			"f64" => return DataGlType{data_type: GL_DOUBLE, size: 1, rows: 1},
			_ => {
				if member_type.contains("Vec") {
					let data_type =
					     if member_type.starts_with("U32") {GL_UNSIGNED_INT}
					else if member_type.starts_with("U16") {GL_UNSIGNED_SHORT}
					else if member_type.starts_with("U8")  {GL_UNSIGNED_BYTE}
					else if member_type.starts_with("I32") {GL_INT}
					else if member_type.starts_with("I16") {GL_SHORT}
					else if member_type.starts_with("I8")  {GL_BYTE}
					else {
						match member_type.chars().next().unwrap() {
							'V' => GL_FLOAT,
							'D' => GL_DOUBLE,
							'B' => GL_BYTE,
							'I' => GL_INT,
							'U' => GL_UNSIGNED_INT,
							_ => panic!("Unsupported type of member: `{member_type}`"),
						}
					};
					let size = u32::from(member_type.chars().last().unwrap());
					DataGlType{data_type, size, rows: 1}
				} else if member_type.contains("Mat") {
					let data_type = if member_type.starts_with("D") {
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
				} else if member_type.contains("Quat") {
					let data_type = if member_type.starts_with("D") {
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
		else if data.is::<Vec1>() {"Vec1"}
		else if data.is::<Vec2>() {"Vec2"}
		else if data.is::<Vec3>() {"Vec3"}
		else if data.is::<Vec4>() {"Vec4"}
		else if data.is::<DVec1>() {"DVec1"}
		else if data.is::<DVec2>() {"DVec2"}
		else if data.is::<DVec3>() {"DVec3"}
		else if data.is::<DVec4>() {"DVec4"}
		else if data.is::<BVec1>() {"BVec1"}
		else if data.is::<BVec2>() {"BVec2"}
		else if data.is::<BVec3>() {"BVec3"}
		else if data.is::<BVec4>() {"BVec4"}
		else if data.is::<IVec1>() {"IVec1"}
		else if data.is::<IVec2>() {"IVec2"}
		else if data.is::<IVec3>() {"IVec3"}
		else if data.is::<IVec4>() {"IVec4"}
		else if data.is::<I8Vec1>() {"I8Vec1"}
		else if data.is::<I8Vec2>() {"I8Vec2"}
		else if data.is::<I8Vec3>() {"I8Vec3"}
		else if data.is::<I8Vec4>() {"I8Vec4"}
		else if data.is::<I16Vec1>() {"I16Vec1"}
		else if data.is::<I16Vec2>() {"I16Vec2"}
		else if data.is::<I16Vec3>() {"I16Vec3"}
		else if data.is::<I16Vec4>() {"I16Vec4"}
		else if data.is::<I32Vec1>() {"I32Vec1"}
		else if data.is::<I32Vec2>() {"I32Vec2"}
		else if data.is::<I32Vec3>() {"I32Vec3"}
		else if data.is::<I32Vec4>() {"I32Vec4"}
		else if data.is::<UVec1>() {"UVec1"}
		else if data.is::<UVec2>() {"UVec2"}
		else if data.is::<UVec3>() {"UVec3"}
		else if data.is::<UVec4>() {"UVec4"}
		else if data.is::<U8Vec1>() {"U8Vec1"}
		else if data.is::<U8Vec2>() {"U8Vec2"}
		else if data.is::<U8Vec3>() {"U8Vec3"}
		else if data.is::<U8Vec4>() {"U8Vec4"}
		else if data.is::<U16Vec1>() {"U16Vec1"}
		else if data.is::<U16Vec2>() {"U16Vec2"}
		else if data.is::<U16Vec3>() {"U16Vec3"}
		else if data.is::<U16Vec4>() {"U16Vec4"}
		else if data.is::<U32Vec1>() {"U32Vec1"}
		else if data.is::<U32Vec2>() {"U32Vec2"}
		else if data.is::<U32Vec3>() {"U32Vec3"}
		else if data.is::<U32Vec4>() {"U32Vec4"}
		else if data.is::<Quat>() {"Quat"}
		else if data.is::<DQuat>() {"DQuat"}
		else if data.is::<Mat2>() {"Mat2"}
		else if data.is::<Mat3>() {"Mat3"}
		else if data.is::<Mat4>() {"Mat4"}
		else if data.is::<Mat2x2>() {"Mat2x2"}
		else if data.is::<Mat2x3>() {"Mat2x3"}
		else if data.is::<Mat2x4>() {"Mat2x4"}
		else if data.is::<Mat3x2>() {"Mat3x2"}
		else if data.is::<Mat3x3>() {"Mat3x3"}
		else if data.is::<Mat3x4>() {"Mat3x4"}
		else if data.is::<Mat4x2>() {"Mat4x2"}
		else if data.is::<Mat4x3>() {"Mat4x3"}
		else if data.is::<Mat4x4>() {"Mat4x4"}
		else if data.is::<DMat2>() {"DMat2"}
		else if data.is::<DMat3>() {"DMat3"}
		else if data.is::<DMat4>() {"DMat4"}
		else if data.is::<DMat2x2>() {"DMat2x2"}
		else if data.is::<DMat2x3>() {"DMat2x3"}
		else if data.is::<DMat2x4>() {"DMat2x4"}
		else if data.is::<DMat3x2>() {"DMat3x2"}
		else if data.is::<DMat3x3>() {"DMat3x3"}
		else if data.is::<DMat3x4>() {"DMat3x4"}
		else if data.is::<DMat4x2>() {"DMat4x2"}
		else if data.is::<DMat4x3>() {"DMat4x3"}
		else if data.is::<DMat4x4>() {"DMat4x4"}
		else {panic!("Unsupported type of value: {:?}", data)}
	}
}

impl<'a, M: Mesh> PipelineBind<'a, M> {
	fn new(pipeline: &'a Pipeline<M>) -> Self {
		pipeline.glcore.glBindVertexArray(pipeline.name);
		Self {
			pipeline,
		}
	}

	/// Unbind the VAO by utilizing the RAII rules.
	pub fn unbind(self) {}
}

impl<'a, M: Mesh> Drop for PipelineBind<'a, M> {
	fn drop(&mut self) {
		self.pipeline.glcore.glBindVertexArray(0);
	}
}

impl<M: Mesh> Drop for Pipeline<M> {
	fn drop(&mut self) {
		self.glcore.glDeleteVertexArrays(1, &self.name as *const u32);
	}
}

impl<M: Mesh> Debug for Pipeline<M> {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		f.debug_struct("Pipeline")
		.field("name", &self.name)
		.field("mesh", &self.mesh)
		.field("framebuffer", &self.framebuffer)
		.field("shader", &self.shader)
		.finish()
	}
}


