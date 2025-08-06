
#![allow(clippy::too_many_arguments)]

use crate::prelude::*;
use std::{
	any::type_name,
	ffi::{OsStr, c_void},
	fmt::{self, Debug, Formatter},
	mem::size_of_val,
	path::Path,
	ptr::null,
	rc::Rc,
};
use image::{ImageReader, Pixel, ImageBuffer, RgbImage, DynamicImage};

/// The dimension of the texture represents the type of texture
#[derive(Clone, Copy, PartialEq)]
pub enum TextureDimension {
	Tex1d = GL_TEXTURE_1D as isize,
	Tex2d = GL_TEXTURE_2D as isize,
	Tex3d = GL_TEXTURE_3D as isize,
	TexCube = GL_TEXTURE_CUBE_MAP as isize,
}

/// The binding target of the texture includes the 6 faces of a cubemap
#[derive(Clone, Copy, PartialEq)]
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

/// The cubemap faces enum
#[derive(Clone, Copy, PartialEq)]
pub enum CubeMapFaces {
	TexCubePosX = TextureTarget::TexCubePosX as isize,
	TexCubeNegX = TextureTarget::TexCubeNegX as isize,
	TexCubePosY = TextureTarget::TexCubePosY as isize,
	TexCubeNegY = TextureTarget::TexCubeNegY as isize,
	TexCubePosZ = TextureTarget::TexCubePosZ as isize,
	TexCubeNegZ = TextureTarget::TexCubeNegZ as isize,
}

/// The constant helps to conveniently iterate through the 6 faces of a cubemap
pub const CUBE_FACE_TARGETS: [CubeMapFaces; 6] = [
	CubeMapFaces::TexCubePosX,
	CubeMapFaces::TexCubeNegX,
	CubeMapFaces::TexCubePosY,
	CubeMapFaces::TexCubeNegY,
	CubeMapFaces::TexCubePosZ,
	CubeMapFaces::TexCubeNegZ,
];

/// The **internal format** of the texture indicates how the pixels are stored in the GPU texture
#[derive(Clone, Copy, PartialEq)]
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

/// The wrapping rules of the textures
#[derive(Clone, Copy, PartialEq)]
pub enum TextureWrapping {
	ClampToEdge = GL_CLAMP_TO_EDGE as isize,
	ClampToBorder = GL_CLAMP_TO_BORDER as isize,
	MirrorClampToEdge = GL_MIRROR_CLAMP_TO_EDGE as isize,

	/// **NOTE**: `Repeat` is only supported to the 2^N size of the textures by most of the GPU
	Repeat = GL_REPEAT as isize,

	/// **NOTE**: `MirroredRepeat` is only supported to the 2^N size of the textures by most of the GPU
	MirroredRepeat = GL_MIRRORED_REPEAT as isize,
}

/// The sampler filters of the textures, including how mipmap sampling should be done
#[derive(Clone, Copy, PartialEq)]
pub enum SamplerFilter {
	Nearest = GL_NEAREST as isize,
	Linear = GL_LINEAR as isize,
	NearestMipmapNearest = GL_NEAREST_MIPMAP_NEAREST as isize,
	LinearMipmapNearest = GL_LINEAR_MIPMAP_NEAREST as isize,
	NearestMipmapLinear = GL_NEAREST_MIPMAP_LINEAR as isize,
	LinearMipmapLinear = GL_LINEAR_MIPMAP_LINEAR as isize,
}

/// The sampler filters of the textures, only for magnifying sampling
#[derive(Clone, Copy, PartialEq)]
pub enum SamplerMagFilter {
	Nearest = GL_NEAREST as isize,
	Linear = GL_LINEAR as isize,
}

