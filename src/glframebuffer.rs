
#![allow(dead_code)]

use glcore::*;
use crate::glshader::*;
use crate::gltexture::*;
use std::{
	cmp::max,
	collections::BTreeMap,
	fmt::{self, Debug, Formatter},
	rc::Rc,
};

pub struct FramebufferTarget {
	pub texture_target: TextureTarget,
	pub layer_of_3d: i32,
}

pub struct Framebuffer {
	pub glcore: Rc<GLCore>,
	name: u32,
	pub draw_targets: BTreeMap<String, (FramebufferTarget, Rc<Texture>)>,
}

pub struct FramebufferBind<'a> {
	framebuffer: &'a Framebuffer,
}

impl Framebuffer {
	pub fn new(glcore: Rc<GLCore>) -> Self {
		let mut name: u32 = 0;
		glcore.glGenFramebuffers(1, &mut name as *mut _);
		Self {
			glcore,
			name,
			draw_targets: BTreeMap::new(),
		}
	}

	pub fn bind<'a>(&'a self) -> FramebufferBind<'a> {
		FramebufferBind::new(self)
	}
}

impl<'a> FramebufferBind<'a> {
	fn new(framebuffer: &'a Framebuffer) -> Self {
		framebuffer.glcore.glBindFramebuffer(GL_DRAW_FRAMEBUFFER, framebuffer.name);
		Self {
			framebuffer,
		}
	}

	/// Setup the framebuffer, apply `draw_targets`
	pub fn setup(&self, program: &Shader) {
		let draw_targets = &self.framebuffer.draw_targets;
		assert!(draw_targets.len() > 0);
		let glcore = self.framebuffer.glcore.clone();
		let mut draw_buffers: Vec<u32> = Vec::with_capacity(draw_targets.len());
		let mut max_width: u32 = 0;
		let mut max_height: u32 = 0;
		for (target_name, target) in draw_targets.iter() {
			let location = glcore.glGetFragDataLocation(program.get_name(), target_name.as_ptr() as *const i8);
			if location >= 0 {
				let location = location as u32;
				let (target, texture) = target;
				let attachment = GL_COLOR_ATTACHMENT0 + location;
				max_width = max(max_width, texture.get_width());
				max_height = max(max_height, texture.get_height());
				match texture.get_dim() {
					TextureDimension::Tex1d => {
						glcore.glFramebufferTexture1D(GL_DRAW_FRAMEBUFFER, attachment, target.texture_target as u32, texture.get_name(), 0);
					}
					TextureDimension::Tex2d => {
						glcore.glFramebufferTexture2D(GL_DRAW_FRAMEBUFFER, attachment, target.texture_target as u32, texture.get_name(), 0);
					}
					TextureDimension::Tex3d => {
						glcore.glFramebufferTexture3D(GL_DRAW_FRAMEBUFFER, attachment, target.texture_target as u32, texture.get_name(), 0, target.layer_of_3d);
					}
					TextureDimension::TexCube => {
						glcore.glFramebufferTexture2D(GL_DRAW_FRAMEBUFFER, attachment, target.texture_target as u32, texture.get_name(), 0);
					}
				}
				draw_buffers.push(attachment);
			} else {
				eprintln!("Location of shader output `{target_name}` couldn't be found.");
			}
		}
		glcore.glDrawBuffers(draw_buffers.len() as i32, draw_buffers.as_ptr());
		glcore.glViewport(0, 0, max_width as i32, max_height as i32);
	}

	/// Unbind the framebuffer by utilizing the RAII rules.
	pub fn unbind(self) {}
}

impl Drop for FramebufferBind<'_> {
	fn drop(&mut self) {
		self.framebuffer.glcore.glBindFramebuffer(GL_DRAW_FRAMEBUFFER, 0);
	}
}

impl Drop for Framebuffer {
	fn drop(&mut self) {
		self.glcore.glDeleteFramebuffers(1, &self.name as *const _);
	}
}

impl Debug for Framebuffer {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		f.debug_struct("Framebuffer")
		.field("name", &self.name)
		.finish()
	}
}
