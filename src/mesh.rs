#![allow(dead_code)]

use glcore::*;
use crate::glbuffer::*;

#[derive(Debug, Clone)]
pub struct Mesh<'a> {
	glcore: &'a GLCore,
	vertex_buffer: Buffer<'a>,
	instance_buffer: Option<Buffer<'a>>,
	command_buffer: Option<Buffer<'a>>
}


