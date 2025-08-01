
#![allow(dead_code)]

use glcore::*;
use std::{
	fmt::{self, Debug, Formatter},
};

pub struct Framebuffer<'a> {
	pub glcore: &'a GLCore,
	name: u32,
}

impl Debug for Framebuffer<'_> {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		f.debug_struct("Framebuffer")
		.field("name", &self.name)
		.finish()
	}
}
