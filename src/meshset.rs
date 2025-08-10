
use crate::prelude::*;
use std::{
	collections::BTreeMap,
	rc::Rc,
};

/// Mesh set, each mesh has its name and material.
#[derive(Debug, Clone)]
pub struct Meshset {
	pub subsets: BTreeMap<String, Rc<dyn GenericMeshWithMaterial>>,
}

/// Pipeline set, converted from the mesh set, for batch drawing.
#[derive(Debug, Clone)]
pub struct Pipelineset<V: VertexType, I: VertexType> {
	pub subsets: BTreeMap<String, Vec<Rc<Pipeline<V, I>>>>,
}

impl<V: VertexType, I: VertexType> Pipelineset<V, I> {
	/// Create a pipeline set from the mesh set with shaders.
	pub fn from_meshset(glcore: Rc<GLCore>, meshset: Meshset, shaders: &[Rc<Shader>]) -> Result<Self, PipelineError> {
		let mut subsets = BTreeMap::new();
		for (name, mesh) in meshset.subsets.iter() {
			let mut v = Vec::with_capacity(shaders.len());
			for shader in shaders.iter() {
				v.push(Rc::new(Pipeline::new(glcore.clone(), mesh.clone(), shader.clone())?));
			}
			subsets.insert(name.clone(), v);
		}
		Ok(Self {
			subsets
		})
	}

	/// Draw the pipeline set to a framebuffer
	pub fn draw(&self, fbo: Option<&Framebuffer>) -> Result<(), PipelineError> {
		for (_name, pipelines) in self.subsets.iter() {
			for pipeline in pipelines.iter() {
				let bind = pipeline.bind()?;
				bind.draw(fbo)?;
				bind.unbind();
			}
		}
		Ok(())
	}
}
