
#![allow(non_upper_case_globals)]

use crate::prelude::*;
use std::{
	any::{Any, type_name},
	collections::BTreeMap,
	ffi::{CString, c_void},
	fmt::{self, Debug, Display, Formatter},
	mem::{transmute, size_of},
	path::Path,
	ptr::null_mut,
	rc::Rc,
	string::FromUtf8Error,
};
use bincode::{Encode, Decode};

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

	/// Attrib not found
	AttribNotFound(String),

	/// Uniform not found
	UniformNotFound(String),
}

/// Error produced from the shader
#[derive(Encode, Decode, Debug, Clone, Copy, PartialEq)]
pub enum ShaderType {
	Draw,
	Compute,
}

/// The OpenGL shader object
pub struct Shader {
	glcore: Rc<GLCore>,
	program: u32,
	shader_type: ShaderType,
}

/// The struct for monitoring using the shader
#[derive(Debug)]
pub struct ShaderUse<'a> {
	pub shader: &'a Shader,
}

/// The pre-compiled OpenGL shader binary
#[derive(Encode, Decode, Debug, Clone)]
pub struct ShaderBinary {
	format: u32,
	shader_type: ShaderType,
	binary: Vec<u8>,
}

/// The error info of loading the shader binary
#[derive(Debug)]
pub enum ShaderBinaryLoadError {
	IOError(std::io::Error),
	DecodeError(bincode::error::DecodeError),
}

/// The error info of storing the shader binary
#[derive(Debug)]
pub enum ShaderBinarySaveError {
	IOError(std::io::Error),
	EncodeError(bincode::error::EncodeError),
}

/// The OpenGL attrib types
#[derive(Clone, Copy)]
pub enum ShaderInputType {
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

/// The OpenGL attrib type with length
#[derive(Debug, Clone, Copy)]
pub struct ShaderInputVarType {
	pub type_: ShaderInputType,
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

	/// Link a shader program, returns compiler/linker info log if linkage isn't successful.
	fn link_program(glcore: &GLCore, program: u32) -> Result<(), ShaderError> {
		glcore.glLinkProgram(program);
		Self::get_linkage_status(glcore, program)
	}

