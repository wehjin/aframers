use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::{Element, HtmlElement, Node};
use web_sys::js_sys;
use web_sys::js_sys::Object;

use crate::af_sys::components::AComponent;
use crate::af_sys::scenes::AScene;
use crate::browser::document;
use crate::components::core::ComponentAttribute;

#[wasm_bindgen]
extern "C" {
	#[wasm_bindgen(extends = HtmlElement, extends = Element, extends = Node)]
	#[derive(Clone)]
	pub type AEntity;

	#[wasm_bindgen(method, getter, js_name = sceneEl)]
	pub fn a_scene(this: &AEntity) -> AScene;

	#[wasm_bindgen(method, getter)]
	pub fn components(this: &AEntity) -> Object;

	#[wasm_bindgen(method, js_name = setAttribute)]
	pub fn update_component_property(this: &AEntity, component: &str, property: &str, value: &JsValue);

	#[wasm_bindgen(method, js_name = addState)]
	pub fn add_state(this: &AEntity, name: &str);

	#[wasm_bindgen(method, js_name = removeState)]
	pub fn remove_state(this: &AEntity, name: &str);

	#[wasm_bindgen(method, js_name = is)]
	pub fn is_state(this: &AEntity, name: &str) -> bool;

	#[wasm_bindgen(method, js_name = emit)]
	pub fn emit_event(this: &AEntity, name: &str);

	#[wasm_bindgen(method, js_name = emit)]
	pub fn emit_event_with_details(this: &AEntity, name: &str, details: &JsValue);
}

impl From<&str> for AEntity {
	fn from(value: &str) -> Self {
		a_entity_create(value).unwrap()
	}
}

impl AEntity {
	pub fn new() -> Self {
		Self::new_with_tag("a-entity")
	}
	pub fn new_with_tag(tag: impl AsRef<str>) -> Self {
		a_entity_create(tag).unwrap()
	}
	pub fn get_component(&self, name: impl AsRef<str>) -> AComponent {
		let components = self.components();
		let component = js_sys::Reflect::get(&components, &name.as_ref().into()).expect("get component");
		component.unchecked_into()
	}
	pub fn set_component_attribute(&self, attr: impl ComponentAttribute) -> &Self {
		let (name, value) = (attr.as_attribute_name(), attr.as_attribute_str());
		let (name, value) = (name.as_ref(), value.as_ref());
		assert!(
			self.set_attribute(name, value).is_ok(),
			"Failed to set attribute '{name}' to value '{value}' in AEntity '{}'", self.id()
		);
		&self
	}
}
pub fn a_entity_create(tag: impl AsRef<str>) -> Result<AEntity, JsValue> {
	document()
		.create_element(tag.as_ref())
		.map(|element| element.unchecked_into())
}