/// The channel type of a pixel
#[derive(Clone, Copy, PartialEq)]
pub enum ChannelType {
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

/// The component type for each channel of a pixel
#[derive(Clone, Copy, PartialEq)]
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

/// The pixel type trait must be able to be a `BufferVec` item
pub trait PixelType: BufferVecItem {}
impl<T> PixelType for T where T: BufferVecItem {}

/// The pixel buffer object (PBO) for the texture helps with asynchronous texture updating or retrieving back to the system memory
#[derive(Debug, Clone)]
pub struct PixelBuffer {
	buffer: BufferVec,
	pixel_size: usize,
	width: u32,
	height: u32,
	depth: u32,
	pitch: usize,
	pitch_wh: usize,
	format: ChannelType,
	format_type: ComponentType,
}

/// The OpenGL texture object
pub struct Texture {
	pub glcore: Rc<GLCore>,
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
	pixel_buffer: Option<PixelBuffer>,
}

/// The binding state of the texture, utilizing the RAII rules to manage the binding state
pub struct TextureBind<'a> {
	pub texture: &'a Texture,
	target: TextureTarget,
}

/// The error for loading an image from a file, decoding the byte stream of the image
#[derive(Debug)]
pub enum LoadImageError {
	IOError(std::io::Error),
	TurboJpegError(turbojpeg::Error),
	ImageError(image::ImageError),
	UnsupportedImageType(String),
}

impl From<std::io::Error> for LoadImageError {
	fn from(err: std::io::Error) -> Self {
		Self::IOError(err)
	}
}

impl From<turbojpeg::Error> for LoadImageError {
	fn from(err: turbojpeg::Error) -> Self {
		Self::TurboJpegError(err)
	}
}

impl From<image::ImageError> for LoadImageError {
	fn from(err: image::ImageError) -> Self {
		Self::ImageError(err)
	}
}

impl TextureFormat {
	/// Get how many bits that composed of a pixel. The implementation is just to ask anything from OpenGL
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

	pub fn from_format_and_type(format: PixelFormat, format_type: ComponentType) -> Option<Self> {
		match format_type {
	/// Create a `TextureFormat` from the channel type and the component type, returns `None` if the combination couldn't have its corresponding format
			ComponentType::U8_332 => Some(Self::R3g3b2),
			ComponentType::U16_4444 => Some(Self::Rgba4),
			ComponentType::U16_5551 => Some(Self::Rgb5a1),
			ComponentType::U32_8888 => Some(Self::Rgba8),
			ComponentType::U32_10_10_10_2 => Some(Self::Rgb10a2),
			ComponentType::I8 => match format {
				ChannelType::Red =>  Some(Self::R8i),
				ChannelType::Rg =>   Some(Self::Rg8i),
				ChannelType::Rgb =>  Some(Self::Rgb8i),
				ChannelType::Rgba => Some(Self::Rgba8i),
				_ => None,
			}
			ComponentType::U8 => match format {
				ChannelType::Red =>  Some(Self::R8ui),
				ChannelType::Rg =>   Some(Self::Rg8ui),
				ChannelType::Rgb =>  Some(Self::Rgb8ui),
				ChannelType::Rgba => Some(Self::Rgba8ui),
				_ => None,
			}
			ComponentType::I16 => match format {
				ChannelType::Red =>  Some(Self::R16i),
				ChannelType::Rg =>   Some(Self::Rg16i),
				ChannelType::Rgb =>  Some(Self::Rgb16i),
				ChannelType::Rgba => Some(Self::Rgba16i),
				_ => None,
			}
			ComponentType::U16 => match format {
				ChannelType::Red =>  Some(Self::R16ui),
				ChannelType::Rg =>   Some(Self::Rg16ui),
				ChannelType::Rgb =>  Some(Self::Rgb16ui),
				ChannelType::Rgba => Some(Self::Rgba16ui),
				_ => None,
			}
			ComponentType::I32 => match format {
				ChannelType::Red =>  Some(Self::R32i),
				ChannelType::Rg =>   Some(Self::Rg32i),
				ChannelType::Rgb =>  Some(Self::Rgb32i),
				ChannelType::Rgba => Some(Self::Rgba32i),
				_ => None,
			}
			ComponentType::U32 => match format {
				ChannelType::Red =>  Some(Self::R32ui),
				ChannelType::Rg =>   Some(Self::Rg32ui),
				ChannelType::Rgb =>  Some(Self::Rgb32ui),
				ChannelType::Rgba => Some(Self::Rgba32ui),
				_ => None,
			}
			ComponentType::F32 => match format {
				ChannelType::Red =>  Some(Self::R32f),
				ChannelType::Rg =>   Some(Self::Rg32f),
				ChannelType::Rgb =>  Some(Self::Rgb32f),
				ChannelType::Rgba => Some(Self::Rgba32f),
				_ => None,
			}
			_ => None
		}
	}
}

