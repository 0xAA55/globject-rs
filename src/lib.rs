
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
		GLCoreError(GLCoreError),
		ShaderError(ShaderError),
		PipelineError(PipelineError),
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

	impl From<GLCoreError> for AppError {
		fn from(val: GLCoreError) -> Self {
			Self::GLCoreError(val)
		}
	}

	impl From<ShaderError> for AppError {
		fn from(val: ShaderError) -> Self {
			Self::ShaderError(val)
		}
	}

	impl From<PipelineError> for AppError {
		fn from(val: PipelineError) -> Self {
			Self::PipelineError(val)
		}
	}

	impl Renderer {
		fn new(glcore: Rc<GLCore>) -> Result<Self, AppError> {
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
			let vertex_buffer = Buffer::new(glcore.clone(), BufferTarget::ArrayBuffer, size_of_val(&vertices), BufferUsage::StaticDraw, vertices.as_ptr() as *const c_void)?;
			let mut vertex_buffer = BufferVecStatic::<MyVertex>::new(vertex_buffer);
			vertex_buffer.resize(4, MyVertex::default())?;
			let element_buffer = Buffer::new(glcore.clone(), BufferTarget::ElementArrayBuffer, size_of_val(&elements), BufferUsage::StaticDraw, elements.as_ptr() as *const c_void)?;
			let mut element_buffer = BufferVecStatic::<u8>::new(element_buffer);
			element_buffer.resize(6, 0u8)?;
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
				Some( // **NOTE** The fragment shader below comes from "https://www.shadertoy.com/view/MsjSzz", The author is TDM.
"#version 330\n

#define LINEAR_ROTATION\n

#define WEIGHT (3.0 / iResolution.x)\n
const vec3 RED = vec3(1.0,0.0,0.0);
const vec3 GREEN = vec3(0.0,1.0,0.0);
const vec3 BLUE = vec3(0.0,0.8,1.0);
const vec3 WHITE = vec3(1.0,1.0,0.97);
const vec3 YELLOW = vec3(1.0,1.0,0.0);

uniform vec3 iResolution;
uniform float iTime;

vec2 fragCoord = gl_FragCoord.xy;
out vec4 Color;

/* rasterize functions */
float line(vec2 p, vec2 p0, vec2 p1, float w) {
	vec2 d = p1 - p0;
	float t = clamp(dot(d,p-p0) / dot(d,d), 0.0,1.0);
	vec2 proj = p0 + d * t;
	float dist = length(p - proj);
	dist = 1.0/dist*WEIGHT*w;
	return min(dist*dist,1.0);
}
float circle(vec2 p, vec2 c, float r, float w) {
	float dist = abs(length(p - c)) + r;
	dist = 1.0/dist*WEIGHT*w;
	return min(dist*dist,1.0);
}

/* matrices */
mat4 getRotMatrix(vec3 a) {
	vec3 s = sin(a);
	vec3 c = cos(a);
	mat4 ret;
	ret[0] = vec4(c.y*c.z,c.y*s.z,-s.y,0.0);
	ret[1] = vec4(s.x*s.y*c.z-c.x*s.z,s.x*s.y*s.z+c.x*c.z,s.x*c.y,0.0);
	ret[2] = vec4(c.x*s.y*c.z+s.x*s.z,c.x*s.y*s.z-s.x*c.z,c.x*c.y,0.0);
	ret[3] = vec4(0.0,0.0,0.0,1.0);
	return ret;
}
mat4 getPosMatrix(vec3 p) {
	mat4 ret;
	ret[0] = vec4(1.0,0.0,0.0,p.x);
	ret[1] = vec4(0.0,1.0,0.0,p.y);
	ret[2] = vec4(0.0,0.0,1.0,p.z);
	ret[3] = vec4(0.0,0.0,0.0,1.0);
	return ret;
}

/* utils */
vec3 mix3(vec3 a, vec3 b, vec3 c, float t) {
	if(t>0.5) return mix(b,c,t*2.0-1.0);
	else return mix(a,b,t*2.0);
}
vec3 fragment(vec3 p) {
	float t = sin(p.x*0.8+iTime*0.5)*0.5+0.5;
	float fog = min(pow(p.z,3.0)*400.0,1.0);
	return mix3(RED,GREEN,BLUE,t) * fog;
}

void main() {
	vec2 uv = fragCoord.xy / iResolution.xy;
	uv = uv * 2.0 - 1.0;
	uv.x *= iResolution.x / iResolution.y;
	/* uv = uv * (1.0 + pow(length(uv)*0.4,0.5)) * 0.6; */

	float line_width = 0.4;
	float time = iTime * 0.31415;
	vec3 c = vec3(mix(vec3(0.19,0.13,0.1),vec3(1.0), 0.5*pow(length(uv)*0.5,2.0)));
	mat4 cam = getPosMatrix(vec3(0.0,0.0,10.0));

#ifdef LINEAR_ROTATION
	mat4 rot = getRotMatrix(vec3(time,time*0.86,time*0.473));
#else
	float p = 0.08;
	mat4 rot = getRotMatrix(vec3(time		+sin(time*30.0)*p,
								 time*0.860	+sin(time*20.0)*p*1.24,
								 time*0.473	+sin(time*10.0)*p));
#endif

	vec3 instances[18];
	instances[0] = vec3( 0.0, 0.0,-1.0);
	instances[1] = vec3(-1.0, 0.0,-1.0);
	instances[2] = vec3( 1.0, 0.0,-1.0);
	instances[3] = vec3( 0.0, 1.0,-1.0);
	instances[4] = vec3( 0.0,-1.0,-1.0);
	instances[5] = vec3(-1.0, 0.0, 0.0);
	instances[6] = vec3( 1.0, 0.0, 0.0);
	instances[7] = vec3( 0.0, 1.0, 0.0);
	instances[8] = vec3( 0.0,-1.0, 0.0);
	instances[9] = vec3(-1.0,-1.0, 0.0);
	instances[10] = vec3( 1.0, 1.0, 0.0);
	instances[11] = vec3(-1.0, 1.0, 0.0);
	instances[12] = vec3( 1.0,-1.0, 0.0);
	instances[13] = vec3( 0.0, 0.0, 1.0);
	instances[14] = vec3(-1.0, 0.0, 1.0);
	instances[15] = vec3( 1.0, 0.0, 1.0);
	instances[16] = vec3( 0.0, 1.0, 1.0);
	instances[17] = vec3( 0.0,-1.0, 1.0);

	/* box pipeline */
	for(int dip = 0; dip < 18; dip++) {

		/* input assembly */
		vec3 vert[8];
		vert[0] = vec3(-1.0,-1.0, 1.0);
		vert[1] = vec3(-1.0, 1.0, 1.0);
		vert[2] = vec3( 1.0, 1.0, 1.0);
		vert[3] = vec3( 1.0,-1.0, 1.0);
		vert[4] = vec3(-1.0,-1.0,-1.0);
		vert[5] = vec3(-1.0, 1.0,-1.0);
		vert[6] = vec3( 1.0, 1.0,-1.0);
		vert[7] = vec3( 1.0,-1.0,-1.0);

		/* vertex processing */
		mat4 pos = getPosMatrix(instances[dip] * 4.0);
		mat4 mat = pos * rot * cam;

		for(int i = 0; i < 8; i++) {

			/* transform */
			vert[i] = (vec4(vert[i],1.0) * mat).xyz;

			/* perspective */
			vert[i].z = 1.0 / vert[i].z;
			vert[i].xy *= vert[i].z;
		}

		/* primitive assembly and rasterize */
		float i;
		i  = line(uv,vert[0].xy,vert[1].xy,line_width);
		i += line(uv,vert[1].xy,vert[2].xy,line_width);
		i += line(uv,vert[2].xy,vert[3].xy,line_width);
		i += line(uv,vert[3].xy,vert[0].xy,line_width);
		i += line(uv,vert[4].xy,vert[5].xy,line_width);
		i += line(uv,vert[5].xy,vert[6].xy,line_width);
		i += line(uv,vert[6].xy,vert[7].xy,line_width);
		i += line(uv,vert[7].xy,vert[4].xy,line_width);
		i += line(uv,vert[0].xy,vert[4].xy,line_width);
		i += line(uv,vert[1].xy,vert[5].xy,line_width);
		i += line(uv,vert[2].xy,vert[6].xy,line_width);
		i += line(uv,vert[3].xy,vert[7].xy,line_width);
		c += fragment(vert[0]) * min(i,1.0);
	}

	instances[0] = vec3(-1.0, 1.0,-1.0);
	instances[1] = vec3( 1.0, 1.0,-1.0);
	instances[2] = vec3(-1.0,-1.0,-1.0);
	instances[3] = vec3( 1.0,-1.0,-1.0);
	instances[4] = vec3(-1.0, 1.0, 1.0);
	instances[5] = vec3( 1.0, 1.0, 1.0);
	instances[6] = vec3(-1.0,-1.0, 1.0);
	instances[7] = vec3( 1.0,-1.0, 1.0);

	/* cicle pipeline */
	for(int dip = 0; dip < 8; dip++) {

		/* input assembly */
		vec3 vert = vec3(0.0);

		/* vertex processing */
		mat4 pos = getPosMatrix(instances[dip] * 4.0);
		mat4 mat = pos * rot * cam;

		/* transform */
		vert = (vec4(vert,1.0) * mat).xyz;

		/* perspective */
		vert.z = 1.0 / vert.z;
		vert.xy *= vert.z;

		/* rasterize */
		c += fragment(vert) * circle(uv,vert.xy,-vert.z,line_width);
	}

	/* fragment */
	Color = vec4(c, 1.0);
}
				")
			)?);
			let pipeline = Rc::new(Pipeline::new(glcore.clone(), mesh.clone(), shader.clone())?);
			Ok(Self {
				mesh,
				shader,
				pipeline,
			})
		}

		fn render(&self, glcore: &GLCore, frame_time: f64, width: u32, height: u32) -> Result<(), AppError> {
			glcore.glViewport(0, 0, width as i32, height as i32)?;
			glcore.glClearColor(0.0, 0.3, 0.5, 1.0)?;
			glcore.glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT)?;

			let shader = self.shader.use_program()?;
			let time = frame_time as f32;
			let _ = shader.set_uniform("iResolution", &Vec3::new(width as f32, height as f32, 0.0));
			let _ = shader.set_uniform("iTime", &time);
			shader.unuse();

			let p_bind = self.pipeline.bind()?;
			p_bind.draw(None)?;
			p_bind.unbind();
			Ok(())
		}
	}

	impl AppInstance {
		pub fn new() -> Result<Self, AppError> {
			let mut glfw = match glfw::init_no_callbacks() {
				Ok(glfw) => glfw,
				Err(_) => return Err(AppError::GLFWInitErr), // According to the spec, `glfw::init_no_callbacks()` won't return `glfw::InitError::AlreadyInitialized`
			};
			let (mut window, events) = glfw.create_window(1024, 768, "GLFW Window", glfw::WindowMode::Windowed).ok_or(AppError::GLFWCreateWindowFailed)?;
			window.set_key_polling(true);
			window.make_current();
			glfw.set_swap_interval(SwapInterval::Adaptive);
			let glcore = Rc::new(GLCore::new(|proc_name|window.get_proc_address(proc_name))?);
			let renderer = Some(Renderer::new(glcore.clone())?);
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
					let (width, height) = self.window.get_framebuffer_size();
					renderer.render(&self.glcore, time_cur_frame, width as u32, height as u32).unwrap();
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
