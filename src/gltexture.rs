
#![allow(dead_code)]
#![allow(clippy::too_many_arguments)]

use glcore::*;
use crate::glbuffer::*;
use crate::buffervec::*;
use std::{
	ffi::c_void,
	fmt::{self, Debug, Formatter},
};

#[derive(Clone, Copy)]
pub enum TextureDimension {
	Tex1d = GL_TEXTURE_1D as isize,
	Tex2d = GL_TEXTURE_2D as isize,
	Tex3d = GL_TEXTURE_3D as isize,
	TexCube = GL_TEXTURE_CUBE_MAP as isize,
}

#[derive(Clone, Copy)]
pub enum TextureTarget {
	Tex1d = GL_TEXTURE_1D as isize,
	Tex2d = GL_TEXTURE_2D as isize,
	Tex3d = GL_TEXTURE_3D as isize,
	TexCube = GL_TEXTURE_CUBE_MAP as isize,
	TexCubePosX = GL_TEXTURE_CUBE_MAP_POSITIVE_X as isize,
	TexCubeNegX = GL_TEXTURE_CUBE_MAP_NEGATIVE_X as isize,
	TexCubePosY = GL_TEXTURE_CUBE_MAP_POSITIVE_Y as isize,
	TexCubeNegY = GL_TEXTURE_CUBE_MAP_NEGATIVE_Y as isize,
	TexCubePosZ = GL_TEXTURE_CUBE_MAP_POSITIVE_Z as isize,
	TexCubeNegZ = GL_TEXTURE_CUBE_MAP_NEGATIVE_Z as isize,
}

#[derive(Clone, Copy)]
pub enum CubeMapFaces {
	TexCubePosX = TextureTarget::TexCubePosX as isize,
	TexCubeNegX = TextureTarget::TexCubeNegX as isize,
	TexCubePosY = TextureTarget::TexCubePosY as isize,
	TexCubeNegY = TextureTarget::TexCubeNegY as isize,
	TexCubePosZ = TextureTarget::TexCubePosZ as isize,
	TexCubeNegZ = TextureTarget::TexCubeNegZ as isize,
}

pub const CUBE_FACE_TARGETS: [CubeMapFaces; 6] = [
	CubeMapFaces::TexCubePosX,
	CubeMapFaces::TexCubeNegX,
	CubeMapFaces::TexCubePosY,
	CubeMapFaces::TexCubeNegY,
	CubeMapFaces::TexCubePosZ,
	CubeMapFaces::TexCubeNegZ,
];

#[derive(Clone, Copy)]
pub enum TextureFormat {
	Depth = GL_DEPTH_COMPONENT as isize,
	DepthStencil = GL_DEPTH_STENCIL as isize,
	Red = GL_RED as isize,
	Rg = GL_RG as isize,
	Rgb = GL_RGB as isize,
	Rgba = GL_RGBA as isize,
	Red8 = GL_R8 as isize,
	Red8Snorm = GL_R8_SNORM as isize,
	Red16 = GL_R16 as isize,
	Red16Snorm = GL_R16_SNORM as isize,
	Rg8 = GL_RG8 as isize,
	Rg8Snorm = GL_RG8_SNORM as isize,
	Rg16 = GL_RG16 as isize,
	Rg16Snorm = GL_RG16_SNORM as isize,
	R3g3b2 = GL_R3_G3_B2 as isize,
	Rgb4 = GL_RGB4 as isize,
	Rgb5 = GL_RGB5 as isize,
	Rgb8 = GL_RGB8 as isize,
	Rgb8Snorm = GL_RGB8_SNORM as isize,
	Rgb10 = GL_RGB10 as isize,
	Rgb12 = GL_RGB12 as isize,
	Rgb16Snorm = GL_RGB16_SNORM as isize,
	Rgba2 = GL_RGBA2 as isize,
	Rgba4 = GL_RGBA4 as isize,
	Rgb5a1 = GL_RGB5_A1 as isize,
	Rgba8 = GL_RGBA8 as isize,
	Rgba8Snorm = GL_RGBA8_SNORM as isize,
	Rgb10a2 = GL_RGB10_A2 as isize,
	Rgb10a2ui = GL_RGB10_A2UI as isize,
	Rgba12 = GL_RGBA12 as isize,
	Rgba16 = GL_RGBA16 as isize,
	R32f = GL_R32F as isize,
	Rg32f = GL_RG32F as isize,
	Rgb32f = GL_RGB32F as isize,
	Rgba32f = GL_RGBA32F as isize,
	R11fg11fb10f = GL_R11F_G11F_B10F as isize,
	Rgb9e5 = GL_RGB9_E5 as isize,
	R8i = GL_R8I as isize,
	R8ui = GL_R8UI as isize,
	R16i = GL_R16I as isize,
	R16ui = GL_R16UI as isize,
	R32i = GL_R32I as isize,
	R32ui = GL_R32UI as isize,
	Rg8i = GL_RG8I as isize,
	Rg8ui = GL_RG8UI as isize,
	Rg16i = GL_RG16I as isize,
	Rg16ui = GL_RG16UI as isize,
	Rg32i = GL_RG32I as isize,
	Rg32ui = GL_RG32UI as isize,
	Rgb8i = GL_RGB8I as isize,
	Rgb8ui = GL_RGB8UI as isize,
	Rgb16i = GL_RGB16I as isize,
	Rgb16ui = GL_RGB16UI as isize,
	Rgb32i = GL_RGB32I as isize,
	Rgb32ui = GL_RGB32UI as isize,
	Rgba8i = GL_RGBA8I as isize,
	Rgba8ui = GL_RGBA8UI as isize,
	Rgba16i = GL_RGBA16I as isize,
	Rgba16ui = GL_RGBA16UI as isize,
	Rgba32i = GL_RGBA32I as isize,
	Rgba32ui = GL_RGBA32UI as isize,
}

