#![allow(dead_code)]

use glcore::*;
use std::{
	cmp::min,
	ffi::c_void,
	fmt::{self, Debug, Formatter},
	mem::size_of,
};

/// The OpenGL buffer binding targets
#[derive(Clone, Copy)]
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
#[derive(Clone, Copy)]
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
#[derive(Clone, Copy)]
pub enum MapAccess {
	ReadOnly = GL_READ_ONLY as isize,
	WriteOnly = GL_WRITE_ONLY as isize,
	ReadWrite = GL_READ_WRITE as isize,
}

/// The OpenGL buffer object
pub struct Buffer<'a> {
	pub glcore: &'a GLCore,
	name: u32,
	usage: BufferUsage,
	target: BufferTarget,
	size: usize,
}

/// When to use a buffer, must bind the buffer first. The RAII system could help automatically unbind the buffer.
#[derive(Debug)]
pub struct BufferBind<'a, 'b> {
	pub buffer: &'b Buffer<'a>,
	target: BufferTarget,
}

/// When to modify the buffer or retrieve the data from the buffer, use map to update the buffer.
#[derive(Debug)]
pub struct BufferMapping<'a, 'b> {
	pub buffer: &'b Buffer<'a>,
	target: BufferTarget,
	access: MapAccess,
	address: *mut c_void,
}

impl<'a> Buffer<'a> {
	/// Create a new OpenGL buffer with the specified size and data. The data could be `NULL`, indicating no initialization to the buffer.
	pub fn new(glcore: &'a GLCore, target: BufferTarget, size: usize, usage: BufferUsage, data_ptr: *const c_void) -> Self {
		let mut name: u32 = 0;
		glcore.glGenBuffers(1, &mut name as *mut u32);
		glcore.glBindBuffer(target as u32, name);
		glcore.glBufferData(target as u32, size, data_ptr, usage as u32);
		glcore.glBindBuffer(target as u32, 0);
		Self {
			glcore,
			name,
			usage,
			target,
			size,
		}
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
	pub fn resize<T: Copy + Sized>(&'a mut self, new_len: usize, value: T) {
		let new_len = min(self.size, new_len);
		let data = vec![value; new_len / size_of::<T>()];
		let mut name: u32 = 0;
		self.glcore.glGenBuffers(1, &mut name as *mut u32);
		self.glcore.glBindBuffer(BufferTarget::CopyReadBuffer as u32, self.name);
		self.glcore.glBindBuffer(BufferTarget::CopyWriteBuffer as u32, name);
		self.glcore.glBufferData(BufferTarget::CopyWriteBuffer as u32, new_len,
			if new_len > self.size {
				data.as_ptr() as *const c_void
			} else {
				std::ptr::null()
			},
			self.usage as u32);
		self.glcore.glCopyBufferSubData(BufferTarget::CopyReadBuffer as u32, BufferTarget::CopyWriteBuffer as u32, 0, 0, new_len);
		self.glcore.glBindBuffer(BufferTarget::CopyReadBuffer as u32, 0);
		self.glcore.glBindBuffer(BufferTarget::CopyWriteBuffer as u32, 0);
		self.glcore.glDeleteBuffers(1, &self.name as *const u32);
		self.name = name;
	}

	/// Set the default binding target
	pub fn set_target(&mut self, target: BufferTarget) {
		self.target = target;
	}

	/// Create a `BufferBind` to use the RAII system to manage the binding state.
	pub fn bind<'b>(&'a self) -> BufferBind<'a, 'b> {
		BufferBind::new(&*self, self.target)
	}

	/// Create a `BufferBind` to use the RAII system to manage the binding state.
	pub fn bind_to<'b>(&'a mut self, target: BufferTarget) -> BufferBind<'a, 'b> {
		self.target = target;
		BufferBind::new(&*self, target)
	}
}

impl<'a> Drop for Buffer<'a> {
	/// Delete the OpenGL buffer on `drop()` called.
	fn drop(&mut self) {
		self.glcore.glDeleteBuffers(1, &self.name as *const u32);
	}
}

impl<'a> Clone for Buffer<'a> {
	fn clone(&self) -> Self {
		let mut name: u32 = 0;
		self.glcore.glGenBuffers(1, &mut name as *mut u32);
		self.glcore.glBindBuffer(self.target as u32, name);
		self.glcore.glBindBuffer(BufferTarget::CopyReadBuffer as u32, self.name);
		self.glcore.glBufferData(BufferTarget::CopyWriteBuffer as u32, self.size, std::ptr::null(), self.usage as u32);
		self.glcore.glCopyBufferSubData(BufferTarget::CopyReadBuffer as u32, self.target as u32, 0, 0, self.size);
		self.glcore.glBindBuffer(self.target as u32, 0);
		self.glcore.glBindBuffer(BufferTarget::CopyReadBuffer as u32, 0);
		Self {
			glcore: self.glcore,
			name,
			usage: self.usage,
			target: self.target,
			size: self.size,
		}
	}
}

impl<'a> Debug for Buffer<'a> {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		f.debug_struct("Buffer")
		.field("name", &self.name)
		.field("usage", &self.usage)
		.field("target", &self.target)
		.field("size", &self.size)
		.finish()
	}
}

impl<'a, 'b> BufferBind<'a, 'b> {
	/// Bind the buffer to the target
	fn new(buffer: &'b Buffer<'a>, target: BufferTarget) -> Self {
		buffer.glcore.glBindBuffer(target as u32, buffer.name);
		Self {
			buffer,
			target,
		}
	}

	/// Unbind the buffer
	pub fn unbind(self) {} // Unbind by owning it in the function and `drop()`

	/// Create a `BufferMapping` to use the RAII system to manage the mapping state.
	pub fn map(&self, access: MapAccess) -> (BufferMapping<'a, 'b>, *mut c_void) {
		BufferMapping::new(&self.buffer, self.target, access)
	}

	/// Create a `BufferMapping` to use the RAII system to manage the mapping state, with partially mapped range.
	pub fn map_ranged(&self, offset: usize, length: usize, access: MapAccess) -> (BufferMapping<'a, 'b>, *mut c_void) {
		BufferMapping::new_ranged(&self.buffer, self.target, offset, length, access)
	}

	/// Get the binding target
	pub fn get_target(&self) -> BufferTarget {
		self.target
	}
}

impl<'a, 'b> Drop for BufferBind<'a, 'b> {
	/// Unbind if dropped
	fn drop(&mut self) {
		self.buffer.glcore.glBindBuffer(self.target as u32, 0);
	}
}

impl<'a, 'b> BufferMapping<'a, 'b> {
	/// Map to the buffer to modify or retrieve the data of the buffer
	fn new(buffer: &'b Buffer<'a>, target: BufferTarget, access: MapAccess) -> (Self, *mut c_void) {
		let address = buffer.glcore.glMapBuffer(target as u32, access as u32);
		(Self {
			buffer,
			target,
			access,
			address,
		}, address)
	}

	/// Map to the buffer partially to modify or retrieve the data of the buffer
	fn new_ranged(buffer: &'b Buffer<'a>, target: BufferTarget, offset: usize, length: usize, access: MapAccess) -> (Self, *mut c_void) {
		let address = buffer.glcore.glMapBufferRange(target as u32, offset, length, access as u32);
		(Self {
			buffer,
			target,
			access,
			address,
		}, address)
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

impl<'a, 'b> Drop for BufferMapping<'a, 'b> {
	/// Unmap the buffer when dropped
	fn drop(&mut self) {
		self.buffer.glcore.glUnmapBuffer(self.target as u32);
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



