
use crate::prelude::*;
use std::{
	cmp::min,
	ffi::c_void,
	fmt::{self, Debug, Formatter},
	mem::size_of,
	rc::Rc,
};

/// The OpenGL buffer binding targets
#[derive(Clone, Copy, PartialEq)]
pub enum BufferTarget {
	ArrayBuffer = GL_ARRAY_BUFFER as isize,
	AtomicCounterBuffer = GL_ATOMIC_COUNTER_BUFFER as isize,
	CopyReadBuffer = GL_COPY_READ_BUFFER as isize,
	CopyWriteBuffer = GL_COPY_WRITE_BUFFER as isize,
	DispatchIndirectBuffer = GL_DISPATCH_INDIRECT_BUFFER as isize,
	DrawIndirectBuffer = GL_DRAW_INDIRECT_BUFFER as isize,
	ElementArrayBuffer = GL_ELEMENT_ARRAY_BUFFER as isize,
	PixelPackBuffer = GL_PIXEL_PACK_BUFFER as isize,
	PixelUnpackBuffer = GL_PIXEL_UNPACK_BUFFER as isize,
	QueryBuffer = GL_QUERY_BUFFER as isize,
	ShaderStorageBuffer = GL_SHADER_STORAGE_BUFFER as isize,
	TextureBuffer = GL_TEXTURE_BUFFER as isize,
	TransformFeedbackBuffer = GL_TRANSFORM_FEEDBACK_BUFFER as isize,
	UniformBuffer = GL_UNIFORM_BUFFER as isize,
}

/// The usage for the buffer
#[derive(Clone, Copy, PartialEq)]
pub enum BufferUsage {
	StreamDraw = GL_STREAM_DRAW as isize,
	StreamRead = GL_STREAM_READ as isize,
	StreamCopy = GL_STREAM_COPY as isize,
	StaticDraw = GL_STATIC_DRAW as isize,
	StaticRead = GL_STATIC_READ as isize,
	StaticCopy = GL_STATIC_COPY as isize,
	DynamicDraw = GL_DYNAMIC_DRAW as isize,
	DynamicRead = GL_DYNAMIC_READ as isize,
	DynamicCopy = GL_DYNAMIC_COPY as isize,
}

/// The access flags for `glMapBuffers()`
#[derive(Clone, Copy, PartialEq)]
pub enum MapAccess {
	ReadOnly = GL_READ_ONLY as isize,
	WriteOnly = GL_WRITE_ONLY as isize,
	ReadWrite = GL_READ_WRITE as isize,
}

/// The OpenGL buffer object
pub struct Buffer {
	pub glcore: Rc<GLCore>,
	name: u32,
	usage: BufferUsage,
	target: BufferTarget,
	size: usize,
}

/// When to use a buffer, must bind the buffer first. The RAII system could help automatically unbind the buffer.
#[derive(Debug)]
pub struct BufferBind<'a> {
	pub buffer: &'a Buffer,
	target: BufferTarget,
}

/// When to modify the buffer or retrieve the data from the buffer, use map to update the buffer.
#[derive(Debug)]
pub struct BufferMapping<'a> {
	pub buffer: &'a Buffer,
	target: BufferTarget,
	access: MapAccess,
	address: *mut c_void,
}

impl Buffer {
	/// Get the internal name
	pub fn get_name(&self) -> u32 {
		self.name
	}

	/// Release the internal name
	pub unsafe fn to_raw(mut self) -> u32 {
		let ret = self.name;
		self.name = 0;
		ret
	}

	/// From an internal name
	pub unsafe fn from_raw(glcore: Rc<GLCore>, name: u32, target: BufferTarget) -> Result<Self, GLCoreError> {
		glcore.glBindBuffer(target as u32, name)?;
		let mut size = 0;
		let mut usage = 0;
		glcore.glGetBufferParameteriv(target as u32, GL_BUFFER_SIZE, &mut size as *mut _)?;
		glcore.glGetBufferParameteriv(target as u32, GL_BUFFER_USAGE, &mut usage as *mut _)?;
		glcore.glBindBuffer(target as u32, 0)?;
		let usage = match usage as u32 {
			GL_STREAM_DRAW  => BufferUsage::StreamDraw,
			GL_STREAM_READ  => BufferUsage::StreamRead,
			GL_STREAM_COPY  => BufferUsage::StreamCopy,
			GL_STATIC_DRAW  => BufferUsage::StaticDraw,
			GL_STATIC_READ  => BufferUsage::StaticRead,
			GL_STATIC_COPY  => BufferUsage::StaticCopy,
			GL_DYNAMIC_DRAW => BufferUsage::DynamicDraw,
			GL_DYNAMIC_READ => BufferUsage::DynamicRead,
			GL_DYNAMIC_COPY => BufferUsage::DynamicCopy,
			_ => panic!("Unknown buffer usage: `{usage}`"),
		};
		Ok(Self {
			glcore,
			name,
			usage,
			target,
			size: size as usize,
		})
	}