#[derive(Clone, Copy)]
pub enum TextureWrapping {
	ClampToEdge = GL_CLAMP_TO_EDGE as isize,
	ClampToBorder = GL_CLAMP_TO_BORDER as isize,
	MirrorClampToEdge = GL_MIRROR_CLAMP_TO_EDGE as isize,
	Repeat = GL_REPEAT as isize,
	MirroredRepeat = GL_MIRRORED_REPEAT as isize,
}

#[derive(Clone, Copy)]
pub enum SamplerFilter {
	Nearest = GL_NEAREST as isize,
	Linear = GL_LINEAR as isize,
	NearestMipmapNearest = GL_NEAREST_MIPMAP_NEAREST as isize,
	LinearMipmapNearest = GL_LINEAR_MIPMAP_NEAREST as isize,
	NearestMipmapLinear = GL_NEAREST_MIPMAP_LINEAR as isize,
	LinearMipmapLinear = GL_LINEAR_MIPMAP_LINEAR as isize,
}

#[derive(Clone, Copy)]
pub enum SamplerMagFilter {
	Nearest = GL_NEAREST as isize,
	Linear = GL_LINEAR as isize,
}

#[derive(Clone, Copy)]
pub enum PixelFormat {
	Red = GL_RED as isize,
	Rg = GL_RG as isize,
	Rgb = GL_RGB as isize,
	Bgr = GL_BGR as isize,
	Rgba = GL_RGBA as isize,
	Bgra = GL_BGRA as isize,
	RedInteger = GL_RED_INTEGER as isize,
	RgInteger = GL_RG_INTEGER as isize,
	RgbInteger = GL_RGB_INTEGER as isize,
	BgrInteger = GL_BGR_INTEGER as isize,
	RgbaInteger = GL_RGBA_INTEGER as isize,
	BgraInteger = GL_BGRA_INTEGER as isize,
	StencilIndex = GL_STENCIL_INDEX as isize,
	Depth = GL_DEPTH_COMPONENT as isize,
	DepthStencil = GL_DEPTH_STENCIL as isize,
}

#[derive(Clone, Copy)]
pub enum ComponentType {
	U8 = GL_UNSIGNED_BYTE as isize,
	I8 = GL_BYTE as isize,
	U16 = GL_UNSIGNED_SHORT as isize,
	I16 = GL_SHORT as isize,
	U32 = GL_UNSIGNED_INT as isize,
	I32 = GL_INT as isize,
	F16 = GL_HALF_FLOAT as isize,
	F32 = GL_FLOAT as isize,
	U8_332 = GL_UNSIGNED_BYTE_3_3_2 as isize,
	U8_233Rev = GL_UNSIGNED_BYTE_2_3_3_REV as isize,
	U16_565 = GL_UNSIGNED_SHORT_5_6_5 as isize,
	U16_565Rev = GL_UNSIGNED_SHORT_5_6_5_REV as isize,
	U16_4444 = GL_UNSIGNED_SHORT_4_4_4_4 as isize,
	U16_4444Rev = GL_UNSIGNED_SHORT_4_4_4_4_REV as isize,
	U16_5551 = GL_UNSIGNED_SHORT_5_5_5_1 as isize,
	U16_1555Rev = GL_UNSIGNED_SHORT_1_5_5_5_REV as isize,
	U32_8888 = GL_UNSIGNED_INT_8_8_8_8 as isize,
	U32_8888Rev = GL_UNSIGNED_INT_8_8_8_8_REV as isize,
	U32_10_10_10_2 = GL_UNSIGNED_INT_10_10_10_2 as isize,
	U32_2_10_10_10Rev = GL_UNSIGNED_INT_2_10_10_10_REV as isize,
}