impl ComponentType {
	/// Get the component type from a string
	pub fn from_typename(typename: &str) -> Self {
		match typename {
			"u8"  => Self::U8,
			"u16" => Self::U16,
			"u32" => Self::U32,
			"i8"  => Self::I8,
			"i16" => Self::I16,
			"i32" => Self::I32,
			"f16" => Self::F16,
			"f32" => Self::F32,
			_ => panic!("Currently only supports: u8, u16, u32, i8, i16, i32, f16, f32."),
		}
	}
}

pub fn get_format_and_type_from_image_pixel<P: Pixel>(format: &mut ChannelType, format_type: &mut ComponentType) -> Result<(), LoadImageError> {
	*format_type = match type_name::<P::Subpixel>() {
		"u8" =>  ComponentType::U8,
		"u16" => ComponentType::U16,
		"i8" =>  ComponentType::I8,
		"i16" => ComponentType::I16,
		"f16" => ComponentType::F16,
		"f32" => ComponentType::F32,
		"i32" => ComponentType::I32,
		"u32" => ComponentType::U32,
		other => return Err(LoadImageError::UnsupportedImageType(format!("Unknown subpixel type `{other}`"))),
	};
	*format = match format_type {
/// Input a generic type of `P` as the pixel data type, retrieve the channel type, and the component type
		ComponentType::I32 | ComponentType::U32 => {
			match P::CHANNEL_COUNT {
				1 => ChannelType::RedInteger,
				2 => ChannelType::RgInteger,
				3 => ChannelType::RgbInteger,
				4 => ChannelType::RgbaInteger,
				o => return Err(LoadImageError::UnsupportedImageType(format!("Unknown channel count ({o}) of the `ImageBuffer`"))),
			}
		}
		_ => {
			match P::CHANNEL_COUNT {
				1 => ChannelType::Red,
				2 => ChannelType::Rg,
				3 => ChannelType::Rgb,
				4 => ChannelType::Rgba,
				o => return Err(LoadImageError::UnsupportedImageType(format!("Unknown channel count ({o}) of the `ImageBuffer`"))),
			}
		}
	};
	Ok(())
}

impl PixelBuffer {
	/// Get the internal name
	pub fn get_name(&self) -> u32 {
		self.buffer.get_name()
	}

