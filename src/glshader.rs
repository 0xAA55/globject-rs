
#![allow(dead_code)]
#![allow(non_upper_case_globals)]

use glcore::*;
use std::{
	collections::BTreeMap,
	fmt::{self, Debug, Display, Formatter},
	mem::transmute,
	rc::Rc,
	string::FromUtf8Error,
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

/// The struct for monitoring using the shader
#[derive(Debug)]
pub struct ShaderUse<'a> {
	pub shader: &'a Shader,
}

/// The OpenGL attrib types
#[derive(Clone, Copy)]
pub enum AttribType {
	Float = GL_FLOAT as isize,
	Vec2 = GL_FLOAT_VEC2 as isize,
	Vec3 = GL_FLOAT_VEC3 as isize,
	Vec4 = GL_FLOAT_VEC4 as isize,
	Mat2 = GL_FLOAT_MAT2 as isize,
	Mat3 = GL_FLOAT_MAT3 as isize,
	Mat4 = GL_FLOAT_MAT4 as isize,
	Mat2x3 = GL_FLOAT_MAT2x3 as isize,
	Mat2x4 = GL_FLOAT_MAT2x4 as isize,
	Mat3x2 = GL_FLOAT_MAT3x2 as isize,
	Mat3x4 = GL_FLOAT_MAT3x4 as isize,
	Mat4x2 = GL_FLOAT_MAT4x2 as isize,
	Mat4x3 = GL_FLOAT_MAT4x3 as isize,
	Int = GL_INT as isize,
	IVec2 = GL_INT_VEC2 as isize,
	IVec3 = GL_INT_VEC3 as isize,
	IVec4 = GL_INT_VEC4 as isize,
	UInt = GL_UNSIGNED_INT as isize,
	UVec2 = GL_UNSIGNED_INT_VEC2 as isize,
	UVec3 = GL_UNSIGNED_INT_VEC3 as isize,
	UVec4 = GL_UNSIGNED_INT_VEC4 as isize,
	Double = GL_DOUBLE as isize,
	DVec2 = GL_DOUBLE_VEC2 as isize,
	DVec3 = GL_DOUBLE_VEC3 as isize,
	DVec4 = GL_DOUBLE_VEC4 as isize,
	DMat2 = GL_DOUBLE_MAT2 as isize,
	DMat3 = GL_DOUBLE_MAT3 as isize,
	DMat4 = GL_DOUBLE_MAT4 as isize,
	DMat2x3 = GL_DOUBLE_MAT2x3 as isize,
	DMat2x4 = GL_DOUBLE_MAT2x4 as isize,
	DMat3x2 = GL_DOUBLE_MAT3x2 as isize,
	DMat3x4 = GL_DOUBLE_MAT3x4 as isize,
	DMat4x2 = GL_DOUBLE_MAT4x2 as isize,
	DMat4x3 = GL_DOUBLE_MAT4x3 as isize,
}

#[derive(Debug, Clone, Copy)]
pub struct AttribVarType {
	pub type_: AttribType,
	pub size: i32,
}

impl Shader {
	/// Get the internal name
	pub fn get_name(&self) -> u32 {
		self.program
	}

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

	/// Get all of the active attributes of the shader
	pub fn get_active_attribs(&self) -> Result<BTreeMap<String, AttribVarType>, FromUtf8Error> {
		let mut num_attrib: i32 = 0;
		let mut max_length: i32 = 0;
		self.glcore.glGetProgramiv(self.program, GL_ACTIVE_ATTRIBUTES, &mut num_attrib as *mut _);
		self.glcore.glGetProgramiv(self.program, GL_ACTIVE_ATTRIBUTE_MAX_LENGTH, &mut max_length as *mut _);

		let mut ret = BTreeMap::<String, AttribVarType>::new();
		for i in 0..num_attrib {
			let mut name = vec![0i8; max_length as usize];
			let mut size: i32 = 0;
			let mut type_: u32 = 0;
			self.glcore.glGetActiveAttrib(self.program, i as u32, max_length, 0 as *mut i32, &mut size as *mut _, &mut type_ as *mut _, name.as_mut_ptr());
			let name = String::from_utf8(unsafe{transmute(name)})?;
			let type_ = AttribType::from(type_);
			ret.insert(name, AttribVarType{type_, size});
		}
		Ok(ret)
	}