pub trait PixelType: BufferVecItem {}
impl<T> PixelType for T where T: BufferVecItem {}

#[derive(Debug, Clone)]
pub struct PixelBuffer<'a> {
	buffer: BufferVec<'a>,
	pixel_size: usize,
	width: u32,
	height: u32,
	depth: u32,
	pitch: usize,
	pitch_wh: usize,
	format: PixelFormat,
	format_type: ComponentType,
}

pub struct Texture<'a> {
	pub glcore: &'a GLCore,
	name: u32,
	dim: TextureDimension,
	format: TextureFormat,
	width: u32,
	height: u32,
	depth: u32,
	has_mipmap: bool,
	mag_filter: SamplerMagFilter,
	min_filter: SamplerFilter,
	bytes_of_texture: usize,
	bytes_of_face: usize,
	pixel_buffer: Option<PixelBuffer<'a>>,
}

pub struct TextureBind<'a, 'b> {
	pub texture: &'b Texture<'a>,
	target: TextureTarget,
}

impl TextureFormat {
	pub fn bits_of_pixel(&self, glcore: &GLCore, target: TextureTarget) -> usize {
		let target = target as u32;
		let mut data: i32 = 0;
		let mut size: usize = 0;
		glcore.glGetTexLevelParameteriv(target, 0, GL_TEXTURE_DEPTH_SIZE, &mut data as *mut _);
		size += data as usize;
		glcore.glGetTexLevelParameteriv(target, 0, GL_TEXTURE_STENCIL_SIZE, &mut data as *mut _);
		size += data as usize;
		glcore.glGetTexLevelParameteriv(target, 0, GL_TEXTURE_RED_SIZE, &mut data as *mut _);
		size += data as usize;
		glcore.glGetTexLevelParameteriv(target, 0, GL_TEXTURE_GREEN_SIZE, &mut data as *mut _);
		size += data as usize;
		glcore.glGetTexLevelParameteriv(target, 0, GL_TEXTURE_BLUE_SIZE, &mut data as *mut _);
		size += data as usize;
		glcore.glGetTexLevelParameteriv(target, 0, GL_TEXTURE_ALPHA_SIZE, &mut data as *mut _);
		size += data as usize;
		size
	}
}

impl<'a> PixelBuffer<'a> {
	pub fn new(glcore: &'a GLCore,
			width: u32,
			height: u32,
			depth: u32,
			size_in_bytes: usize,
			format: PixelFormat,
			format_type: ComponentType
		) -> Self {
		let pixel_size = Self::size_of_pixel(format, format_type);
		let pitch = ((width as usize * pixel_size - 1) / 4 + 1) * 4;
		let pitch_wh = pitch * height as usize;
		let empty_data = vec![0u8; size_in_bytes];
		let buffer = Buffer::new(glcore, BufferTarget::PixelUnpackBuffer, size_in_bytes, BufferUsage::StreamDraw, empty_data.as_ptr() as *const c_void);
		let buffer = BufferVec::new(glcore, buffer);
		Self {
			buffer,
			pixel_size,
			width,
			height,
			depth,
			pitch,
			pitch_wh,
			format,
			format_type,
		}
	}

	/// Get the size of the buffer
	pub fn size_in_bytes(&self) -> usize {
		self.buffer.size_in_bytes()
	}

	/// Get the size for each pixel
	pub fn size_of_pixel(format: PixelFormat, format_type: ComponentType) -> usize {
		let component_len = match format_type {
			ComponentType::U8_332 |
			ComponentType::U8_233Rev => return 1,
			ComponentType::U16_565 |
			ComponentType::U16_565Rev |
			ComponentType::U16_4444 |
			ComponentType::U16_4444Rev |
			ComponentType::U16_5551 |
			ComponentType::U16_1555Rev => return 2,
			ComponentType::U32_8888 |
			ComponentType::U32_8888Rev |
			ComponentType::U32_10_10_10_2 |
			ComponentType::U32_2_10_10_10Rev => return 4,
			ComponentType::U8 |
			ComponentType::I8 => 1,
			ComponentType::U16 |
			ComponentType::I16 |
			ComponentType::F16 => 2,
			ComponentType::U32 |
			ComponentType::I32 |
			ComponentType::F32 => 4,
		};
		match format {
			PixelFormat::Red |
			PixelFormat::RedInteger |
			PixelFormat::StencilIndex |
			PixelFormat::Depth => component_len,
			PixelFormat::Rg |
			PixelFormat::RgInteger |
			PixelFormat::DepthStencil => component_len * 2,
			PixelFormat::Rgb |
			PixelFormat::RgbInteger |
			PixelFormat::Bgr |
			PixelFormat::BgrInteger => component_len * 3,
			PixelFormat::Rgba |
			PixelFormat::RgbaInteger |
			PixelFormat::Bgra |
			PixelFormat::BgraInteger => component_len * 4,
		}
	}

