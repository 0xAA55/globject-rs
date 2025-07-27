#![allow(dead_code)]

use glcore::*;

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone, Copy)]
pub enum MapAccess {
	ReadOnly = GL_READ_ONLY as isize,
	WriteOnly = GL_WRITE_ONLY as isize,
	ReadWrite = GL_READ_WRITE as isize,
}

pub struct Buffer<'a> {
	glcore: &'a GLCore,
	name: u32,
}

pub struct BufferBind<'a, 'b> {
	buffer: &'b Buffer<'a>,
	target: BufferTarget,
}

pub struct BufferMap<'a, 'b> {
	buffer: &'b Buffer<'a>,
	target: BufferTarget,
}

impl<'a> Buffer<'a> {
	pub fn new(glcore: &'a GLCore) -> Self {
		let mut name: u32 = 0;
		glcore.glGenBuffers(1, &mut name as *mut u32);
		Self {
			glcore,
			name,
		}
	}

	pub fn bind<'b>(&'a self, target: BufferTarget) -> BufferBind<'a, 'b> {
		BufferBind::new(&self, target)
	}

	fn drop(&self) {
		self.glcore.glDeleteBuffers(1, &self.name as *const u32);
	}
}

impl<'a, 'b> BufferBind<'a, 'b> {
	fn new(buffer: &'b Buffer<'a>, target: BufferTarget) -> Self {
		buffer.glcore.glBindBuffer(target as u32, buffer.name);
		Self {
			buffer,
			target,
		}
	}

	fn drop(&self) {
		self.buffer.glcore.glBindBuffer(self.target as u32, 0);
	}

	pub fn map(&self, access: MapAccess) -> BufferMap<'a, 'b> {
		BufferMap::new(&self.buffer, self.target, access)
	}
}

impl<'a, 'b> BufferMap<'a, 'b> {
	fn new(buffer: &'b Buffer<'a>, target: BufferTarget, access: MapAccess) -> Self {
		buffer.glcore.glMapBuffer(target as u32, access as u32);
		Self {
			buffer,
			target,
		}
	}

	fn new_ranged(buffer: &'b Buffer<'a>, target: BufferTarget, offset: usize, length: usize, access: MapAccess) -> Self {
		buffer.glcore.glMapBufferRange(target as u32, offset, length, access as u32);
		Self {
			buffer,
			target,
		}
	}

	fn drop(&self) {
		self.buffer.glcore.glUnmapBuffer(self.target as u32);
	}
}