	/// Create a new pixel buffer
	pub fn new(glcore: Rc<GLCore>,
			width: u32,
			height: u32,
			depth: u32,
			size_in_bytes: usize,
			format: ChannelType,
			format_type: ComponentType,
			initial_data: Option<*const c_void>,
		) -> Self {
		let pixel_size = Self::size_of_pixel(format, format_type);
		let pitch = ((width as usize * pixel_size - 1) / 4 + 1) * 4;
		let pitch_wh = pitch * height as usize;
		let buffer = match initial_data {
			Some(initial_data) => Buffer::new(glcore.clone(), BufferTarget::PixelUnpackBuffer, size_in_bytes, BufferUsage::StreamDraw, initial_data),
			None => {
				let empty_data = vec![0u8; size_in_bytes];
				Buffer::new(glcore.clone(), BufferTarget::PixelUnpackBuffer, size_in_bytes, BufferUsage::StreamDraw, empty_data.as_ptr() as *const c_void)
			}
		};
		let buffer = BufferVec::new(glcore.clone(), buffer);
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

	/// Create from an `ImageBuffer`
	pub fn from_image<P: Pixel>(glcore: Rc<GLCore>, img: &ImageBuffer<P, Vec<P::Subpixel>>) -> Self {
		let container = img.as_raw();
		let mut format = ChannelType::Rgb;
		let mut format_type = ComponentType::U8;
		get_format_and_type_from_image_pixel::<P>(&mut format, &mut format_type).unwrap();
		Self::new(glcore, img.width(), img.height(), 1, size_of_val(&container[..]), format, format_type, Some(container.as_ptr() as *const c_void))
	}

	/// Create from a file
	pub fn from_file(glcore: Rc<GLCore>, path: &Path) -> Result<Self, LoadImageError> {
		let ext = path.extension().map_or_else(|| String::new(), |ext| OsStr::to_str(ext).unwrap().to_lowercase());
		match &ext[..] {
			"jpg" | "jpeg" => {
				let image_data = std::fs::read(path)?;
				let img: RgbImage = turbojpeg::decompress_image(&image_data)?;
				Ok(Self::from_image(glcore, &img))
			}
			_ => {
				match ImageReader::open(path)?.decode()? {
					DynamicImage::ImageLuma8(img) => Ok(Self::from_image(glcore, &img)),
					DynamicImage::ImageLumaA8(img) => Ok(Self::from_image(glcore, &img)),
					DynamicImage::ImageRgb8(img) => Ok(Self::from_image(glcore, &img)),
					DynamicImage::ImageRgba8(img) => Ok(Self::from_image(glcore, &img)),
					DynamicImage::ImageLuma16(img) => Ok(Self::from_image(glcore, &img)),
					DynamicImage::ImageLumaA16(img) => Ok(Self::from_image(glcore, &img)),
					DynamicImage::ImageRgb16(img) => Ok(Self::from_image(glcore, &img)),
					DynamicImage::ImageRgba16(img) => Ok(Self::from_image(glcore, &img)),
					DynamicImage::ImageRgb32F(img) => Ok(Self::from_image(glcore, &img)),
					DynamicImage::ImageRgba32F(img) => Ok(Self::from_image(glcore, &img)),
					_ => Err(LoadImageError::UnsupportedImageType(format!("Unsupported image type when loading pixel buffer from {path:?}"))),
				}
			}
		}
	}

	/// Get the size of the buffer
	pub fn size_in_bytes(&self) -> usize {
		self.buffer.size_in_bytes()
	}

	/// Get the size for each pixel
	pub fn size_of_pixel(format: ChannelType, format_type: ComponentType) -> usize {
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
			ChannelType::Red |
			ChannelType::RedInteger |
			ChannelType::StencilIndex |
			ChannelType::Depth => component_len,
			ChannelType::Rg |
			ChannelType::RgInteger |
			ChannelType::DepthStencil => component_len * 2,
			ChannelType::Rgb |
			ChannelType::RgbInteger |
			ChannelType::Bgr |
			ChannelType::BgrInteger => component_len * 3,
			ChannelType::Rgba |
			ChannelType::RgbaInteger |
			ChannelType::Bgra |
			ChannelType::BgraInteger => component_len * 4,
		}
	}

	/// Get the underlying buffer
	pub fn get_buffer(&self) -> &Buffer {
		self.buffer.get_buffer()
	}

	pub fn get_format(&self) -> ChannelType {
		self.format
	}

	/// Get the component type
	pub fn get_format_type(&self) -> ComponentType {
		self.format_type
	}

	/// Create a `BufferBind` to use the RAII system to manage the binding state
	pub fn bind<'a>(&'a self) -> BufferBind<'a> {
		self.buffer.bind()
	}
}