	/// Get the program linkage status, returns compiler/linker info log if linkage isn't successful.
	fn get_linkage_status(glcore: &GLCore, program: u32)  -> Result<(), ShaderError> {
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
			shader_type: ShaderType::Draw,
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
			shader_type: ShaderType::Compute,
		})
	}

	/// Get all of the active attributes of the shader
	pub fn get_active_attribs(&self) -> Result<BTreeMap<String, ShaderInputVarType>, FromUtf8Error> {
		let mut num_attribs: i32 = 0;
		let mut max_length: i32 = 0;
		self.glcore.glGetProgramiv(self.program, GL_ACTIVE_ATTRIBUTES, &mut num_attribs as *mut _);
		self.glcore.glGetProgramiv(self.program, GL_ACTIVE_ATTRIBUTE_MAX_LENGTH, &mut max_length as *mut _);

		let mut ret = BTreeMap::<String, ShaderInputVarType>::new();
		for i in 0..num_attribs {
			let mut name = vec![0i8; max_length as usize];
			let mut size: i32 = 0;
			let mut type_: u32 = 0;
			self.glcore.glGetActiveAttrib(self.program, i as u32, max_length, null_mut::<i32>(), &mut size as *mut _, &mut type_ as *mut _, name.as_mut_ptr());
			let name = String::from_utf8(unsafe{transmute::<Vec<i8>, Vec<u8>>(name)})?;
			let name = name.trim_end_matches('\0').to_string();
			let type_ = ShaderInputType::from(type_);
			ret.insert(name, ShaderInputVarType{type_, size});
		}
		Ok(ret)
	}

	/// Get the location of the shader attrib
	pub fn get_attrib_location(&self, attrib_name: &str) -> i32 {
		let attrib_name = CString::new(attrib_name).unwrap();
		self.glcore.glGetAttribLocation(self.program, attrib_name.as_ptr())
	}

	/// Get all of the active uniforms of the shader
	pub fn get_active_uniforms(&self) -> Result<BTreeMap<String, ShaderInputVarType>, FromUtf8Error> {
		let mut num_uniforms: i32 = 0;
		let mut max_length: i32 = 0;
		self.glcore.glGetProgramiv(self.program, GL_ACTIVE_UNIFORMS, &mut num_uniforms as *mut _);
		self.glcore.glGetProgramiv(self.program, GL_ACTIVE_UNIFORM_MAX_LENGTH, &mut max_length as *mut _);

		let mut ret = BTreeMap::<String, ShaderInputVarType>::new();
		for i in 0..num_uniforms {
			let mut name = vec![0i8; max_length as usize];
			let mut size: i32 = 0;
			let mut type_: u32 = 0;
			self.glcore.glGetActiveUniform(self.program, i as u32, max_length, null_mut::<i32>(), &mut size as *mut _, &mut type_ as *mut _, name.as_mut_ptr());
			let name = String::from_utf8(unsafe{transmute::<Vec<i8>, Vec<u8>>(name)})?;
			let name = name.trim_end_matches('\0').to_string();
			let type_ = ShaderInputType::from(type_);
			ret.insert(name, ShaderInputVarType{type_, size});
		}
		Ok(ret)
	}

	/// Get the location of the shader attrib
	pub fn get_uniform_location(&self, uniform_name: &str) -> i32 {
		let uniform_name = CString::new(uniform_name).unwrap();
		self.glcore.glGetUniformLocation(self.program, uniform_name.as_ptr())
	}

	/// Get the compiled + linked program binary
	pub fn get_program_binary(&self) -> ShaderBinary {
		let mut binary_length = 0;
		let mut binary_format = 0;
		self.glcore.glGetProgramiv(self.program, GL_PROGRAM_BINARY_LENGTH, &mut binary_length as *mut _);
		let mut binary = Vec::<u8>::new();
		binary.resize(binary_length as usize, 0);
		self.glcore.glGetProgramBinary(self.program, binary_length, null_mut(), &mut binary_format as *mut _, binary.as_mut_ptr() as *mut _);
		ShaderBinary::new(binary_format, self.shader_type, binary)
	}

	/// Create a program from pre-compiled binary
	pub fn from_program_binary(glcore: Rc<GLCore>, binary: &ShaderBinary) -> Result<Self, ShaderError> {
		let program = glcore.glCreateProgram();
		glcore.glProgramBinary(program, binary.format, binary.binary.as_ptr() as *const _, binary.binary.len() as i32);
		match Self::get_linkage_status(&glcore, program) {
			Ok(_) => Ok(Self {
				glcore,
				shader_type: binary.shader_type,
				program,
			}),
			Err(e) => {
				glcore.glDeleteProgram(program);
				Err(e)
			}
		}
	}

	/// Set to use the shader
	pub fn use_program<'a>(&'a self) -> ShaderUse<'a> {
		ShaderUse::new(self)
	}
}

impl<'a> ShaderUse<'a> {
	/// Create a new `using` state to the `Shader`
	fn new(shader: &'a Shader) -> Self {
		shader.glcore.glUseProgram(shader.get_name());
		Self {
			shader,
		}
	}

	/// Dispatch the compute shader
	pub fn dispatch_compute(&self, num_groups_x: u32, num_groups_y: u32, num_groups_z: u32) {
		if self.shader.shader_type != ShaderType::Compute {
			panic!("Only compute shaders could use the `dispatch_compute()` method.");
		}
		self.shader.glcore.glDispatchCompute(num_groups_x, num_groups_y, num_groups_z);
	}

	/// Dispatch the compute shader
	pub fn dispatch_compute_indirect(&self, buffer: &Buffer, index: usize) {
		if self.shader.shader_type != ShaderType::Compute {
			panic!("Only compute shaders could use the `dispatch_compute_indirect()` method.");
		}
		let bind = buffer.bind_to(BufferTarget::DispatchIndirectBuffer);
		self.shader.glcore.glDispatchComputeIndirect(index * size_of::<DispatchIndirectCommand>());
		bind.unbind();
	}