	/// Get the buffer
	pub fn get_buffer(&self) -> &Buffer {
		self.buffer.get_buffer()
	}

	/// Create a `BufferBind` to use the RAII system to manage the binding state.
	pub fn bind<'b>(&'a self) -> BufferBind<'a, 'b> {
		self.buffer.bind()
	}
}

impl<'a> Texture<'a> {
	fn new(glcore: &'a GLCore,
			dim: TextureDimension,
			format: TextureFormat,
			width: u32,
			mut height: u32,
			mut depth: u32,
			wrapping_s: TextureWrapping,
			wrapping_t: TextureWrapping,
			wrapping_r: TextureWrapping,
			has_mipmap: bool,
			mag_filter: SamplerMagFilter,
			min_filter: SamplerFilter,
			buffering: bool,
			buffer_format: PixelFormat,
			buffer_format_type: ComponentType,
		) -> Self {
		let mut name: u32 = 0;
		glcore.glGenTextures(1, &mut name as *mut _);
		let target;
		let size_mod;
		match dim {
			TextureDimension::Tex1d => {
				target = TextureTarget::Tex1d;
				height = 1;
				depth = 1;
				size_mod = 1;
			}
			TextureDimension::Tex2d => {
				target = TextureTarget::Tex2d;
				depth = 1;
				size_mod = 1;
			}
			TextureDimension::Tex3d => {
				target = TextureTarget::Tex3d;
				size_mod = 1;
			}
			TextureDimension::TexCube => {
				target = TextureTarget::TexCube;
				height = width;
				depth = 1;
				size_mod = 6;
			}
		}
		glcore.glBindTexture(target as u32, name);
		match dim {
			TextureDimension::Tex1d => {
				glcore.glTexParameteri(target as u32, GL_TEXTURE_WRAP_S, wrapping_s as i32);
			}
			TextureDimension::Tex2d => {
				glcore.glTexParameteri(target as u32, GL_TEXTURE_WRAP_S, wrapping_s as i32);
				glcore.glTexParameteri(target as u32, GL_TEXTURE_WRAP_T, wrapping_t as i32);
			}
			TextureDimension::Tex3d => {
				glcore.glTexParameteri(target as u32, GL_TEXTURE_WRAP_S, wrapping_s as i32);
				glcore.glTexParameteri(target as u32, GL_TEXTURE_WRAP_T, wrapping_t as i32);
				glcore.glTexParameteri(target as u32, GL_TEXTURE_WRAP_R, wrapping_r as i32);
			}
			_ => {}
		}
		glcore.glTexParameteri(target as u32, GL_TEXTURE_MAG_FILTER, mag_filter as i32);
		glcore.glTexParameteri(target as u32, GL_TEXTURE_MIN_FILTER, min_filter as i32);
		let pixel_bits = format.bits_of_pixel(glcore, target);
		let pitch = ((pixel_bits - 1) / 32 + 1) * 4;
		let bytes_of_face = pitch * height as usize * depth as usize;
		let bytes_of_texture = bytes_of_face * size_mod;
		let mut ret = Self {
			glcore,
			name,
			dim,
			format,
			width,
			height,
			depth,
			has_mipmap,
			mag_filter,
			min_filter,
			bytes_of_texture,
			bytes_of_face,
			pixel_buffer: None,
		};
		if buffering {
			ret.create_pixel_buffer(buffer_format, buffer_format_type);
		} else {
			let empty_data = vec![0u8; bytes_of_texture];
			ret.upload_texture(empty_data.as_ptr() as *const c_void, buffer_format, buffer_format_type, has_mipmap);
		}
		ret
	}

	/// Create an 1D texture
	pub fn new_1d(
	        glcore: &'a GLCore,
	        format: TextureFormat,
	        width: u32,
	        wrapping_s: TextureWrapping,
	        has_mipmap: bool,
	        mag_filter: SamplerMagFilter,
			min_filter: SamplerFilter,
			buffering: bool,
			buffer_format: PixelFormat,
			buffer_format_type: ComponentType
		) -> Self {
		Self::new(glcore, TextureDimension::Tex1d, format, width, 1, 1, wrapping_s, TextureWrapping::Repeat, TextureWrapping::Repeat, has_mipmap, mag_filter, min_filter, buffering, buffer_format, buffer_format_type)
	}