	/// Set to use the shader
	pub fn use_program<'a>(&'a self) -> ShaderUse<'a> {
		ShaderUse::new(self)
	}
}

impl<'a> ShaderUse<'a> {
	fn new(shader: &'a Shader) -> Self {
		shader.glcore.glUseProgram(shader.get_name());
		Self {
			shader,
		}
	}

	/// Unuse the program.
	pub fn unuse(self) {}
}

impl Drop for ShaderUse<'_> {
	fn drop(&mut self) {
		self.shader.glcore.glUseProgram(0)
	}
}

impl Drop for Shader {
	fn drop(&mut self) {
		self.glcore.glDeleteProgram(self.program)
	}
}

impl AttribType {
	pub fn is_float(&self) -> bool {
		match self {
			Self::Float | Self::Vec2 | Self::Vec3 | Self::Vec4 | Self::Mat2 | Self::Mat3 | Self::Mat4 | Self::Mat2x3 | Self::Mat2x4 | Self::Mat3x2 | Self::Mat3x4 | Self::Mat4x2 | Self::Mat4x3 => true,
			_ => false,
		}
	}

	pub fn is_double(&self) -> bool {
		match self {
			Self::Double | Self::DVec2 | Self::DVec3 | Self::DVec4 | Self::DMat2 | Self::DMat3 | Self::DMat4 | Self::DMat2x3 | Self::DMat2x4 | Self::DMat3x2 | Self::DMat3x4 | Self::DMat4x2 | Self::DMat4x3 => true,
			_ => false,
		}
	}

	pub fn is_integer(&self) -> bool {
		match self {
			Self::Int | Self::IVec2 | Self::IVec3 | Self::IVec4 | Self::UInt | Self::UVec2 | Self::UVec3 | Self::UVec4 => true,
			_ => false,
		}
	}

	pub fn get_size_and_rows(&self) -> (u32, u32) {
		match self {
			Self::Float | Self::Double | Self::Int | Self::UInt => (1, 1),
			Self::Vec2 | Self::DVec2 | Self::IVec2 | Self::UVec2 => (2, 1),
			Self::Vec3 | Self::DVec3 | Self::IVec3 | Self::UVec3 => (3, 1),
			Self::Vec4 | Self::DVec4 | Self::IVec4 | Self::UVec4 => (4, 1),
			Self::Mat2 | Self::DMat2 => (2, 2),
			Self::Mat3 | Self::DMat3 => (3, 3),
			Self::Mat4 | Self::DMat4 => (4, 4),
			Self::Mat2x3 | Self::DMat2x3 => (2, 3),
			Self::Mat2x4 | Self::DMat2x4 => (2, 4),
			Self::Mat3x2 | Self::DMat3x2 => (3, 2),
			Self::Mat3x4 | Self::DMat3x4 => (3, 4),
			Self::Mat4x2 | Self::DMat4x2 => (4, 2),
			Self::Mat4x3 | Self::DMat4x3 => (4, 3),
		}
	}
}

impl AttribVarType {
	pub fn is_float(&self) -> bool {
		self.type_.is_float()
	}

	pub fn is_double(&self) -> bool {
		self.type_.is_double()
	}

	pub fn is_integer(&self) -> bool {
		self.type_.is_integer()
	}

	pub fn get_size_and_rows(&self) -> (u32, u32) {
		self.type_.get_size_and_rows()
	}

