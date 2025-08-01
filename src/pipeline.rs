
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

impl<M: Mesh> Pipeline<M> {
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
			let typename = Self::get_typename(field_value);
			println!("{field_name}: {typename}");
		}
	}

	fn get_typename(data: &dyn Any) -> &str {
		if data.is::<u8>() {"u8"}
		else if data.is::<u16>() {"u16"}
		else if data.is::<u32>() {"u32"}
		else if data.is::<u64>() {"u64"}
		else if data.is::<i8>() {"i8"}
		else if data.is::<i16>() {"i16"}
		else if data.is::<i32>() {"i32"}
		else if data.is::<i64>() {"i64"}
		else if data.is::<f32>() {"f32"}
		else if data.is::<f64>() {"f64"}
		else if data.is::<bool>() {"bool"}
		else if data.is::<Vec1>() {"Vec1"}
		else if data.is::<Vec2>() {"Vec2"}
		else if data.is::<Vec3>() {"Vec3"}
		else if data.is::<Vec4>() {"Vec4"}
		else if data.is::<DVec1>() {"DVec1"}
		else if data.is::<DVec2>() {"DVec2"}
		else if data.is::<DVec3>() {"DVec3"}
		else if data.is::<DVec4>() {"DVec4"}
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
		else if data.is::<I64Vec1>() {"I64Vec1"}
		else if data.is::<I64Vec2>() {"I64Vec2"}
		else if data.is::<I64Vec3>() {"I64Vec3"}
		else if data.is::<I64Vec4>() {"I64Vec4"}
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
		else if data.is::<U64Vec1>() {"U64Vec1"}
		else if data.is::<U64Vec2>() {"U64Vec2"}
		else if data.is::<U64Vec3>() {"U64Vec3"}
		else if data.is::<U64Vec4>() {"U64Vec4"}
		else if data.is::<BVec1>() {"BVec1"}
		else if data.is::<BVec2>() {"BVec2"}
		else if data.is::<BVec3>() {"BVec3"}
		else if data.is::<BVec4>() {"BVec4"}
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
		else {panic!("Unknown type of value: {:?}", data)}
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
