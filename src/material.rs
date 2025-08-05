
use crate::prelude::*;
use std::{
	collections::{HashMap, BTreeSet},
	fmt::Debug,
	rc::Rc,
};
use glm::*;

#[derive(Debug, Clone)]
pub enum TextureOrColor {
	Texture(Rc<Texture>),
	Color(Vec4),
}

#[derive(Default, Debug, Clone)]
pub struct MaterialLegacy {
	pub ambient: TextureOrColor,
	pub diffuse: TextureOrColor,
	pub specular: TextureOrColor,
	pub normal: TextureOrColor,
	pub emissive: TextureOrColor,
	pub others: HashMap<String, TextureOrColor>,
}

#[derive(Default, Debug, Clone)]
pub struct MaterialPbr {
	pub albedo: TextureOrColor,
	pub normal: TextureOrColor,
	pub ao: TextureOrColor,
	pub displacement: TextureOrColor,
	pub roughness: TextureOrColor,
	pub metalness: TextureOrColor,
	pub emissive: TextureOrColor,
	pub others: HashMap<String, TextureOrColor>,
}

impl Default for TextureOrColor {
	fn default() -> Self {
		Self::Color(Vec4::new(0.5, 0.5, 0.5, 1.0))
	}
}

pub trait Material: Debug {
	fn get_ambient(&self) -> Option<&TextureOrColor>;
	fn get_diffuse(&self) -> Option<&TextureOrColor>;
	fn get_specular(&self) -> Option<&TextureOrColor>;
	fn get_albedo(&self) -> Option<&TextureOrColor>;
	fn get_ao(&self) -> Option<&TextureOrColor>;
	fn get_displacement(&self) -> Option<&TextureOrColor>;
	fn get_roughness(&self) -> Option<&TextureOrColor>;
	fn get_metalness(&self) -> Option<&TextureOrColor>;
	fn get_normal(&self) -> Option<&TextureOrColor>;
	fn get_emissive(&self) -> Option<&TextureOrColor>;
	fn get_names(&self) -> BTreeSet<String>;
	fn get_by_name(&self, name: &str) -> Option<&TextureOrColor>;
	fn set_by_name(&mut self, name: &str, texture: TextureOrColor);
}

impl Material for MaterialLegacy {
	fn get_ambient(&self) ->		Option<&TextureOrColor> {Some(&self.ambient)}
	fn get_diffuse(&self) ->		Option<&TextureOrColor> {Some(&self.diffuse)}
	fn get_specular(&self) ->		Option<&TextureOrColor> {Some(&self.specular)}
	fn get_normal(&self) ->			Option<&TextureOrColor> {Some(&self.normal)}
	fn get_emissive(&self) ->		Option<&TextureOrColor> {Some(&self.emissive)}

	fn get_albedo(&self) ->			Option<&TextureOrColor> {None}
	fn get_ao(&self) ->				Option<&TextureOrColor> {None}
	fn get_displacement(&self) ->	Option<&TextureOrColor> {None}
	fn get_roughness(&self) ->		Option<&TextureOrColor> {None}
	fn get_metalness(&self) ->		Option<&TextureOrColor> {None}

	fn get_names(&self) -> BTreeSet<String> {
		let mut ret = BTreeSet::new();
		ret.insert("ambient".to_owned());
		ret.insert("diffuse".to_owned());
		ret.insert("specular".to_owned());
		ret.insert("normal".to_owned());
		ret.insert("emissive".to_owned());
		for (name, _) in self.others.iter() {
			ret.insert(name.clone());
		}
		ret
	}

	fn get_by_name(&self, name: &str) -> Option<&TextureOrColor> {
		match self.others.get(&name.to_owned()) {
			Some(data) => Some(data),
			None => {
				match name {
					"ambient" =>	self.get_ambient(),
					"diffuse" =>	self.get_diffuse(),
					"specular" =>	self.get_specular(),
					"normal" =>		self.get_normal(),
					"emissive" =>	self.get_emissive(),
					_ => None,
				}
			}
		}
	}

	fn set_by_name(&mut self, name: &str, texture: TextureOrColor) {
		match name {
			"ambient" =>	self.ambient = texture,
			"diffuse" =>	self.diffuse = texture,
			"specular" =>	self.specular = texture,
			"normal" =>		self.normal = texture,
			"emissive" =>	self.emissive = texture,
			others =>{
				self.others.insert(others.to_owned(), texture);
			}
		}
	}
}

impl Material for MaterialPbr {
	fn get_albedo(&self) ->			Option<&TextureOrColor> {Some(&self.albedo)}
	fn get_ao(&self) ->				Option<&TextureOrColor> {Some(&self.ao)}
	fn get_displacement(&self) ->	Option<&TextureOrColor> {Some(&self.displacement)}
	fn get_roughness(&self) ->		Option<&TextureOrColor> {Some(&self.roughness)}
	fn get_metalness(&self) ->		Option<&TextureOrColor> {Some(&self.metalness)}
	fn get_normal(&self) ->			Option<&TextureOrColor> {Some(&self.normal)}
	fn get_emissive(&self) ->		Option<&TextureOrColor> {Some(&self.emissive)}

	fn get_ambient(&self) ->		Option<&TextureOrColor> {None}
	fn get_diffuse(&self) ->		Option<&TextureOrColor> {None}
	fn get_specular(&self) ->		Option<&TextureOrColor> {None}

	fn get_names(&self) -> BTreeSet<String> {
		let mut ret = BTreeSet::new();
		ret.insert("albedo".to_owned());
		ret.insert("ao".to_owned());
		ret.insert("displacement".to_owned());
		ret.insert("roughness".to_owned());
		ret.insert("metalness".to_owned());
		ret.insert("normal".to_owned());
		ret.insert("emissive".to_owned());
		for (name, _) in self.others.iter() {
			ret.insert(name.clone());
		}
		ret
	}

	fn get_by_name(&self, name: &str) -> Option<&TextureOrColor> {
		match self.others.get(&name.to_owned()) {
			Some(data) => Some(data),
			None => {
				match name {
					"albedo" =>			self.get_albedo(),
					"ao" =>				self.get_ao(),
					"displacement" =>	self.get_displacement(),
					"roughness" =>		self.get_roughness(),
					"metalness" =>		self.get_metalness(),
					"normal" =>			self.get_normal(),
					"emissive" =>		self.get_emissive(),
					_ => None,
				}
			}
		}
	}

	fn set_by_name(&mut self, name: &str, texture: TextureOrColor) {
		match name {
			"albedo" =>			self.albedo = texture,
			"ao" =>				self.ao = texture,
			"displacement" =>	self.displacement = texture,
			"roughness" =>		self.roughness = texture,
			"metalness" =>		self.metalness = texture,
			"normal" =>			self.normal = texture,
			"emissive" =>		self.emissive = texture,
			others =>{
				self.others.insert(others.to_owned(), texture);
			}
		}
	}
}