	/// Wrapper for matrices of attrib
	pub unsafe fn vertex_attrib_matrix_pointer(&self, location: u32, cols: u32, rows: u32, base_type: ShaderInputType, normalize: bool, stride: isize, pointer: *const c_void) {
		match base_type {
			ShaderInputType::Float => {
				for i in 0..rows {
					self.shader.glcore.glVertexAttribPointer(location + i, cols as i32, base_type as u32, normalize as u8, stride as i32, pointer as *const _)
				}
			}
			ShaderInputType::Double => {
				for i in 0..rows {
					self.shader.glcore.glVertexAttribLPointer(location + i, cols as i32, base_type as u32, stride as i32, pointer as *const _)
				}
			}
			_ => panic!("Bad parameter for `vertex_attrib_matrix_pointer()`: base_type = {base_type:?}"),
		}
	}

	/// Set attrib value by pointer
	pub unsafe fn set_attrib_ptr<T: Any>(&self, name: &str, attrib_type: &ShaderInputVarType, do_normalize: bool, stride: isize, ptr_param: *const c_void) -> Result<(), ShaderError> {
		let location = self.shader.get_attrib_location(&name);
		if location >= 0 {
			let location = location as u32;
			let (p_size, p_rows) = attrib_type.get_size_and_rows();
			if type_name::<T>().ends_with("f32") {unsafe {self.vertex_attrib_matrix_pointer(location, p_size, p_rows, attrib_type.get_base_type(), do_normalize, stride, ptr_param)}} else
			if type_name::<T>().ends_with("i32") {self.shader.glcore.glVertexAttribIPointer(location, p_size as i32, attrib_type.get_base_type() as u32, stride as i32, ptr_param)} else
			if type_name::<T>().ends_with("u32") {self.shader.glcore.glVertexAttribIPointer(location, p_size as i32, attrib_type.get_base_type() as u32, stride as i32, ptr_param)} else
			if type_name::<T>().ends_with("f64") {unsafe {self.vertex_attrib_matrix_pointer(location, p_size, p_rows, attrib_type.get_base_type(), do_normalize, stride, ptr_param)}} else
			{panic!("The generic type parameter of `ShaderUse::set_attrib_ptr()` must be `f32`, `i32`, `u32`, `f64`")}
			Ok(())
		} else {
			Err(ShaderError::AttribNotFound(name.to_owned()))
		}
	}

