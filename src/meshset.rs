
use crate::prelude::*;
use std::{
	collections::BTreeMap,
	rc::Rc,
};

/// Mesh set, each mesh has its name and material.
#[derive(Debug, Clone)]
pub struct Meshset<M: Mesh, Mat: Material> {
	pub subsets: BTreeMap<String, Rc<MeshWithMaterial<M, Mat>>>,
}

/// Pipeline set, converted from the mesh set, for batch drawing.
#[derive(Debug, Clone)]
pub struct Pipelineset<V: VertexType, I: VertexType, M: Mesh, Mat: Material> {
	pub subsets: BTreeMap<String, Vec<Rc<Pipeline<V, I, M, Mat>>>>,
}

impl<V: VertexType, I: VertexType, M: Mesh, Mat: Material> Pipelineset<V, I, M, Mat> {
	/// Create a pipeline set from the mesh set with shaders.
	pub fn from_meshset(glcore: Rc<GLCore>, meshset: Meshset<M, Mat>, shaders: &[Rc<Shader>]) -> Self {
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

	/// Draw the pipeline set to a framebuffer
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