impl Texture {
	/// Get the internal name
	pub fn get_name(&self) -> u32 {
		self.name
	}

	/// When to create a new texture, must set up all of the parameters that are needed to be set up due to the specifications of OpenGL
	fn set_texture_params(
			glcore: Rc<GLCore>,
			name: u32,
			dim: TextureDimension,
			width: u32,
			height: &mut u32,
			depth: &mut u32,
			size_mod: &mut usize,
			wrapping_s: TextureWrapping,
			wrapping_t: TextureWrapping,
			wrapping_r: TextureWrapping,
			mag_filter: SamplerMagFilter,
			min_filter: SamplerFilter,
		) -> TextureTarget
	{
		let target;
		match dim {
			TextureDimension::Tex1d => {
				target = TextureTarget::Tex1d;
				*height = 1;
				*depth = 1;
				*size_mod = 1;
			}
			TextureDimension::Tex2d => {
				target = TextureTarget::Tex2d;
				*depth = 1;
				*size_mod = 1;
			}
			TextureDimension::Tex3d => {
				target = TextureTarget::Tex3d;
				*size_mod = 1;
			}
			TextureDimension::TexCube => {
				target = TextureTarget::TexCube;
				*height = width;
				*depth = 1;
				*size_mod = 6;
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
		target
	}

	/// Create an unallocated texture for further initialization
	fn new_unallocates(
			glcore: Rc<GLCore>,
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
		) -> Self {
		let mut name: u32 = 0;
		glcore.glGenTextures(1, &mut name as *mut _);
		let mut size_mod = 1;
		let target = Self::set_texture_params(glcore.clone(), name, dim, width, &mut height, &mut depth, &mut size_mod, wrapping_s, wrapping_t, wrapping_r, mag_filter, min_filter);
		let pixel_bits = format.bits_of_pixel(glcore.as_ref(), target);
		let pitch = ((pixel_bits - 1) / 32 + 1) * 4;
		let bytes_of_face = pitch * height as usize * depth as usize;
		let bytes_of_texture = bytes_of_face * size_mod;
		Self {
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
		}
	}

	/// Create from a pixel buffer
	fn new_from_pixel_buffer(
			glcore: Rc<GLCore>,
			dim: TextureDimension,
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
			pixel_buffer: PixelBuffer,
		) -> Self {
		let ret = Self::new_unallocates(glcore, dim, format, width, height, depth, wrapping_s, wrapping_t, wrapping_r, has_mipmap, mag_filter, min_filter);
		unsafe {ret.upload_texture(null(), pixel_buffer.get_format(), pixel_buffer.get_format_type(), has_mipmap)};
		ret
	}

	/// Create without pixel buffer
	fn new(glcore: Rc<GLCore>,
			dim: TextureDimension,
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
			buffer_format: ChannelType,
			buffer_format_type: ComponentType,
			initial_data: Option<*const c_void>,
		) -> Self {
		let mut ret = Self::new_unallocates(glcore, dim, format, width, height, depth, wrapping_s, wrapping_t, wrapping_r, has_mipmap, mag_filter, min_filter);
		if buffering {
			ret.create_pixel_buffer(buffer_format, buffer_format_type, initial_data);
		} else {
			if let Some(data_pointer) = initial_data {
				unsafe {ret.upload_texture(data_pointer, buffer_format, buffer_format_type, has_mipmap)};
			} else {
				let empty_data = vec![0u8; ret.bytes_of_texture];
				unsafe {ret.upload_texture(empty_data.as_ptr() as *const c_void, buffer_format, buffer_format_type, has_mipmap)};
			}
		}
		ret
	}

	/// Create an 1D texture
	pub fn new_1d(
	        glcore: Rc<GLCore>,
	        format: TextureFormat,
	        width: u32,
	        wrapping_s: TextureWrapping,
	        has_mipmap: bool,
	        mag_filter: SamplerMagFilter,
			min_filter: SamplerFilter,
			buffering: bool,
			buffer_format: ChannelType,
			buffer_format_type: ComponentType,
			initial_data: Option<*const c_void>,
		) -> Self {
		Self::new(glcore, TextureDimension::Tex1d, format, width, 1, 1, wrapping_s, TextureWrapping::Repeat, TextureWrapping::Repeat, has_mipmap, mag_filter, min_filter, buffering, buffer_format, buffer_format_type, initial_data)
	}

	/// Create an 2D texture
	pub fn new_2d(
	        glcore: Rc<GLCore>,
	        format: TextureFormat,
	        width: u32,
	        height: u32,
	        wrapping_s: TextureWrapping,
	        wrapping_t: TextureWrapping,
	        has_mipmap: bool,
	        mag_filter: SamplerMagFilter,
			min_filter: SamplerFilter,
			buffering: bool,
			buffer_format: ChannelType,
			buffer_format_type: ComponentType,
			initial_data: Option<*const c_void>,
		) -> Self {
		Self::new(glcore, TextureDimension::Tex2d, format, width, height, 1, wrapping_s, wrapping_t, TextureWrapping::Repeat, has_mipmap, mag_filter, min_filter, buffering, buffer_format, buffer_format_type, initial_data)
	}

	/// Create an 3D texture
	pub fn new_3d(
	        glcore: Rc<GLCore>,
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
			buffer_format: ChannelType,
			buffer_format_type: ComponentType,
			initial_data: Option<*const c_void>,
		) -> Self {
		Self::new(glcore, TextureDimension::Tex3d, format, width, height, depth, wrapping_s, wrapping_t, wrapping_r, has_mipmap, mag_filter, min_filter, buffering, buffer_format, buffer_format_type, initial_data)
	}

	/// Create an cube map texture
	pub fn new_cube(
	        glcore: Rc<GLCore>,
	        format: TextureFormat,
	        size: u32,
	        has_mipmap: bool,
	        mag_filter: SamplerMagFilter,
			min_filter: SamplerFilter,
			buffering: bool,
			buffer_format: ChannelType,
			buffer_format_type: ComponentType,
			initial_data: Option<*const c_void>,
		) -> Self {
		Self::new(glcore, TextureDimension::TexCube, format, size, size, 1, TextureWrapping::ClampToEdge, TextureWrapping::ClampToEdge, TextureWrapping::ClampToEdge, has_mipmap, mag_filter, min_filter, buffering, buffer_format, buffer_format_type, initial_data)
	}

	/// Create a texture from an image
	pub fn from_image<P: Pixel>(
			glcore: Rc<GLCore>,
			dim: TextureDimension,
			img: &ImageBuffer<P, Vec<P::Subpixel>>,
			wrapping_s: TextureWrapping,
			wrapping_t: TextureWrapping,
			has_mipmap: bool,
			mag_filter: SamplerMagFilter,
			min_filter: SamplerFilter,
		) -> Self {
		let mut buffer_format = ChannelType::Rgb;
		let mut buffer_format_type = ComponentType::U8;
		get_format_and_type_from_image_pixel::<P>(&mut buffer_format, &mut buffer_format_type).unwrap();
		let format = TextureFormat::from_format_and_type(buffer_format, buffer_format_type).unwrap();
		let pixel_buffer = PixelBuffer::from_image(glcore.clone(), img);
		match dim {
			TextureDimension::Tex1d => {
				assert_eq!(img.height(), 1);
				Self::new_from_pixel_buffer(glcore, dim, format, img.width(), 1, 1, wrapping_s, wrapping_t, TextureWrapping::Repeat, has_mipmap, mag_filter, min_filter, pixel_buffer)
			}
			TextureDimension::Tex2d => {
				Self::new_from_pixel_buffer(glcore, dim, format, img.width(), img.height(), 1, wrapping_s, wrapping_t, TextureWrapping::Repeat, has_mipmap, mag_filter, min_filter, pixel_buffer)
			}
			TextureDimension::TexCube => {
				assert_eq!(img.width() * 6, img.height());
				Self::new_from_pixel_buffer(glcore, dim, format, img.width(), img.width(), 1, TextureWrapping::ClampToEdge, TextureWrapping::ClampToEdge, TextureWrapping::ClampToEdge, has_mipmap, mag_filter, min_filter, pixel_buffer)
			}
			other => panic!("Could not create a {other:?} texture from a `ImageBuffer`")
		}
	}

	/// Create a texture from a file
	pub fn from_file(
			glcore: Rc<GLCore>,
			path: &Path,
			dim: TextureDimension,
			wrapping_s: TextureWrapping,
			wrapping_t: TextureWrapping,
			has_mipmap: bool,
			mag_filter: SamplerMagFilter,
			min_filter: SamplerFilter,
		) -> Result<Self, LoadImageError> {
		let ext = path.extension().map_or_else(|| String::new(), |ext| OsStr::to_str(ext).unwrap().to_lowercase());
		match &ext[..] {
			"jpg" | "jpeg" => {
				let image_data = std::fs::read(path)?;
				let img: RgbImage = turbojpeg::decompress_image(&image_data)?;
				Ok(Self::from_image(glcore, dim, &img, wrapping_s, wrapping_t, has_mipmap, mag_filter, min_filter))
			}
			_ => {
				match ImageReader::open(path)?.decode()? {
					DynamicImage::ImageLuma8(img) => Ok(Self::from_image(glcore, dim, &img, wrapping_s, wrapping_t, has_mipmap, mag_filter, min_filter)),
					DynamicImage::ImageLumaA8(img) => Ok(Self::from_image(glcore, dim, &img, wrapping_s, wrapping_t, has_mipmap, mag_filter, min_filter)),
					DynamicImage::ImageRgb8(img) => Ok(Self::from_image(glcore, dim, &img, wrapping_s, wrapping_t, has_mipmap, mag_filter, min_filter)),
					DynamicImage::ImageRgba8(img) => Ok(Self::from_image(glcore, dim, &img, wrapping_s, wrapping_t, has_mipmap, mag_filter, min_filter)),
					DynamicImage::ImageLuma16(img) => Ok(Self::from_image(glcore, dim, &img, wrapping_s, wrapping_t, has_mipmap, mag_filter, min_filter)),
					DynamicImage::ImageLumaA16(img) => Ok(Self::from_image(glcore, dim, &img, wrapping_s, wrapping_t, has_mipmap, mag_filter, min_filter)),
					DynamicImage::ImageRgb16(img) => Ok(Self::from_image(glcore, dim, &img, wrapping_s, wrapping_t, has_mipmap, mag_filter, min_filter)),
					DynamicImage::ImageRgba16(img) => Ok(Self::from_image(glcore, dim, &img, wrapping_s, wrapping_t, has_mipmap, mag_filter, min_filter)),
					DynamicImage::ImageRgb32F(img) => Ok(Self::from_image(glcore, dim, &img, wrapping_s, wrapping_t, has_mipmap, mag_filter, min_filter)),
					DynamicImage::ImageRgba32F(img) => Ok(Self::from_image(glcore, dim, &img, wrapping_s, wrapping_t, has_mipmap, mag_filter, min_filter)),
					_ => Err(LoadImageError::UnsupportedImageType(format!("Unsupported image type when loading texture from {path:?}"))),
				}
			}
		}
	}


	/// Bind the texture, using the RAII system to manage the binding state
	pub fn bind<'a>(&'a self) -> TextureBind<'a> {
		match self.dim {
			TextureDimension::Tex1d => TextureBind::new(self, TextureTarget::Tex1d),
			TextureDimension::Tex2d => TextureBind::new(self, TextureTarget::Tex2d),
			TextureDimension::Tex3d => TextureBind::new(self, TextureTarget::Tex3d),
			TextureDimension::TexCube => panic!("Please use `bind_face()` to bind a cube map."),
		}
	}

