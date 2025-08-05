
use glcore::*;
use crate::glbuffer::*;
use crate::glshader::*;
use crate::glframebuffer::*;
use crate::mesh::*;
use crate::material::*;
use crate::pipeline::*;
use std::{
	collections::BTreeMap,
	rc::Rc,
};

#[derive(Debug, Clone)]
pub struct Meshset<M: Mesh, Mat: Material> {
	pub subsets: BTreeMap<String, Rc<MeshWithMaterial<M, Mat>>>,
}

#[derive(Debug, Clone)]
pub struct Pipelineset<V: VertexType, I: VertexType, M: Mesh, Mat: Material> {
	pub subsets: BTreeMap<String, Rc<Pipeline<V, I, M, Mat>>>,
}

impl<V: VertexType, I: VertexType, M: Mesh, Mat: Material> Pipelineset<V, I, M, Mat> {
	pub fn from_meshset(glcore: Rc<GLCore>, meshset: Meshset<M, Mat>, shader: Rc<Shader>) -> Self {
		let mut subsets: BTreeMap<String, Rc<Pipeline<V, I, M, Mat>>> = BTreeMap::new();
		for (name, mesh) in meshset.subsets.iter() {
			subsets.insert(name.clone(), Rc::new(Pipeline::new(glcore.clone(), mesh.clone(), shader.clone())));
		}
		Self {
			subsets
		}
	}

	pub fn draw(&self, fbo: Option<&Framebuffer>) {
		for (_name, pipeline) in self.subsets.iter() {
			let bind = pipeline.bind();
			bind.draw(fbo);
			bind.unbind();
		}
	}
}

impl<V: VertexType, I: VertexType, M: Mesh, Mat: Material> From<Pipelineset<V, I, M, Mat>> for Meshset<M, Mat> {
	fn from(val: Pipelineset<V, I, M, Mat>) -> Self {
		let mut subsets: BTreeMap<String, Rc<MeshWithMaterial<M, Mat>>> = BTreeMap::new();
		for (name, pipeline) in val.subsets.iter() {
			subsets.insert(name.clone(), pipeline.mesh.clone());
		}
		Self {
			subsets
		}
	}
}