	/// Set attrib value
	pub fn set_attrib(&self, name: &str, v: &dyn Any) -> Result<(), ShaderError> {
		let location = self.shader.get_attrib_location(&name);
		if location >= 0 {
			let location = location as u32;
			if let Some(v) = v.downcast_ref::<f32>()		{self.shader.glcore.glVertexAttribPointer (location, 1, ShaderInputType::Float as u32, 0, 0, (v as *const f32) as *const _)} else
			if let Some(v) = v.downcast_ref::<Vec2>()		{self.shader.glcore.glVertexAttribPointer (location, 2, ShaderInputType::Float as u32, 0, 0, v.as_ptr() as *const _)} else
			if let Some(v) = v.downcast_ref::<Vec3>()		{self.shader.glcore.glVertexAttribPointer (location, 3, ShaderInputType::Float as u32, 0, 0, v.as_ptr() as *const _)} else
			if let Some(v) = v.downcast_ref::<Vec4>()		{self.shader.glcore.glVertexAttribPointer (location, 4, ShaderInputType::Float as u32, 0, 0, v.as_ptr() as *const _)} else
			if let Some(v) = v.downcast_ref::<Mat2>()		{unsafe {self.vertex_attrib_matrix_pointer(location, 2, 2, ShaderInputType::Float, false, 0, v.as_ptr() as *const _)}} else
			if let Some(v) = v.downcast_ref::<Mat3>()		{unsafe {self.vertex_attrib_matrix_pointer(location, 3, 3, ShaderInputType::Float, false, 0, v.as_ptr() as *const _)}} else
			if let Some(v) = v.downcast_ref::<Mat4>()		{unsafe {self.vertex_attrib_matrix_pointer(location, 4, 4, ShaderInputType::Float, false, 0, v.as_ptr() as *const _)}} else
			if let Some(v) = v.downcast_ref::<Mat2x3>()		{unsafe {self.vertex_attrib_matrix_pointer(location, 2, 3, ShaderInputType::Float, false, 0, v.as_ptr() as *const _)}} else
			if let Some(v) = v.downcast_ref::<Mat2x4>()		{unsafe {self.vertex_attrib_matrix_pointer(location, 2, 4, ShaderInputType::Float, false, 0, v.as_ptr() as *const _)}} else
			if let Some(v) = v.downcast_ref::<Mat3x2>()		{unsafe {self.vertex_attrib_matrix_pointer(location, 3, 2, ShaderInputType::Float, false, 0, v.as_ptr() as *const _)}} else
			if let Some(v) = v.downcast_ref::<Mat3x4>()		{unsafe {self.vertex_attrib_matrix_pointer(location, 3, 4, ShaderInputType::Float, false, 0, v.as_ptr() as *const _)}} else
			if let Some(v) = v.downcast_ref::<Mat4x2>()		{unsafe {self.vertex_attrib_matrix_pointer(location, 4, 2, ShaderInputType::Float, false, 0, v.as_ptr() as *const _)}} else
			if let Some(v) = v.downcast_ref::<Mat4x3>()		{unsafe {self.vertex_attrib_matrix_pointer(location, 4, 3, ShaderInputType::Float, false, 0, v.as_ptr() as *const _)}} else
			if let Some(v) = v.downcast_ref::<i32>()		{self.shader.glcore.glVertexAttribIPointer(location, 1, ShaderInputType::Int as u32, 0, (v as *const i32) as *const _)} else
			if let Some(v) = v.downcast_ref::<IVec2>()		{self.shader.glcore.glVertexAttribIPointer(location, 2, ShaderInputType::Int as u32, 0, v.as_ptr() as *const _)} else
			if let Some(v) = v.downcast_ref::<IVec3>()		{self.shader.glcore.glVertexAttribIPointer(location, 3, ShaderInputType::Int as u32, 0, v.as_ptr() as *const _)} else
			if let Some(v) = v.downcast_ref::<IVec4>()		{self.shader.glcore.glVertexAttribIPointer(location, 4, ShaderInputType::Int as u32, 0, v.as_ptr() as *const _)} else
			if let Some(v) = v.downcast_ref::<u32>()		{self.shader.glcore.glVertexAttribIPointer(location, 1, ShaderInputType::UInt as u32, 0, (v as *const u32) as *const _)} else
			if let Some(v) = v.downcast_ref::<UVec2>()		{self.shader.glcore.glVertexAttribIPointer(location, 2, ShaderInputType::UInt as u32, 0, v.as_ptr() as *const _)} else
			if let Some(v) = v.downcast_ref::<UVec3>()		{self.shader.glcore.glVertexAttribIPointer(location, 3, ShaderInputType::UInt as u32, 0, v.as_ptr() as *const _)} else
			if let Some(v) = v.downcast_ref::<UVec4>()		{self.shader.glcore.glVertexAttribIPointer(location, 4, ShaderInputType::UInt as u32, 0, v.as_ptr() as *const _)} else
			if let Some(v) = v.downcast_ref::<f64>()		{self.shader.glcore.glVertexAttribLPointer(location, 1, ShaderInputType::Double as u32, 0, (v as *const f64) as *const _)} else
			if let Some(v) = v.downcast_ref::<DVec2>()		{self.shader.glcore.glVertexAttribLPointer(location, 2, ShaderInputType::Double as u32, 0, v.as_ptr() as *const _)} else
			if let Some(v) = v.downcast_ref::<DVec3>()		{self.shader.glcore.glVertexAttribLPointer(location, 3, ShaderInputType::Double as u32, 0, v.as_ptr() as *const _)} else
			if let Some(v) = v.downcast_ref::<DVec4>()		{self.shader.glcore.glVertexAttribLPointer(location, 4, ShaderInputType::Double as u32, 0, v.as_ptr() as *const _)} else
			if let Some(v) = v.downcast_ref::<DMat2>()		{unsafe {self.vertex_attrib_matrix_pointer(location, 2, 2, ShaderInputType::Double, false, 0, v.as_ptr() as *const _)}} else
			if let Some(v) = v.downcast_ref::<DMat3>()		{unsafe {self.vertex_attrib_matrix_pointer(location, 3, 3, ShaderInputType::Double, false, 0, v.as_ptr() as *const _)}} else
			if let Some(v) = v.downcast_ref::<DMat4>()		{unsafe {self.vertex_attrib_matrix_pointer(location, 4, 4, ShaderInputType::Double, false, 0, v.as_ptr() as *const _)}} else
			if let Some(v) = v.downcast_ref::<DMat2x3>()	{unsafe {self.vertex_attrib_matrix_pointer(location, 2, 3, ShaderInputType::Double, false, 0, v.as_ptr() as *const _)}} else
			if let Some(v) = v.downcast_ref::<DMat2x4>()	{unsafe {self.vertex_attrib_matrix_pointer(location, 2, 4, ShaderInputType::Double, false, 0, v.as_ptr() as *const _)}} else
			if let Some(v) = v.downcast_ref::<DMat3x2>()	{unsafe {self.vertex_attrib_matrix_pointer(location, 3, 2, ShaderInputType::Double, false, 0, v.as_ptr() as *const _)}} else
			if let Some(v) = v.downcast_ref::<DMat3x4>()	{unsafe {self.vertex_attrib_matrix_pointer(location, 3, 4, ShaderInputType::Double, false, 0, v.as_ptr() as *const _)}} else
			if let Some(v) = v.downcast_ref::<DMat4x2>()	{unsafe {self.vertex_attrib_matrix_pointer(location, 4, 2, ShaderInputType::Double, false, 0, v.as_ptr() as *const _)}} else
			if let Some(v) = v.downcast_ref::<DMat4x3>()	{unsafe {self.vertex_attrib_matrix_pointer(location, 4, 3, ShaderInputType::Double, false, 0, v.as_ptr() as *const _)}} else
			{panic!("Unknown type of attrib value: {v:?}")}
			Ok(())
		} else {
			Err(ShaderError::AttribNotFound(name.to_owned()))
		}
	}