	/// Bind a cubemap face, using the RAII system to manage the binding state
	pub fn bind_face<'a>(&'a self, face: CubeMapFaces) -> TextureBind<'a> {
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
	pub fn map_buffer<'a>(&'a mut self, access: MapAccess) -> Option<(BufferBind<'a>, BufferMapping<'a>, *mut c_void)> {
		self.pixel_buffer.as_ref().map(|b|{
			let bind = b.bind();
			let (mapping, address) = bind.map(access);
			(bind, mapping, address)
		})
	}

	pub unsafe fn download_texture(&self, data: *mut c_void, buffer_format: ChannelType, buffer_format_type: ComponentType) {
	/// Retrieve the pixels from the texture to the specified data pointer regardless of is currently using a PBO or not
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

	pub unsafe fn upload_texture(&self, data: *const c_void, buffer_format: ChannelType, buffer_format_type: ComponentType, regen_mipmap: bool) {
	/// Load the texture with the specified data pointer regardless of is currently using a PBO or not
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
		unsafe {self.download_texture(std::ptr::null_mut::<c_void>(), buffer_format, buffer_format_type)};
		bind_pbo.unbind();
	}

	/// Apply the change to the pixel buffer of the texture
	pub fn unpack_pixel_buffer(&self, regen_mipmap: bool) {
		let pixel_buffer = self.pixel_buffer.as_ref().unwrap();
		let buffer_format = pixel_buffer.format;
		let buffer_format_type = pixel_buffer.format_type;
		let bind_pbo = pixel_buffer.bind();
		unsafe {self.upload_texture(std::ptr::null(), buffer_format, buffer_format_type, regen_mipmap)};
		bind_pbo.unbind();
	}

	pub fn create_pixel_buffer(&mut self, buffer_format: ChannelType, buffer_format_type: ComponentType, initial_data: Option<*const c_void>) {
		self.pixel_buffer = Some(PixelBuffer::new(self.glcore.clone(), self.width, self.height, self.depth, self.bytes_of_texture, buffer_format, buffer_format_type, initial_data))
	/// Create the PBO if not been created earlier
	}

	/// Discard the PBO if not necessarily need it
	pub fn drop_pixel_buffer(&mut self) {
		self.pixel_buffer = None
	}

	/// Get width
	pub fn get_width(&self) -> u32 {
		self.width
	}

	/// Get height
	pub fn get_height(&self) -> u32 {
		self.height
	}

	/// Get depth
	pub fn get_depth(&self) -> u32 {
		self.depth
	}

	/// Get dimension
	pub fn get_dim(&self) -> TextureDimension {
		self.dim
	}

	/// Set the active texture unit
	pub fn set_active_unit(&self, unit: u32) {
		self.glcore.glActiveTexture(GL_TEXTURE0 + unit)
	}
}

impl Drop for Texture {
	fn drop(&mut self) {
		self.glcore.glDeleteTextures(1, &self.name as *const u32);
	}
}

impl<'a> TextureBind<'a> {
	/// Create a binding state to the texture, utilizing the RAII rules to manage the binding state
	fn new(texture: &'a Texture, target: TextureTarget) -> Self {
		texture.glcore.glBindTexture(target as u32, texture.name);
		Self {
			texture,
			target,
		}
	}

	/// Set the active texture unit
	pub fn set_active_unit(&self, unit: u32) {
		self.texture.glcore.glActiveTexture(GL_TEXTURE0 + unit)
	}

	/// Explicitly unbind the texture.
	pub fn unbind(self) {}
}

impl Drop for TextureBind<'_> {
	fn drop(&mut self) {
		self.texture.glcore.glBindTexture(self.target as u32, 0);
	}
}

impl Debug for Texture {
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

impl Debug for ChannelType {
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