	/// Create a new OpenGL buffer with the specified size and data. The data could be `NULL`, indicating no initialization to the buffer.
	pub fn new(glcore: Rc<GLCore>, target: BufferTarget, size: usize, usage: BufferUsage, data_ptr: *const c_void) -> Result<Self, GLCoreError> {
		let mut name: u32 = 0;
		glcore.glGenBuffers(1, &mut name as *mut u32)?;
		glcore.glBindBuffer(target as u32, name)?;
		glcore.glBufferData(target as u32, size, data_ptr, usage as u32)?;
		glcore.glBindBuffer(target as u32, 0)?;
		Ok(Self {
			glcore,
			name,
			usage,
			target,
			size,
		})
	}

	/// Get the size of the buffer in bytes
	pub fn size(&self) -> usize {
		self.size
	}

	/// Get the default binding target
	pub fn get_target(&self) -> BufferTarget {
		self.target
	}

	/// Get the usage when initializing
	pub fn get_usage(&self) -> BufferUsage {
		self.usage
	}

	/// Resize the buffer. Actually, this operation will reallocate the buffer and copy the data.
	pub fn resize<T: Copy + Sized>(&mut self, new_len: usize, value: T) -> Result<(), GLCoreError> {
		let new_len = min(self.size, new_len);
		let data = vec![value; new_len / size_of::<T>()];
		let mut name: u32 = 0;
		self.glcore.glGenBuffers(1, &mut name as *mut u32)?;
		self.glcore.glBindBuffer(BufferTarget::CopyReadBuffer as u32, self.name)?;
		self.glcore.glBindBuffer(BufferTarget::CopyWriteBuffer as u32, name)?;
		self.glcore.glBufferData(BufferTarget::CopyWriteBuffer as u32, new_len,
			if new_len > self.size {
				data.as_ptr() as *const c_void
			} else {
				std::ptr::null()
			},
			self.usage as u32)?;
		self.glcore.glCopyBufferSubData(BufferTarget::CopyReadBuffer as u32, BufferTarget::CopyWriteBuffer as u32, 0, 0, new_len)?;
		self.glcore.glBindBuffer(BufferTarget::CopyReadBuffer as u32, 0)?;
		self.glcore.glBindBuffer(BufferTarget::CopyWriteBuffer as u32, 0)?;
		self.glcore.glDeleteBuffers(1, &self.name as *const u32)?;
		self.name = name;
		Ok(())
	}

	/// Set the default binding target
	pub fn set_target(&mut self, target: BufferTarget) {
		self.target = target;
	}

	/// Create a `BufferBind` to use the RAII system to manage the binding state.
	pub fn bind<'a>(&'a self) -> Result<BufferBind<'a>, GLCoreError> {
		BufferBind::new(self, self.target)
	}

	/// Bind to a specific target. WILL NOT change the default target of the buffer. Create a `BufferBind` to use the RAII system to manage the binding state.
	pub fn bind_to<'a>(&'a self, target: BufferTarget) -> Result<BufferBind<'a>, GLCoreError> {
		BufferBind::new(self, target)
	}
}

impl Drop for Buffer {
	/// Delete the OpenGL buffer on `drop()` called.
	fn drop(&mut self) {
		if self.name != 0 {
			self.glcore.glDeleteBuffers(1, &self.name as *const u32).unwrap();
		}
	}
}

impl Clone for Buffer {
	fn clone(&self) -> Self {
		let mut name: u32 = 0;
		self.glcore.glGenBuffers(1, &mut name as *mut u32).unwrap();
		self.glcore.glBindBuffer(self.target as u32, name).unwrap();
		self.glcore.glBindBuffer(BufferTarget::CopyReadBuffer as u32, self.name).unwrap();
		self.glcore.glBufferData(BufferTarget::CopyWriteBuffer as u32, self.size, std::ptr::null(), self.usage as u32).unwrap();
		self.glcore.glCopyBufferSubData(BufferTarget::CopyReadBuffer as u32, self.target as u32, 0, 0, self.size).unwrap();
		self.glcore.glBindBuffer(self.target as u32, 0).unwrap();
		self.glcore.glBindBuffer(BufferTarget::CopyReadBuffer as u32, 0).unwrap();
		Self {
			glcore: self.glcore.clone(),
			name,
			usage: self.usage,
			target: self.target,
			size: self.size,
		}
	}
}

impl Debug for Buffer {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		f.debug_struct("Buffer")
		.field("name", &self.name)
		.field("usage", &self.usage)
		.field("target", &self.target)
		.field("size", &self.size)
		.finish()
	}
}

impl<'a> BufferBind<'a> {
	/// Bind the buffer to the target
	fn new(buffer: &'a Buffer, target: BufferTarget) -> Result<Self, GLCoreError> {
		buffer.glcore.glBindBuffer(target as u32, buffer.name)?;
		Ok(Self {
			buffer,
			target,
		})
	}