	/// Set uniform value
	pub fn set_uniform(&self, name: &str, v: &dyn Any) -> Result<(), ShaderError> {
		let location = self.shader.get_uniform_location(&name);
		if location >= 0 {
			if let Some(v) = v.downcast_ref::<f32>()		{self.shader.glcore.glUniform1fv(location, 1, v as *const _)} else
			if let Some(v) = v.downcast_ref::<Vec2>()		{self.shader.glcore.glUniform2fv(location, 1, v.as_ptr())} else
			if let Some(v) = v.downcast_ref::<Vec3>()		{self.shader.glcore.glUniform3fv(location, 1, v.as_ptr())} else
			if let Some(v) = v.downcast_ref::<Vec4>()		{self.shader.glcore.glUniform4fv(location, 1, v.as_ptr())} else
			if let Some(v) = v.downcast_ref::<Mat2>()		{self.shader.glcore.glUniformMatrix2fv(location, 1, 0, v.as_ptr())} else
			if let Some(v) = v.downcast_ref::<Mat3>()		{self.shader.glcore.glUniformMatrix3fv(location, 1, 0, v.as_ptr())} else
			if let Some(v) = v.downcast_ref::<Mat4>()		{self.shader.glcore.glUniformMatrix4fv(location, 1, 0, v.as_ptr())} else
			if let Some(v) = v.downcast_ref::<Mat2x3>()		{self.shader.glcore.glUniformMatrix2x3fv(location, 1, 0, v.as_ptr())} else
			if let Some(v) = v.downcast_ref::<Mat2x4>()		{self.shader.glcore.glUniformMatrix2x4fv(location, 1, 0, v.as_ptr())} else
			if let Some(v) = v.downcast_ref::<Mat3x2>()		{self.shader.glcore.glUniformMatrix3x2fv(location, 1, 0, v.as_ptr())} else
			if let Some(v) = v.downcast_ref::<Mat3x4>()		{self.shader.glcore.glUniformMatrix3x4fv(location, 1, 0, v.as_ptr())} else
			if let Some(v) = v.downcast_ref::<Mat4x2>()		{self.shader.glcore.glUniformMatrix4x2fv(location, 1, 0, v.as_ptr())} else
			if let Some(v) = v.downcast_ref::<Mat4x3>()		{self.shader.glcore.glUniformMatrix4x3fv(location, 1, 0, v.as_ptr())} else
			if let Some(v) = v.downcast_ref::<i32>()		{self.shader.glcore.glUniform1iv(location, 1, v as *const _)} else
			if let Some(v) = v.downcast_ref::<IVec2>()		{self.shader.glcore.glUniform2iv(location, 1, v.as_ptr())} else
			if let Some(v) = v.downcast_ref::<IVec3>()		{self.shader.glcore.glUniform3iv(location, 1, v.as_ptr())} else
			if let Some(v) = v.downcast_ref::<IVec4>()		{self.shader.glcore.glUniform4iv(location, 1, v.as_ptr())} else
			if let Some(v) = v.downcast_ref::<u32>()		{self.shader.glcore.glUniform1uiv(location, 1, v as *const _)} else
			if let Some(v) = v.downcast_ref::<UVec2>()		{self.shader.glcore.glUniform2uiv(location, 1, v.as_ptr())} else
			if let Some(v) = v.downcast_ref::<UVec3>()		{self.shader.glcore.glUniform3uiv(location, 1, v.as_ptr())} else
			if let Some(v) = v.downcast_ref::<UVec4>()		{self.shader.glcore.glUniform4uiv(location, 1, v.as_ptr())} else
			if let Some(v) = v.downcast_ref::<f64>()		{self.shader.glcore.glUniform1dv(location, 1, v as *const _)} else
			if let Some(v) = v.downcast_ref::<DVec2>()		{self.shader.glcore.glUniform2dv(location, 1, v.as_ptr())} else
			if let Some(v) = v.downcast_ref::<DVec3>()		{self.shader.glcore.glUniform3dv(location, 1, v.as_ptr())} else
			if let Some(v) = v.downcast_ref::<DVec4>()		{self.shader.glcore.glUniform4dv(location, 1, v.as_ptr())} else
			if let Some(v) = v.downcast_ref::<DMat2>()		{self.shader.glcore.glUniformMatrix2dv(location, 1, 0, v.as_ptr())} else
			if let Some(v) = v.downcast_ref::<DMat3>()		{self.shader.glcore.glUniformMatrix3dv(location, 1, 0, v.as_ptr())} else
			if let Some(v) = v.downcast_ref::<DMat4>()		{self.shader.glcore.glUniformMatrix4dv(location, 1, 0, v.as_ptr())} else
			if let Some(v) = v.downcast_ref::<DMat2x3>()	{self.shader.glcore.glUniformMatrix2x3dv(location, 1, 0, v.as_ptr())} else
			if let Some(v) = v.downcast_ref::<DMat2x4>()	{self.shader.glcore.glUniformMatrix2x4dv(location, 1, 0, v.as_ptr())} else
			if let Some(v) = v.downcast_ref::<DMat3x2>()	{self.shader.glcore.glUniformMatrix3x2dv(location, 1, 0, v.as_ptr())} else
			if let Some(v) = v.downcast_ref::<DMat3x4>()	{self.shader.glcore.glUniformMatrix3x4dv(location, 1, 0, v.as_ptr())} else
			if let Some(v) = v.downcast_ref::<DMat4x2>()	{self.shader.glcore.glUniformMatrix4x2dv(location, 1, 0, v.as_ptr())} else
			if let Some(v) = v.downcast_ref::<DMat4x3>()	{self.shader.glcore.glUniformMatrix4x3dv(location, 1, 0, v.as_ptr())} else
			{panic!("Unknown type of uniform value: {v:?}")}
			Ok(())
		} else {
			Err(ShaderError::UniformNotFound(name.to_owned()))
		}
	}