	pub fn get_type(&self) -> AttribType {
		self.type_
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

impl From<u32> for AttribType {
	fn from(val: u32) -> Self {
		match val {
			GL_FLOAT => Self::Float,
			GL_FLOAT_VEC2 => Self::Vec2,
			GL_FLOAT_VEC3 => Self::Vec3,
			GL_FLOAT_VEC4 => Self::Vec4,
			GL_FLOAT_MAT2 => Self::Mat2,
			GL_FLOAT_MAT3 => Self::Mat3,
			GL_FLOAT_MAT4 => Self::Mat4,
			GL_FLOAT_MAT2x3 => Self::Mat2x3,
			GL_FLOAT_MAT2x4 => Self::Mat2x4,
			GL_FLOAT_MAT3x2 => Self::Mat3x2,
			GL_FLOAT_MAT3x4 => Self::Mat3x4,
			GL_FLOAT_MAT4x2 => Self::Mat4x2,
			GL_FLOAT_MAT4x3 => Self::Mat4x3,
			GL_INT => Self::Int,
			GL_INT_VEC2 => Self::IVec2,
			GL_INT_VEC3 => Self::IVec3,
			GL_INT_VEC4 => Self::IVec4,
			GL_UNSIGNED_INT => Self::UInt,
			GL_UNSIGNED_INT_VEC2 => Self::UVec2,
			GL_UNSIGNED_INT_VEC3 => Self::UVec3,
			GL_UNSIGNED_INT_VEC4 => Self::UVec4,
			GL_DOUBLE => Self::Double,
			GL_DOUBLE_VEC2 => Self::DVec2,
			GL_DOUBLE_VEC3 => Self::DVec3,
			GL_DOUBLE_VEC4 => Self::DVec4,
			GL_DOUBLE_MAT2 => Self::DMat2,
			GL_DOUBLE_MAT3 => Self::DMat3,
			GL_DOUBLE_MAT4 => Self::DMat4,
			GL_DOUBLE_MAT2x3 => Self::DMat2x3,
			GL_DOUBLE_MAT2x4 => Self::DMat2x4,
			GL_DOUBLE_MAT3x2 => Self::DMat3x2,
			GL_DOUBLE_MAT3x4 => Self::DMat3x4,
			GL_DOUBLE_MAT4x2 => Self::DMat4x2,
			GL_DOUBLE_MAT4x3 => Self::DMat4x3,
			_ => panic!("Invalid value {val} of `AttribType`"),
		}
	}
}

impl Debug for AttribType {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match self {
			Self::Float => write!(f, "Float"),
			Self::Vec2 => write!(f, "Vec2"),
			Self::Vec3 => write!(f, "Vec3"),
			Self::Vec4 => write!(f, "Vec4"),
			Self::Mat2 => write!(f, "Mat2"),
			Self::Mat3 => write!(f, "Mat3"),
			Self::Mat4 => write!(f, "Mat4"),
			Self::Mat2x3 => write!(f, "Mat2x3"),
			Self::Mat2x4 => write!(f, "Mat2x4"),
			Self::Mat3x2 => write!(f, "Mat3x2"),
			Self::Mat3x4 => write!(f, "Mat3x4"),
			Self::Mat4x2 => write!(f, "Mat4x2"),
			Self::Mat4x3 => write!(f, "Mat4x3"),
			Self::Int => write!(f, "Int"),
			Self::IVec2 => write!(f, "IVec2"),
			Self::IVec3 => write!(f, "IVec3"),
			Self::IVec4 => write!(f, "IVec4"),
			Self::UInt => write!(f, "UInt"),
			Self::UVec2 => write!(f, "UVec2"),
			Self::UVec3 => write!(f, "UVec3"),
			Self::UVec4 => write!(f, "UVec4"),
			Self::Double => write!(f, "Double"),
			Self::DVec2 => write!(f, "DVec2"),
			Self::DVec3 => write!(f, "DVec3"),
			Self::DVec4 => write!(f, "DVec4"),
			Self::DMat2 => write!(f, "DMat2"),
			Self::DMat3 => write!(f, "DMat3"),
			Self::DMat4 => write!(f, "DMat4"),
			Self::DMat2x3 => write!(f, "DMat2x3"),
			Self::DMat2x4 => write!(f, "DMat2x4"),
			Self::DMat3x2 => write!(f, "DMat3x2"),
			Self::DMat3x4 => write!(f, "DMat3x4"),
			Self::DMat4x2 => write!(f, "DMat4x2"),
			Self::DMat4x3 => write!(f, "DMat4x3"),
		}
	}
}

impl Display for AttribType {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		<Self as Debug>::fmt(self, f)
	}
}