	/// Create an 2D texture
	pub fn new_2d(
	        glcore: &'a GLCore,
	        format: TextureFormat,
	        width: u32,
	        height: u32,
	        wrapping_s: TextureWrapping,
	        wrapping_t: TextureWrapping,
	        has_mipmap: bool,
	        mag_filter: SamplerMagFilter,
			min_filter: SamplerFilter,
			buffering: bool,
			buffer_format: PixelFormat,
			buffer_format_type: ComponentType
		) -> Self {
		Self::new(glcore, TextureDimension::Tex2d, format, width, height, 1, wrapping_s, wrapping_t, TextureWrapping::Repeat, has_mipmap, mag_filter, min_filter, buffering, buffer_format, buffer_format_type)
	}

	/// Create an 3D texture
	pub fn new_3d(
	        glcore: &'a GLCore,
	        format: TextureFormat,
	        width: u32,
	        height: u32,
	        depth: u32,
	        wrapping_s: TextureWrapping,
	        wrapping_t: TextureWrapping,
	        wrapping_r: TextureWrapping,
	        has_mipmap: bool,
	        mag_filter: SamplerMagFilter,
			min_filter: SamplerFilter,
			buffering: bool,
			buffer_format: PixelFormat,
			buffer_format_type: ComponentType
		) -> Self {
		Self::new(glcore, TextureDimension::Tex3d, format, width, height, depth, wrapping_s, wrapping_t, wrapping_r, has_mipmap, mag_filter, min_filter, buffering, buffer_format, buffer_format_type)
	}

	/// Create an cube map texture
	pub fn new_cube(
	        glcore: &'a GLCore,
	        format: TextureFormat,
	        size: u32,
	        has_mipmap: bool,
	        mag_filter: SamplerMagFilter,
			min_filter: SamplerFilter,
			buffering: bool,
			buffer_format: PixelFormat,
			buffer_format_type: ComponentType
		) -> Self {
		Self::new(glcore, TextureDimension::TexCube, format, size, size, 1, TextureWrapping::ClampToEdge, TextureWrapping::ClampToEdge, TextureWrapping::ClampToEdge, has_mipmap, mag_filter, min_filter, buffering, buffer_format, buffer_format_type)
	}