	/// Set shader uniform inputs by a material
	pub fn setup_material_uniforms(&self, material: &dyn Material, prefix: Option<&str>, camel_case: bool) {
		let glcore = &self.shader.glcore;
		let shader_uniforms = self.shader.get_active_uniforms().unwrap();
		let texture_names = material.get_names();
		let mut active_texture = 0u32;
		for name in texture_names.iter() {
			let mut name_mod = String::new();
			if let Some(prefix) = prefix {
				name_mod.push_str(prefix);
			}
			if camel_case {
				name_mod.push_str(&to_camel_case(name, prefix.is_some()));
			} else {
				name_mod.push_str(&name);
			}
			if let Some(_) = shader_uniforms.get(&name_mod) {
				if let Some(texture) = material.get_by_name(&name) {
					let location = self.shader.get_uniform_location(&name_mod);
					if location == -1 {
						continue;
					}
					match texture {
						MaterialComponent::Texture(texture) => {
							texture.set_active_unit(active_texture);
							let bind = texture.bind();
							glcore.glUniform1i(location, active_texture as i32);
							bind.unbind();
							active_texture += 1;
						}
						MaterialComponent::Color(color) => {
							glcore.glUniform4f(location, color.x, color.y, color.z, color.w);
						}
						MaterialComponent::Luminance(lum) => {
							glcore.glUniform1f(location, *lum);
						}
					}
				}
			}
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

impl ShaderBinary {
	pub fn new(format: u32, shader_type: ShaderType, binary: Vec<u8>) -> Self {
		Self {
			format,
			shader_type,
			binary,
		}
	}

	pub fn load_from_file(path: &Path) -> Result<Self, ShaderBinaryLoadError> {
		let config = bincode::config::standard()
			.with_little_endian()
			.with_fixed_int_encoding();
		let mut file = std::fs::File::open(path)?;
		Ok(bincode::decode_from_std_read(&mut file, config)?)
	}

	pub fn save_to_file(&self, path: &Path) -> Result<(), ShaderBinarySaveError> {
		let config = bincode::config::standard()
			.with_little_endian()
			.with_fixed_int_encoding();
		let mut file = std::fs::File::open(path)?;
		bincode::encode_into_std_write(self, &mut file, config)?;
		Ok(())
	}
}

impl ShaderInputType {
	pub fn is_float(&self) -> bool {
		matches!(self, Self::Float | Self::Vec2 | Self::Vec3 | Self::Vec4 | Self::Mat2 | Self::Mat3 | Self::Mat4 | Self::Mat2x3 | Self::Mat2x4 | Self::Mat3x2 | Self::Mat3x4 | Self::Mat4x2 | Self::Mat4x3)
	}

	pub fn is_double(&self) -> bool {
		matches!(self, Self::Double | Self::DVec2 | Self::DVec3 | Self::DVec4 | Self::DMat2 | Self::DMat3 | Self::DMat4 | Self::DMat2x3 | Self::DMat2x4 | Self::DMat3x2 | Self::DMat3x4 | Self::DMat4x2 | Self::DMat4x3)
	}

	pub fn is_integer(&self) -> bool {
		matches!(self, Self::Int | Self::IVec2 | Self::IVec3 | Self::IVec4 | Self::UInt | Self::UVec2 | Self::UVec3 | Self::UVec4)
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

	pub fn get_base_type(&self) -> ShaderInputType {
		match self {
			Self::Float | Self::Vec2 | Self::Vec3 | Self::Vec4 | Self::Mat2 | Self::Mat3 | Self::Mat4 | Self::Mat2x3 | Self::Mat2x4 | Self::Mat3x2 | Self::Mat3x4 | Self::Mat4x2 | Self::Mat4x3 => Self::Float,
			Self::Double | Self::DVec2 | Self::DVec3 | Self::DVec4 | Self::DMat2 | Self::DMat3 | Self::DMat4 | Self::DMat2x3 | Self::DMat2x4 | Self::DMat3x2 | Self::DMat3x4 | Self::DMat4x2 | Self::DMat4x3 => Self::Double,
			Self::Int | Self::IVec2 | Self::IVec3 | Self::IVec4 => Self::Int,
			Self::UInt | Self::UVec2 | Self::UVec3 | Self::UVec4 => Self::UInt,
		}
	}
}

impl ShaderInputVarType {
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

	pub fn get_type(&self) -> ShaderInputType {
		self.type_
	}

	pub fn get_base_type(&self) -> ShaderInputType {
		self.type_.get_base_type()
	}
}

impl Debug for Shader {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		f.debug_struct("Shader")
		.field("program", &self.program)
		.field("shader_type", &self.shader_type)
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
			Self::AttribNotFound(attrib) => write!(f, "Attrib not found: {attrib}"),
			Self::UniformNotFound(uniform) => write!(f, "Uniform not found: {uniform}"),
		}
	}
}

impl From<u32> for ShaderInputType {
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
			_ => panic!("Invalid value {val} of `ShaderInputType`"),
		}
	}
}

