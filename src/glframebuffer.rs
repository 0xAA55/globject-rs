
#![allow(dead_code)]

use glcore::*;
use std::{
	fmt::{self, Debug, Formatter},
	rc::Rc,
};

pub struct Framebuffer {
	pub glcore: Rc<GLCore>,
	name: u32,
}

impl Debug for Framebuffer {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		f.debug_struct("Framebuffer")
		.field("name", &self.name)
		.finish()
	}
}