	/// Unbind the buffer
	pub fn unbind(self) {} // Unbind by owning it in the function and `drop()`

	/// Create a `BufferMapping` to use the RAII system to manage the mapping state.
	pub fn map(&self, access: MapAccess) -> Result<(BufferMapping<'a>, *mut c_void), GLCoreError> {
		BufferMapping::new(self.buffer, self.target, access)
	}

	/// Create a `BufferMapping` to use the RAII system to manage the mapping state, with partially mapped range.
	pub fn map_ranged(&self, offset: usize, length: usize, access: MapAccess) -> Result<(BufferMapping<'a>, *mut c_void), GLCoreError> {
		BufferMapping::new_ranged(self.buffer, self.target, offset, length, access)
	}

	/// Get the binding target
	pub fn get_target(&self) -> BufferTarget {
		self.target
	}
}

impl<'a> Drop for BufferBind<'a> {
	/// Unbind if dropped
	fn drop(&mut self) {
		self.buffer.glcore.glBindBuffer(self.target as u32, 0).unwrap();
	}
}

impl<'a> BufferMapping<'a> {
	/// Map to the buffer to modify or retrieve the data of the buffer
	fn new(buffer: &'a Buffer, target: BufferTarget, access: MapAccess) -> Result<(Self, *mut c_void), GLCoreError> {
		let address = buffer.glcore.glMapBuffer(target as u32, access as u32)?;
		Ok((Self {
			buffer,
			target,
			access,
			address,
		}, address))
	}

	/// Map to the buffer partially to modify or retrieve the data of the buffer
	fn new_ranged(buffer: &'a Buffer, target: BufferTarget, offset: usize, length: usize, access: MapAccess) -> Result<(Self, *mut c_void), GLCoreError> {
		let address = buffer.glcore.glMapBufferRange(target as u32, offset, length, access as u32)?;
		Ok((Self {
			buffer,
			target,
			access,
			address,
		}, address))
	}

	/// Unmap the buffer
	pub fn unmap(self) {} // Unmap by owning it in the function and `drop()`

	/// Get the mapped target
	pub fn get_target(&self) -> BufferTarget {
		self.target
	}

	/// Get the mapped access
	pub fn get_access(&self) -> MapAccess {
		self.access
	}

	/// Get the mapping address
	pub fn get_mapping_address(&self) -> *mut c_void {
		self.address
	}
}

impl<'a> Drop for BufferMapping<'a> {
	/// Unmap the buffer when dropped
	fn drop(&mut self) {
		self.buffer.glcore.glUnmapBuffer(self.target as u32).unwrap();
	}
}

impl Debug for BufferTarget {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match self {
			Self::ArrayBuffer => write!(f, "ArrayBuffer"),
			Self::AtomicCounterBuffer => write!(f, "AtomicCounterBuffer"),
			Self::CopyReadBuffer => write!(f, "CopyReadBuffer"),
			Self::CopyWriteBuffer => write!(f, "CopyWriteBuffer"),
			Self::DispatchIndirectBuffer => write!(f, "DispatchIndirectBuffer"),
			Self::DrawIndirectBuffer => write!(f, "DrawIndirectBuffer"),
			Self::ElementArrayBuffer => write!(f, "ElementArrayBuffer"),
			Self::PixelPackBuffer => write!(f, "PixelPackBuffer"),
			Self::PixelUnpackBuffer => write!(f, "PixelUnpackBuffer"),
			Self::QueryBuffer => write!(f, "QueryBuffer"),
			Self::ShaderStorageBuffer => write!(f, "ShaderStorageBuffer"),
			Self::TextureBuffer => write!(f, "TextureBuffer"),
			Self::TransformFeedbackBuffer => write!(f, "TransformFeedbackBuffer"),
			Self::UniformBuffer => write!(f, "UniformBuffer"),
		}
	}
}

impl Debug for BufferUsage {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match self {
			Self::StreamDraw => write!(f, "StreamDraw"),
			Self::StreamRead => write!(f, "StreamRead"),
			Self::StreamCopy => write!(f, "StreamCopy"),
			Self::StaticDraw => write!(f, "StaticDraw"),
			Self::StaticRead => write!(f, "StaticRead"),
			Self::StaticCopy => write!(f, "StaticCopy"),
			Self::DynamicDraw => write!(f, "DynamicDraw"),
			Self::DynamicRead => write!(f, "DynamicRead"),
			Self::DynamicCopy => write!(f, "DynamicCopy"),
		}
	}
}

impl Debug for MapAccess {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match self {
			Self::ReadOnly => write!(f, "StreamDraw"),
			Self::WriteOnly => write!(f, "StreamRead"),
			Self::ReadWrite => write!(f, "StreamCopy"),
		}
	}
}
