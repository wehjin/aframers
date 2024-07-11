use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::{Element, HtmlElement, Node};
use web_sys::js_sys;
use web_sys::js_sys::Object;

use crate::af_sys::components::AComponent;
use crate::af_sys::scenes::AScene;
use crate::browser::document;

#[wasm_bindgen]
extern "C" {
	#[wasm_bindgen(extends = HtmlElement, extends = Element, extends = Node)]
	#[derive(Clone)]
	pub type AEntity;

	#[wasm_bindgen(method, getter, js_name = sceneEl)]
	pub fn a_scene(this: &AEntity) -> AScene;

	#[wasm_bindgen(method, getter)]
	pub fn components(this: &AEntity) -> Object;

	#[wasm_bindgen(method, js_name = addState)]
	pub fn add_state(this: &AEntity, name: &str);

	#[wasm_bindgen(method, js_name = removeState)]
	pub fn remove_state(this: &AEntity, name: &str);

	#[wasm_bindgen(method, js_name = is)]
	pub fn is_state(this: &AEntity, name: &str) -> bool;

	#[wasm_bindgen(method, js_name = emit)]
	pub fn emit_event(this: &AEntity, name: &str);
}

impl From<&str> for AEntity {
	fn from(value: &str) -> Self {
		a_entity_create(value).unwrap()
	}
}

impl AEntity {
	pub fn get_component(&self, name: impl AsRef<str>) -> AComponent {
		let components = self.components();
		let component = js_sys::Reflect::get(&components, &name.as_ref().into()).expect("get component");
		component.unchecked_into()
	}
}
pub fn a_entity_create(tag: impl AsRef<str>) -> Result<AEntity, JsValue> {
	document()
		.create_element(tag.as_ref())
		.map(|element| element.unchecked_into())
}
