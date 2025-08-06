
#![allow(dead_code)]

/// The most basic OpenGL Buffer Object wrapping
pub mod glbuffer;

/// The most basic OpenGL Shader Program Object wrapping
pub mod glshader;

/// Definitions of `DrawArrayCommand`, `DrawElementsCommand` and `DispatchIndirectCommand`
pub mod glcmdbuf;

/// The most basic OpenGL Texture Object wrapping
pub mod gltexture;

/// The most basic OpenGL Framebuffer Object wrapping
pub mod glframebuffer;

/// An upper layer wrapping for `Buffer`, the `BufferVec` allows editing the buffer items easier than just to use the `Buffer`
pub mod buffervec;

/// An upper layer wrapping for `Mesh`, utilizing the trait `BufferVec` as the `Mesh` generic type of the vertex buffer, element buffer, instance buffer, and command buffer
pub mod mesh;

/// The material module provides `MaterialLegacy`, `MaterialPbr`, and the trait `Material`
pub mod material;

/// The most basic OpenGL Vertex Array Object that manages the pipeline from the data source in the array buffer to the shader attrib inputs
pub mod pipeline;

/// The mesh set for the complex mesh, each mesh subset has its name and material.
pub mod meshset;

/// The common module is to provide some miscellous utilities
pub mod common;

extern crate nalgebra_glm as glm;

/// The prelude module provides all of the things you need to use
pub mod prelude {
	pub use crate::glbuffer::*;
	pub use crate::glshader::*;
	pub use crate::glcmdbuf::*;
	pub use crate::gltexture::*;
	pub use crate::glframebuffer::*;
	pub use crate::buffervec::*;
	pub use crate::mesh::*;
	pub use crate::material::*;
	pub use crate::pipeline::*;
	pub use crate::meshset::*;
	pub use crate::common::*;
	pub use crate::derive_vertex_type;
	pub use glm::*;
	pub use struct_iterable::Iterable;
	pub use glcore::*;
	pub use half::f16;
}

pub use prelude::*;

#[cfg(test)]
mod tests {
	use std::{
		ffi::c_void,
		mem::size_of_val,
		process::ExitCode,
		rc::Rc,
	};
	use super::prelude::*;
	use glfw::*;

	derive_vertex_type! {
		pub struct MyVertex {
			position: Vec2,
		}
	}

	#[derive(Debug)]
	enum AppError {
		GLFWInitErr,
		GLFWCreateWindowFailed,
		GLFWErr(glfw::Error),
	}

	#[derive(Debug)]
	struct Renderer {
		pipeline: Rc<Pipeline<MyVertex, UnusedType>>,
		shader: Rc<Shader>,
		mesh: Rc<dyn GenericMeshWithMaterial>,
	}

	#[derive(Debug)]
	struct AppInstance {
		renderer: Option<Renderer>,
		glcore: Rc<GLCore>,
		events: GlfwReceiver<(f64, WindowEvent)>,
		window: PWindow,
		glfw: Glfw,
	}

	impl Renderer {
		fn new(glcore: Rc<GLCore>) -> Self {
			let vertices = [
				MyVertex{position: Vec2::new(-1.0, -1.0)},
				MyVertex{position: Vec2::new( 1.0, -1.0)},
				MyVertex{position: Vec2::new(-1.0,  1.0)},
				MyVertex{position: Vec2::new( 1.0,  1.0)},
			];
			let elements = [
				0u8, 1u8, 2u8,
				1u8, 3u8, 2u8,
			];
			let vertex_buffer = Buffer::new(glcore.clone(), BufferTarget::ArrayBuffer, size_of_val(&vertices), BufferUsage::StaticDraw, vertices.as_ptr() as *const c_void);
			let mut vertex_buffer = BufferVecStatic::<MyVertex>::new(glcore.clone(), vertex_buffer);
			vertex_buffer.resize(4, MyVertex::default());
			let element_buffer = Buffer::new(glcore.clone(), BufferTarget::ElementArrayBuffer, size_of_val(&elements), BufferUsage::StaticDraw, elements.as_ptr() as *const c_void);
			let mut element_buffer = BufferVecStatic::<u8>::new(glcore.clone(), element_buffer);
			element_buffer.resize(6, 0u8);
			let mesh = StaticMesh::<MyVertex, u8, UnusedType, UnusedType>::new(PrimitiveMode::Triangles, vertex_buffer, Some(element_buffer), None, None);
			let mesh = Rc::new(MeshWithMaterial::new(mesh, Rc::new(MaterialLegacy::default())));
			let mesh: Rc<dyn GenericMeshWithMaterial> = mesh;
			let shader = Rc::new(Shader::new(glcore.clone(),
				Some("
#version 330\n

in vec2 position;

void main()
{
	gl_Position = vec4(position, 0.0, 1.0);
}
				"),
				None,
				Some("
#version 330\n

out vec4 Color;

void main()
{
	Color = vec4(0.0, 0.0, 0.5, 1.0);
}
				")
			).unwrap());
			let pipeline = Rc::new(Pipeline::new(glcore.clone(), mesh.clone(), shader.clone()));
			Self {
				mesh,
				shader,
				pipeline,
			}
		}

		fn render(&self, glcore: &GLCore) {
			glcore.glClearColor(0.0, 0.3, 0.5, 1.0);
			glcore.glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);

			let p_bind = self.pipeline.bind();
			p_bind.draw(None);
			p_bind.unbind();
		}
	}

	impl AppInstance {
		pub fn new() -> Result<Self, AppError> {
			let mut glfw = match glfw::init_no_callbacks() {
				Ok(glfw) => glfw,
				Err(_) => return Err(AppError::GLFWInitErr), // Due to doc, won't return `glfw::InitError::AlreadyInitialized`
			};
			let (mut window, events) = glfw.create_window(1024, 768, "GLFW Window", glfw::WindowMode::Windowed).ok_or(AppError::GLFWCreateWindowFailed)?;
			window.set_key_polling(true);
			window.make_current();
			glfw.set_swap_interval(SwapInterval::Adaptive);
			let glcore = Rc::new(GLCore::new(|proc_name|window.get_proc_address(proc_name)));
			let renderer = Some(Renderer::new(glcore.clone()));
			Ok(Self {
				renderer,
				glcore,
				events,
				window,
				glfw,
			})
		}

		pub fn run(&mut self, timeout: Option<f64>) -> ExitCode {
			let start_debug_time = self.glfw.get_time();
			while !self.window.should_close() {
				let time_cur_frame = self.glfw.get_time();

				if let Some(renderer) = self.renderer.as_ref() {
					renderer.render(&self.glcore);
					self.window.swap_buffers();
				}

				self.glfw.poll_events();
				for (_, event) in glfw::flush_messages(&self.events) {
					match event {
						glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
							self.window.set_should_close(true)
						}
						_ => {}
					}
				}

				if let Some(timeout) = timeout {
					if time_cur_frame - start_debug_time >= timeout {
						self.window.set_should_close(true)
					}
				}
			}

			ExitCode::from(0)
		}
	}

	#[test]
	fn test_glfw() -> ExitCode {
		const DEBUG_TIME: f64 = 10.0;
		let mut test_app = match AppInstance::new() {
			Ok(app) => app,
			Err(e) => {
				eprintln!("GLFW App Initialize failed: {:?}", e);
				return ExitCode::from(2)
			}
		};
		test_app.run(Some(DEBUG_TIME))
	}
}
