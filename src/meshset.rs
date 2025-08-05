
use crate::prelude::*;
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
	pub subsets: BTreeMap<String, Vec<Rc<Pipeline<V, I, M, Mat>>>>,
}

impl<V: VertexType, I: VertexType, M: Mesh, Mat: Material> Pipelineset<V, I, M, Mat> {
	pub fn from_meshset(glcore: Rc<GLCore>, meshset: Meshset<M, Mat>, shader: Rc<Shader>) -> Self {
		let mut subsets = BTreeMap::new();
		for (name, mesh) in meshset.subsets.iter() {
			let mut v = Vec::new();
			v.push(Rc::new(Pipeline::new(glcore.clone(), mesh.clone(), shader.clone())));
			subsets.insert(name.clone(), v);
		}
		Self {
			subsets
		}
	}

	pub fn from_meshset_with_multiple_shaders(glcore: Rc<GLCore>, meshset: Meshset<M, Mat>, shaders: &[Rc<Shader>]) -> Self {
		let mut subsets = BTreeMap::new();
		for (name, mesh) in meshset.subsets.iter() {
			let mut v = Vec::with_capacity(shaders.len());
			for shader in shaders.iter() {
				v.push(Rc::new(Pipeline::new(glcore.clone(), mesh.clone(), shader.clone())));
			}
			subsets.insert(name.clone(), v);
		}
		Self {
			subsets
		}
	}

	pub fn draw(&self, fbo: Option<&Framebuffer>) {
		for (_name, pipelines) in self.subsets.iter() {
			for pipeline in pipelines.iter() {
				let bind = pipeline.bind();
				bind.draw(fbo);
				bind.unbind();
			}
		}
	}
}