	/// Bind the texture, use the RAII system to manage the binding state.
	pub fn bind<'b>(&'a self) -> TextureBind<'a, 'b> {
		match self.dim {
			TextureDimension::Tex1d => TextureBind::new(self, TextureTarget::Tex1d),
			TextureDimension::Tex2d => TextureBind::new(self, TextureTarget::Tex2d),
			TextureDimension::Tex3d => TextureBind::new(self, TextureTarget::Tex3d),
			TextureDimension::TexCube => panic!("Please use `bind_face()` to bind a cube map."),
		}
	}

	/// Bind the cube map face, use the RAII system to manage the binding state.
	pub fn bind_face<'b>(&'a self, face: CubeMapFaces) -> TextureBind<'a, 'b> {
		match self.dim {
			TextureDimension::TexCube => {
				match face {
					CubeMapFaces::TexCubePosX => TextureBind::new(self, TextureTarget::TexCubePosX),
					CubeMapFaces::TexCubeNegX => TextureBind::new(self, TextureTarget::TexCubeNegX),
					CubeMapFaces::TexCubePosY => TextureBind::new(self, TextureTarget::TexCubePosY),
					CubeMapFaces::TexCubeNegY => TextureBind::new(self, TextureTarget::TexCubeNegY),
					CubeMapFaces::TexCubePosZ => TextureBind::new(self, TextureTarget::TexCubePosZ),
					CubeMapFaces::TexCubeNegZ => TextureBind::new(self, TextureTarget::TexCubeNegZ),
				}
			}
			_ => panic!("Please use `bind()` to bind an non-cube-map texture."),
		}
	}

	/// Map the pixel buffer for the specified access
	pub fn map_buffer<'b>(&'a mut self, access: MapAccess) -> Option<(BufferBind<'a, 'b>, BufferMapping<'a, 'b>, *mut c_void)> {
		self.pixel_buffer.as_ref().map(|b|{
			let bind = b.bind();
			let (mapping, address) = bind.map(access);
			(bind, mapping, address)
		})
	}

	/// Retrieve the pixels from the texture to the specified data pointer regardless if currently is using an PBO or not
	fn download_texture(&self, data: *mut c_void, buffer_format: PixelFormat, buffer_format_type: ComponentType) {
		let pointer = data as *mut u8;
		match self.dim {
			TextureDimension::Tex1d => {
				let bind_tex = self.bind();
				self.glcore.glGetTexImage(TextureTarget::Tex1d as u32, 0, buffer_format as u32, buffer_format_type as u32, pointer as *mut c_void);
				bind_tex.unbind();
			}
			TextureDimension::Tex2d => {
				let bind_tex = self.bind();
				self.glcore.glGetTexImage(TextureTarget::Tex2d as u32, 0, buffer_format as u32, buffer_format_type as u32, pointer as *mut c_void);
				bind_tex.unbind();
			}
			TextureDimension::Tex3d => {
				let bind_tex = self.bind();
				self.glcore.glGetTexImage(TextureTarget::Tex3d as u32, 0, buffer_format as u32, buffer_format_type as u32, pointer as *mut c_void);
				bind_tex.unbind();
			}
			TextureDimension::TexCube => {
				for (i, target) in CUBE_FACE_TARGETS.iter().enumerate() {
					let target = *target;
					let bind_tex = self.bind_face(target);
					let pointer = pointer.wrapping_add(i * self.bytes_of_face);
					self.glcore.glGetTexImage(target as u32, 0, buffer_format as u32, buffer_format_type as u32, pointer as *mut c_void);
					bind_tex.unbind();
				}
			}
		}
	}

	/// Load the texture with the specified data pointer regardless if currently is using an PBO or not
	fn upload_texture(&self, data: *const c_void, buffer_format: PixelFormat, buffer_format_type: ComponentType, regen_mipmap: bool) {
		let pointer = data as *const u8;
		match self.dim {
			TextureDimension::Tex1d => {
				let bind_tex = self.bind();
				self.glcore.glTexImage1D(TextureTarget::Tex1d as u32, 0, self.format as i32, self.width as i32, 0, buffer_format as u32, buffer_format_type as u32, pointer as *const c_void);
				if regen_mipmap && self.has_mipmap {
					self.glcore.glGenerateMipmap(TextureTarget::Tex1d as u32);
				}
				bind_tex.unbind();
			}
			TextureDimension::Tex2d => {
				let bind_tex = self.bind();
				self.glcore.glTexImage2D(TextureTarget::Tex2d as u32, 0, self.format as i32, self.width as i32, self.height as i32, 0, buffer_format as u32, buffer_format_type as u32, pointer as *const c_void);
				if regen_mipmap && self.has_mipmap {
					self.glcore.glGenerateMipmap(TextureTarget::Tex2d as u32);
				}
				bind_tex.unbind();
			}
			TextureDimension::Tex3d => {
				let bind_tex = self.bind();
				self.glcore.glTexImage3D(TextureTarget::Tex3d as u32, 0, self.format as i32, self.width as i32, self.height as i32, self.depth as i32, 0, buffer_format as u32, buffer_format_type as u32, pointer as *const c_void);
				if regen_mipmap && self.has_mipmap {
					self.glcore.glGenerateMipmap(TextureTarget::Tex3d as u32);
				}
				bind_tex.unbind();
			}
			TextureDimension::TexCube => {
				for (i, target) in CUBE_FACE_TARGETS.iter().enumerate() {
					let target = *target;
					let bind_tex = self.bind_face(target);
					let pointer = pointer.wrapping_add(i * self.bytes_of_face);
					self.glcore.glTexImage2D(target as u32, 0, self.format as i32, self.width as i32, self.height as i32, 0, buffer_format as u32, buffer_format_type as u32, pointer as *const c_void);
					if regen_mipmap && self.has_mipmap {
						self.glcore.glGenerateMipmap(target as u32);
					}
					bind_tex.unbind();
				}
			}
		}
	}

	/// Read the pixels from the texture to the pixel buffer
	pub fn pack_pixel_buffer(&self) {
		let pixel_buffer = self.pixel_buffer.as_ref().unwrap();
		let buffer_format = pixel_buffer.format;
		let buffer_format_type = pixel_buffer.format_type;
		let bind_pbo = pixel_buffer.bind();
		self.download_texture(std::ptr::null_mut::<c_void>(), buffer_format, buffer_format_type);
		bind_pbo.unbind();
	}

	/// Apply the change to the pixel buffer
	pub fn unpack_pixel_buffer(&self, regen_mipmap: bool) {
		let pixel_buffer = self.pixel_buffer.as_ref().unwrap();
		let buffer_format = pixel_buffer.format;
		let buffer_format_type = pixel_buffer.format_type;
		let bind_pbo = pixel_buffer.bind();
		self.upload_texture(std::ptr::null(), buffer_format, buffer_format_type, regen_mipmap);
		bind_pbo.unbind();
	}

	/// Create the PBO if not created early
	pub fn create_pixel_buffer(&mut self, buffer_format: PixelFormat, buffer_format_type: ComponentType) {
		self.pixel_buffer = Some(PixelBuffer::new(self.glcore, self.width, self.height, self.depth, self.bytes_of_texture, buffer_format, buffer_format_type))
	}

	/// Discard the PBO if not necessarily need it
	pub fn drop_pixel_buffer(&mut self) {
		self.pixel_buffer = None
	}
}

