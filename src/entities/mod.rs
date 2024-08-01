use web_sys::wasm_bindgen::JsValue;

use crate::af_sys::entities::{a_entity_create, AEntity};
use crate::components::core::ComponentAttribute;

pub fn create_entity() -> Result<Entity, JsValue> {
	let a_entity = a_entity_create("a-entity")?;
	let entity = Entity::from(a_entity);
	Ok(entity)
}

pub fn create_camera_entity() -> Result<Entity, JsValue> {
	let a_entity = a_entity_create("a-camera")?;
	let entity = Entity::from(a_entity);
	Ok(entity)
}

pub fn create_light_entity() -> Result<Entity, JsValue> {
	let a_entity = a_entity_create("a-light")?;
	let entity = Entity::from(a_entity);
	Ok(entity)
}

pub fn create_box_entity() -> Result<Entity, JsValue> {
	let a_entity = a_entity_create("a-box")?;
	let entity = Entity::from(a_entity);
	Ok(entity)
}

pub fn create_cone_entity() -> Result<Entity, JsValue> {
	let a_entity = a_entity_create("a-cone")?;
	let entity = Entity::from(a_entity);
	Ok(entity)
}

pub fn create_sphere_entity() -> Result<Entity, JsValue> {
	let a_entity = a_entity_create("a-sphere")?;
	let entity = Entity::from(a_entity);
	Ok(entity)
}

pub fn create_cylinder_entity() -> Result<Entity, JsValue> {
	let a_entity = a_entity_create("a-cylinder")?;
	let entity = Entity::from(a_entity);
	Ok(entity)
}

pub fn create_plane_entity() -> Result<Entity, JsValue> {
	let a_entity = a_entity_create("a-plane")?;
	let entity = Entity::from(a_entity);
	Ok(entity)
}

pub fn create_sky_entity() -> Result<Entity, JsValue> {
	let a_entity = a_entity_create("a-sky")?;
	let entity = Entity::from(a_entity);
	Ok(entity)
}

#[derive(Clone)]
pub struct Entity(AEntity);

impl Entity {
	pub fn a_entity(&self) -> &AEntity { &self.0 }
	pub fn into_a_entity(self) -> AEntity { self.0 }
	pub fn id(&self) -> String { self.0.id() }
	pub fn set_id(self, value: impl AsRef<str>) -> Result<Self, JsValue> {
		self.0.set_id(value.as_ref());
		Ok(self)
	}
	pub fn set_component_attribute(self, attr: impl ComponentAttribute) -> Result<Self, JsValue> {
		self.0.set_component_attribute(attr);
		Ok(self)
	}
	pub fn append_child(self, entity: Entity) -> Result<Self, JsValue> {
		self.0.append_child(&entity.0)?;
		Ok(self)
	}
}

impl From<AEntity> for Entity {
	fn from(value: AEntity) -> Self {
		Self(value)
	}
}