impl Debug for ShaderInputType {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match self {
			Self::Float => write!(f, "float"),
			Self::Vec2 => write!(f, "vec2"),
			Self::Vec3 => write!(f, "vec3"),
			Self::Vec4 => write!(f, "vec4"),
			Self::Mat2 => write!(f, "mat2"),
			Self::Mat3 => write!(f, "mat3"),
			Self::Mat4 => write!(f, "mat4"),
			Self::Mat2x3 => write!(f, "mat2x3"),
			Self::Mat2x4 => write!(f, "mat2x4"),
			Self::Mat3x2 => write!(f, "mat3x2"),
			Self::Mat3x4 => write!(f, "mat3x4"),
			Self::Mat4x2 => write!(f, "mat4x2"),
			Self::Mat4x3 => write!(f, "mat4x3"),
			Self::Int => write!(f, "int"),
			Self::IVec2 => write!(f, "ivec2"),
			Self::IVec3 => write!(f, "ivec3"),
			Self::IVec4 => write!(f, "ivec4"),
			Self::UInt => write!(f, "uint"),
			Self::UVec2 => write!(f, "uvec2"),
			Self::UVec3 => write!(f, "uvec3"),
			Self::UVec4 => write!(f, "uvec4"),
			Self::Double => write!(f, "double"),
			Self::DVec2 => write!(f, "dvec2"),
			Self::DVec3 => write!(f, "dvec3"),
			Self::DVec4 => write!(f, "dvec4"),
			Self::DMat2 => write!(f, "dmat2"),
			Self::DMat3 => write!(f, "dmat3"),
			Self::DMat4 => write!(f, "dmat4"),
			Self::DMat2x3 => write!(f, "dmat2x3"),
			Self::DMat2x4 => write!(f, "dmat2x4"),
			Self::DMat3x2 => write!(f, "dmat3x2"),
			Self::DMat3x4 => write!(f, "dmat3x4"),
			Self::DMat4x2 => write!(f, "dmat4x2"),
			Self::DMat4x3 => write!(f, "dmat4x3"),
		}
	}
}

impl Display for ShaderInputType {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		<Self as Debug>::fmt(self, f)
	}
}

impl From<std::io::Error> for ShaderBinaryLoadError {
	fn from(err: std::io::Error) -> Self {
		Self::IOError(err)
	}
}

impl From<bincode::error::DecodeError> for ShaderBinaryLoadError {
	fn from(err: bincode::error::DecodeError) -> Self {
		Self::DecodeError(err)
	}
}

impl From<std::io::Error> for ShaderBinarySaveError {
	fn from(err: std::io::Error) -> Self {
		Self::IOError(err)
	}
}

impl From<bincode::error::EncodeError> for ShaderBinarySaveError {
	fn from(err: bincode::error::EncodeError) -> Self {
		Self::EncodeError(err)
	}
}