impl Drop for Texture<'_> {
	fn drop(&mut self) {
		self.glcore.glDeleteTextures(1, &self.name as *const u32);
	}
}

impl<'a, 'b> TextureBind<'a, 'b> {
	fn new(texture: &'b Texture<'a>, target: TextureTarget) -> Self {
		texture.glcore.glBindTexture(target as u32, texture.name);
		Self {
			texture,
			target,
		}
	}

	/// Explicitly unbind the texture.
	pub fn unbind(self) {}
}

impl Drop for TextureBind<'_, '_> {
	fn drop(&mut self) {
		self.texture.glcore.glBindTexture(self.target as u32, 0);
	}
}

impl Debug for TextureDimension {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match self {
			Self::Tex1d => write!(f, "1D"),
			Self::Tex2d => write!(f, "2D"),
			Self::Tex3d => write!(f, "3D"),
			Self::TexCube => write!(f, "CubeMap"),
		}
	}
}

impl Debug for Texture<'_> {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		f.debug_struct("Texture")
		.field("name", &self.name)
		.field("dim", &self.dim)
		.field("format", &self.format)
		.field("width", &self.width)
		.field("height", &self.height)
		.field("depth", &self.depth)
		.field("has_mipmap", &self.has_mipmap)
		.field("pixel_buffer", &self.pixel_buffer)
		.finish()
	}
}

impl Debug for TextureFormat {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match self {
			Self::Depth => write!(f, "DEPTH"),
			Self::DepthStencil => write!(f, "DEPTH_STENCIL"),
			Self::Red => write!(f, "RED"),
			Self::Rg => write!(f, "RG"),
			Self::Rgb => write!(f, "RGB"),
			Self::Rgba => write!(f, "RGBA"),
			Self::Red8 => write!(f, "R8"),
			Self::Red8Snorm => write!(f, "R8_SNORM"),
			Self::Red16 => write!(f, "R16"),
			Self::Red16Snorm => write!(f, "R16_SNORM"),
			Self::Rg8 => write!(f, "RG8"),
			Self::Rg8Snorm => write!(f, "RG8_SNORM"),
			Self::Rg16 => write!(f, "RG16"),
			Self::Rg16Snorm => write!(f, "RG16_SNORM"),
			Self::R3g3b2 => write!(f, "R3_G3_B2"),
			Self::Rgb4 => write!(f, "RGB4"),
			Self::Rgb5 => write!(f, "RGB5"),
			Self::Rgb8 => write!(f, "RGB8"),
			Self::Rgb8Snorm => write!(f, "RGB8_SNORM"),
			Self::Rgb10 => write!(f, "RGB10"),
			Self::Rgb12 => write!(f, "RGB12"),
			Self::Rgb16Snorm => write!(f, "RGB16_SNORM"),
			Self::Rgba2 => write!(f, "RGBA2"),
			Self::Rgba4 => write!(f, "RGBA4"),
			Self::Rgb5a1 => write!(f, "RGB5_A1"),
			Self::Rgba8 => write!(f, "RGBA8"),
			Self::Rgba8Snorm => write!(f, "RGBA8_SNORM"),
			Self::Rgb10a2 => write!(f, "RGB10_A2"),
			Self::Rgb10a2ui => write!(f, "RGB10_A2UI"),
			Self::Rgba12 => write!(f, "RGBA12"),
			Self::Rgba16 => write!(f, "RGBA16"),
			Self::R32f => write!(f, "R32F"),
			Self::Rg32f => write!(f, "RG32F"),
			Self::Rgb32f => write!(f, "RGB32F"),
			Self::Rgba32f => write!(f, "RGBA32F"),
			Self::R11fg11fb10f => write!(f, "R11F_G11F_B10F"),
			Self::Rgb9e5 => write!(f, "RGB9_E5"),
			Self::R8i => write!(f, "R8I"),
			Self::R8ui => write!(f, "R8UI"),
			Self::R16i => write!(f, "R16I"),
			Self::R16ui => write!(f, "R16UI"),
			Self::R32i => write!(f, "R32I"),
			Self::R32ui => write!(f, "R32UI"),
			Self::Rg8i => write!(f, "RG8I"),
			Self::Rg8ui => write!(f, "RG8UI"),
			Self::Rg16i => write!(f, "RG16I"),
			Self::Rg16ui => write!(f, "RG16UI"),
			Self::Rg32i => write!(f, "RG32I"),
			Self::Rg32ui => write!(f, "RG32UI"),
			Self::Rgb8i => write!(f, "RGB8I"),
			Self::Rgb8ui => write!(f, "RGB8UI"),
			Self::Rgb16i => write!(f, "RGB16I"),
			Self::Rgb16ui => write!(f, "RGB16UI"),
			Self::Rgb32i => write!(f, "RGB32I"),
			Self::Rgb32ui => write!(f, "RGB32UI"),
			Self::Rgba8i => write!(f, "RGBA8I"),
			Self::Rgba8ui => write!(f, "RGBA8UI"),
			Self::Rgba16i => write!(f, "RGBA16I"),
			Self::Rgba16ui => write!(f, "RGBA16UI"),
			Self::Rgba32i => write!(f, "RGBA32I"),
			Self::Rgba32ui => write!(f, "RGBA32UI"),
		}
	}
}

