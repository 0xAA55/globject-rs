#![allow(dead_code)]

use glcore::*;

pub enum ShaderError {
	VSError(String),
	GSError(String),
	FSError(String),
	LinakgeError(String),
	CSError(String),
}

pub struct Shader<'a> {
	glcore: &'a GLCore,
	program: u32,
}

impl<'a> Shader<'a> {
	fn compile_shader(glcore: &'a GLCore, shader_type: u32, shader_source: &str) -> Result<u32, String> {
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

	fn link_program(glcore: &'a GLCore, program: u32)  -> Result<Self, ShaderError> {
		glcore.glLinkProgram(program);
		let mut linked: i32 = 0;
		glcore.glGetProgramiv(program, GL_LINK_STATUS, &mut linked as *mut i32);
		if linked != 0 {
			Ok(Self{
				glcore,
				program,
			})
		} else {
			let mut output_len: i32 = 0;
			glcore.glGetProgramiv(program, GL_INFO_LOG_LENGTH, &mut output_len as *mut i32);
			let mut output =  Vec::<u8>::new();
			let mut output_len_ret: i32 = 0;
			output.resize(output_len as usize, 0);
			glcore.glGetProgramInfoLog(program, output_len, &mut output_len_ret as *mut i32, output.as_mut_ptr() as *mut i8);
			glcore.glDeleteProgram(program);
			let output = String::from_utf8_lossy(&output).to_string();
			Err(ShaderError::LinakgeError(output))
		}
	}

	pub fn new(glcore: &'a GLCore, vertex_shader: Option<&str>, geometry_shader: Option<&str>, fragment_shader: Option<&str>) -> Result<Self, ShaderError> {
		let program = glcore.glCreateProgram();
		if let Some(vertex_shader) = vertex_shader {
			match Self::compile_shader(glcore, GL_VERTEX_SHADER, vertex_shader) {
				Ok(shader) => {
					glcore.glAttachShader(program, shader);
					glcore.glDeleteShader(shader);
				}
				Err(output) => return Err(ShaderError::VSError(output)),
			};
		}
		if let Some(geometry_shader) = geometry_shader {
			match Self::compile_shader(glcore, GL_GEOMETRY_SHADER, geometry_shader) {
				Ok(shader) => {
					glcore.glAttachShader(program, shader);
					glcore.glDeleteShader(shader);
				}
				Err(output) => return Err(ShaderError::GSError(output)),
			};
		}
		if let Some(fragment_shader) = fragment_shader {
			match Self::compile_shader(glcore, GL_FRAGMENT_SHADER, fragment_shader) {
				Ok(shader) => {
					glcore.glAttachShader(program, shader);
					glcore.glDeleteShader(shader);
				}
				Err(output) => return Err(ShaderError::FSError(output)),
			};
		}
		Self::link_program(glcore, program)
	}

	pub fn new_compute(glcore: &'a GLCore, shader_source: &str) -> Result<Self, ShaderError> {
		let program = glcore.glCreateProgram();
		match Self::compile_shader(glcore, GL_COMPUTE_SHADER, shader_source) {
			Ok(shader) => {
				glcore.glAttachShader(program, shader);
				glcore.glDeleteShader(shader);
			}
			Err(output) => return Err(ShaderError::CSError(output)),
		};
		Self::link_program(glcore, program)
	}

	pub fn use_(&self) {
		self.glcore.glUseProgram(self.program)
	}

	fn drop(&self) {
		self.glcore.glDeleteProgram(self.program)
	}
}
