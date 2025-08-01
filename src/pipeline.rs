
#![allow(dead_code)]

use glcore::*;
use crate::glshader::*;
use crate::glframebuffer::*;
use crate::mesh::*;
use struct_iterable::Iterable;
use std::{
	any::type_name_of_val,
	fmt::{self, Debug, Formatter},
	rc::Rc,
};

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
			let typename = type_name_of_val(field_value);
			println!("{field_name}: {typename}");
		}
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
