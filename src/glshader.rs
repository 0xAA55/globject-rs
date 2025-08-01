
#![allow(dead_code)]

use glcore::*;
use std::{
	fmt::{self, Debug, Formatter},
	rc::Rc,
};

/// Error produced from the shader
#[derive(Clone)]
pub enum ShaderError {
	/// Vertex Shader error
	VSError(String),

	/// Geometry Shader error
	GSError(String),

	/// Fragment Shader error
	FSError(String),
	
	/// Compute Shader error
	CSError(String),

	/// Shader program linkage error
	LinkageError(String),
}

/// The OpenGL shader object
pub struct Shader {
	glcore: Rc<GLCore>,
	program: u32,
}

impl Shader {
	/// Compile a shader, returns the compiled shader object or the compiler info log
	fn compile_shader(glcore: &GLCore, shader_type: u32, shader_source: &str) -> Result<u32, String> {
		let shader = glcore.glCreateShader(shader_type);
		let bytes: Vec<i8> = shader_source.bytes().map(|byte| -> i8 {byte as i8}).collect();
		let ptr_to_bytes = bytes.as_ptr();
		let length = bytes.len() as i32;
		glcore.glShaderSource(shader, 1, &ptr_to_bytes as *const *const i8, &length as *const i32);
		glcore.glCompileShader(shader);

		let mut compiled: i32 = 0;
		glcore.glGetShaderiv(shader, GL_COMPILE_STATUS, &mut compiled as *mut i32);
		if compiled != 0 {
			Ok(shader)
		} else {
			let mut output_len: i32 = 0;
			glcore.glGetShaderiv(shader, GL_INFO_LOG_LENGTH, &mut output_len as *mut i32);
			let mut output =  Vec::<u8>::new();
			let mut output_len_ret: i32 = 0;
			output.resize(output_len as usize, 0);
			glcore.glGetShaderInfoLog(shader, output_len, &mut output_len_ret as *mut i32, output.as_mut_ptr() as *mut i8);
			glcore.glDeleteShader(shader);
			let output = String::from_utf8_lossy(&output).to_string();
			Err(output)
		}
	}

	/// Link a shader program, returns the compiled shader program object or the compiler/linker info log
	fn link_program(glcore: &GLCore, program: u32)  -> Result<(), ShaderError> {
		glcore.glLinkProgram(program);
		let mut linked: i32 = 0;
		glcore.glGetProgramiv(program, GL_LINK_STATUS, &mut linked as *mut i32);
		if linked != 0 {
			Ok(())
		} else {
			let mut output_len: i32 = 0;
			glcore.glGetProgramiv(program, GL_INFO_LOG_LENGTH, &mut output_len as *mut i32);
			let mut output =  Vec::<u8>::new();
			let mut output_len_ret: i32 = 0;
			output.resize(output_len as usize, 0);
			glcore.glGetProgramInfoLog(program, output_len, &mut output_len_ret as *mut i32, output.as_mut_ptr() as *mut i8);
			glcore.glDeleteProgram(program);
			let output = String::from_utf8_lossy(&output).to_string();
			Err(ShaderError::LinkageError(output))
		}
	}

	/// Create a new traditional renderer shader program
	pub fn new(glcore: Rc<GLCore>, vertex_shader: Option<&str>, geometry_shader: Option<&str>, fragment_shader: Option<&str>) -> Result<Self, ShaderError> {
		let program = glcore.glCreateProgram();
		if let Some(vertex_shader) = vertex_shader {
			match Self::compile_shader(glcore.as_ref(), GL_VERTEX_SHADER, vertex_shader) {
				Ok(shader) => {
					glcore.glAttachShader(program, shader);
					glcore.glDeleteShader(shader);
				}
				Err(output) => return Err(ShaderError::VSError(output)),
			};
		}
		if let Some(geometry_shader) = geometry_shader {
			match Self::compile_shader(glcore.as_ref(), GL_GEOMETRY_SHADER, geometry_shader) {
				Ok(shader) => {
					glcore.glAttachShader(program, shader);
					glcore.glDeleteShader(shader);
				}
				Err(output) => return Err(ShaderError::GSError(output)),
			};
		}
		if let Some(fragment_shader) = fragment_shader {
			match Self::compile_shader(glcore.as_ref(), GL_FRAGMENT_SHADER, fragment_shader) {
				Ok(shader) => {
					glcore.glAttachShader(program, shader);
					glcore.glDeleteShader(shader);
				}
				Err(output) => return Err(ShaderError::FSError(output)),
			};
		}
		Self::link_program(glcore.as_ref(), program)?;
		Ok(Self {
			glcore,
			program,
		})
	}

	/// Create a new compute shader program
	pub fn new_compute(glcore: Rc<GLCore>, shader_source: &str) -> Result<Self, ShaderError> {
		let program = glcore.glCreateProgram();
		match Self::compile_shader(glcore.as_ref(), GL_COMPUTE_SHADER, shader_source) {
			Ok(shader) => {
				glcore.glAttachShader(program, shader);
				glcore.glDeleteShader(shader);
			}
			Err(output) => return Err(ShaderError::CSError(output)),
		};
		Self::link_program(glcore.as_ref(), program)?;
		Ok(Self {
			glcore,
			program,
		})
	}

	/// Set to use the shader
	pub fn use_(&self) {
		self.glcore.glUseProgram(self.program)
	}
}

impl Drop for Shader {
	fn drop(&mut self) {
		self.glcore.glDeleteProgram(self.program)
	}
}

impl Debug for Shader {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		f.debug_struct("Shader")
		.field("program", &self.program)
		.finish()
	}
}

impl Debug for ShaderError {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match self {
			Self::VSError(infolog) => write!(f, "Vertex Shader Error:\n{infolog}"),
			Self::GSError(infolog) => write!(f, "Geometry Shader Error:\n{infolog}"),
			Self::FSError(infolog) => write!(f, "Fragment Shader Error:\n{infolog}"),
			Self::CSError(infolog) => write!(f, "Compute Shader Error:\n{infolog}"),
			Self::LinkageError(infolog) => write!(f, "Shader Linkage Error:\n{infolog}"),
		}
	}
}