impl Debug for CubeMapFaces {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match self {
			Self::TexCubePosX => write!(f, "Positive X"),
			Self::TexCubeNegX => write!(f, "Negative X"),
			Self::TexCubePosY => write!(f, "Positive Y"),
			Self::TexCubeNegY => write!(f, "Negative Y"),
			Self::TexCubePosZ => write!(f, "Positive Z"),
			Self::TexCubeNegZ => write!(f, "Negative Z"),
		}
	}
}

impl Debug for PixelFormat {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match self {
			Self::Red => write!(f, "RED"),
			Self::Rg => write!(f, "RG"),
			Self::Rgb => write!(f, "RGB"),
			Self::Bgr => write!(f, "BGR"),
			Self::Rgba => write!(f, "RGBA"),
			Self::Bgra => write!(f, "BGRA"),
			Self::RedInteger => write!(f, "RED_INTEGER"),
			Self::RgInteger => write!(f, "RG_INTEGER"),
			Self::RgbInteger => write!(f, "RGB_INTEGER"),
			Self::BgrInteger => write!(f, "BGR_INTEGER"),
			Self::RgbaInteger => write!(f, "RGBA_INTEGER"),
			Self::BgraInteger => write!(f, "BGRA_INTEGER"),
			Self::StencilIndex => write!(f, "STENCIL_INDEX"),
			Self::Depth => write!(f, "DEPTH_COMPONENT"),
			Self::DepthStencil => write!(f, "DEPTH_STENCIL"),
		}
	}
}

impl Debug for ComponentType {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match self {
			Self::U8 => write!(f, "UNSIGNED_BYTE"),
			Self::I8 => write!(f, "BYTE"),
			Self::U16 => write!(f, "UNSIGNED_SHORT"),
			Self::I16 => write!(f, "SHORT"),
			Self::U32 => write!(f, "UNSIGNED_INT"),
			Self::I32 => write!(f, "INT"),
			Self::F16 => write!(f, "HALF_FLOAT"),
			Self::F32 => write!(f, "FLOAT"),
			Self::U8_332 => write!(f, "UNSIGNED_BYTE_3_3_2"),
			Self::U8_233Rev => write!(f, "UNSIGNED_BYTE_2_3_3_REV"),
			Self::U16_565 => write!(f, "UNSIGNED_SHORT_5_6_5"),
			Self::U16_565Rev => write!(f, "UNSIGNED_SHORT_5_6_5_REV"),
			Self::U16_4444 => write!(f, "UNSIGNED_SHORT_4_4_4_4"),
			Self::U16_4444Rev => write!(f, "UNSIGNED_SHORT_4_4_4_4_REV"),
			Self::U16_5551 => write!(f, "UNSIGNED_SHORT_5_5_5_1"),
			Self::U16_1555Rev => write!(f, "UNSIGNED_SHORT_1_5_5_5_REV"),
			Self::U32_8888 => write!(f, "UNSIGNED_INT_8_8_8_8"),
			Self::U32_8888Rev => write!(f, "UNSIGNED_INT_8_8_8_8_REV"),
			Self::U32_10_10_10_2 => write!(f, "UNSIGNED_INT_10_10_10_2"),
			Self::U32_2_10_10_10Rev => write!(f, "UNSIGNED_INT_2_10_10_10_REV"),
		}
	}
}
