
use crate::prelude::*;
use std::{
	cmp::max,
	collections::BTreeMap,
	fmt::{self, Debug, Formatter},
	rc::Rc,
};

/// The framebuffer render target type
pub struct FramebufferTarget {
	/// The texture binding target
	pub texture_target: TextureTarget,

	/// The layer index of the 3D texture to bind (Only bind a 2D layer to the framebuffer)
	pub layer_of_3d: i32,
}

/// The framebuffer object type
pub struct Framebuffer {
	pub glcore: Rc<GLCore>,
	name: u32,

	/// The name of the draw targets and the binding target and the texture
	pub draw_targets: BTreeMap<String, (FramebufferTarget, Rc<dyn GenericTexture>)>,
}

/// The error of the framebuffers
#[derive(Debug, Clone)]
pub enum FramebufferError {
	NoDefaultFramebuffer,
	IncompleteAttachment,
	IncompleteMissingAttachment,
	IncompleteDrawBuffer,
	IncompleteReadBuffer,
	Unsupported,
	IncompleteMultisample,
	IncompleteLayerTarget,
	UnknownError(GLenum),
	GLCoreError(GLCoreError),
}

impl From<GLCoreError> for FramebufferError {
	fn from(val: GLCoreError) -> Self {
		Self::GLCoreError(val)
	}
}

/// The binding guard of the framebuffer
pub struct FramebufferBind<'a> {
	framebuffer: &'a Framebuffer,
}

impl Framebuffer {
	/// Create a new empty framebuffer object
	pub fn new(glcore: Rc<GLCore>) -> Result<Self, FramebufferError> {
		let mut name: u32 = 0;
		glcore.glGenFramebuffers(1, &mut name as *mut _)?;
		Ok(Self {
			glcore,
			name,
			draw_targets: BTreeMap::new(),
		})
	}

	/// Utilize the RAII rules to manage binding states.
	pub fn bind<'a>(&'a self) -> Result<FramebufferBind<'a>, FramebufferError> {
		FramebufferBind::new(self)
	}

	/// Bind to the default framebuffer
	pub fn default_bind(glcore: &GLCore) -> Result<(), FramebufferError> {
		glcore.glBindFramebuffer(GL_DRAW_FRAMEBUFFER, 0)?;
		Ok(())
	}
}

impl<'a> FramebufferBind<'a> {
	/// Create a new binding state to the framebuffer object, utilizing the RAII rules to manage the binding state.
	fn new(framebuffer: &'a Framebuffer) -> Result<Self, FramebufferError> {
		framebuffer.glcore.glBindFramebuffer(GL_DRAW_FRAMEBUFFER, framebuffer.name)?;
		Ok(Self {
			framebuffer,
		})
	}

	/// Set up the framebuffer, apply `draw_targets`
	pub fn setup(&self, program: &Shader) -> Result<(), FramebufferError> {
		let draw_targets = &self.framebuffer.draw_targets;
		assert!(!draw_targets.is_empty());
		let glcore = self.framebuffer.glcore.clone();
		let mut draw_buffers: Vec<u32> = Vec::with_capacity(draw_targets.len());
		let mut max_width: u32 = 0;
		let mut max_height: u32 = 0;
		for (target_name, target) in draw_targets.iter() {
			let location = glcore.glGetFragDataLocation(program.get_name(), target_name.as_ptr() as *const i8)?;
			if location >= 0 {
				let location = location as u32;
				let (target, texture) = target;
				let attachment = GL_COLOR_ATTACHMENT0 + location;
				max_width = max(max_width, texture.get_width());
				max_height = max(max_height, texture.get_height());
				match texture.get_dim() {
					TextureDimension::Tex1d =>		glcore.glFramebufferTexture1D(GL_DRAW_FRAMEBUFFER, attachment, target.texture_target as u32, texture.get_name(), 0)?,
					TextureDimension::Tex2d =>		glcore.glFramebufferTexture2D(GL_DRAW_FRAMEBUFFER, attachment, target.texture_target as u32, texture.get_name(), 0)?,
					TextureDimension::Tex3d =>		glcore.glFramebufferTexture3D(GL_DRAW_FRAMEBUFFER, attachment, target.texture_target as u32, texture.get_name(), 0, target.layer_of_3d)?,
					TextureDimension::TexCube =>	glcore.glFramebufferTexture2D(GL_DRAW_FRAMEBUFFER, attachment, target.texture_target as u32, texture.get_name(), 0)?,
				}
				draw_buffers.push(attachment);
			} else {
				eprintln!("Location of shader output `{target_name}` couldn't be found.");
			}
		}
		glcore.glDrawBuffers(draw_buffers.len() as i32, draw_buffers.as_ptr())?;
		match glcore.glCheckFramebufferStatus(GL_DRAW_FRAMEBUFFER) ?{
			GL_FRAMEBUFFER_COMPLETE => {},
			GL_FRAMEBUFFER_UNDEFINED => return Err(FramebufferError::NoDefaultFramebuffer),
			GL_FRAMEBUFFER_INCOMPLETE_ATTACHMENT => return Err(FramebufferError::IncompleteAttachment),
			GL_FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT => return Err(FramebufferError::IncompleteMissingAttachment),
			GL_FRAMEBUFFER_INCOMPLETE_DRAW_BUFFER => return Err(FramebufferError::IncompleteDrawBuffer),
			GL_FRAMEBUFFER_INCOMPLETE_READ_BUFFER => return Err(FramebufferError::IncompleteReadBuffer),
			GL_FRAMEBUFFER_UNSUPPORTED => return Err(FramebufferError::Unsupported),
			GL_FRAMEBUFFER_INCOMPLETE_MULTISAMPLE => return Err(FramebufferError::IncompleteMultisample),
			GL_FRAMEBUFFER_INCOMPLETE_LAYER_TARGETS => return Err(FramebufferError::IncompleteLayerTarget),
			other => return Err(FramebufferError::UnknownError(other)),
		}
		glcore.glViewport(0, 0, max_width as i32, max_height as i32)?;
		Ok(())
	}

	/// Explicitly unbind the framebuffer
	pub fn unbind(self) {}
}

impl Drop for FramebufferBind<'_> {
	fn drop(&mut self) {
		self.framebuffer.glcore.glBindFramebuffer(GL_DRAW_FRAMEBUFFER, 0).unwrap();
	}
}

impl Drop for Framebuffer {
	fn drop(&mut self) {
		self.glcore.glDeleteFramebuffers(1, &self.name as *const _).unwrap();
	}
}

impl Debug for Framebuffer {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		f.debug_struct("Framebuffer")
		.field("name", &self.name)
		.finish()
	}
}
