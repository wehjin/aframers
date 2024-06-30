use web_sys::Element;
use web_sys::wasm_bindgen::JsValue;

use crate::browser::document;
use crate::component::core::ComponentValue;

pub fn create_entity() -> Result<Entity, JsValue> {
	let element = document().create_element("a-entity")?;
	let entity = Entity(element);
	Ok(entity)
}

pub fn create_camera_entity() -> Result<Entity, JsValue> {
	let element = document().create_element("a-camera")?;
	let entity = Entity(element);
	Ok(entity)
}

pub fn create_light_entity() -> Result<Entity, JsValue> {
	let element = document().create_element("a-light")?;
	let entity = Entity(element);
	Ok(entity)
}

pub fn create_box_entity() -> Result<Entity, JsValue> {
	let element = document().create_element("a-box")?;
	let entity = Entity(element);
	Ok(entity)
}

pub fn create_cone_entity() -> Result<Entity, JsValue> {
	let element = document().create_element("a-cone")?;
	let entity = Entity(element);
	Ok(entity)
}

pub fn create_sphere_entity() -> Result<Entity, JsValue> {
	let element = document().create_element("a-sphere")?;
	let entity = Entity(element);
	Ok(entity)
}

pub fn create_cylinder_entity() -> Result<Entity, JsValue> {
	let element = document().create_element("a-cylinder")?;
	let entity = Entity(element);
	Ok(entity)
}

pub fn create_plane_entity() -> Result<Entity, JsValue> {
	let element = document().create_element("a-plane")?;
	let entity = Entity(element);
	Ok(entity)
}

pub fn create_sky_entity() -> Result<Entity, JsValue> {
	let element = document().create_element("a-sky")?;
	let entity = Entity(element);
	Ok(entity)
}


pub struct Entity(pub Element);

impl Entity {
	pub fn element(&self) -> &Element { &self.0 }
	pub fn set_component(self, component: impl ComponentValue) -> Result<Self, JsValue> {
		self.0.set_attribute(component.component_name(), component.component_value().as_ref())?;
		Ok(self)
	}
	pub fn append_child(&self, entity: &Entity) -> Result<&Self, JsValue> {
		self.0.append_child(&entity.element())?;
		Ok(self)
	}
}
