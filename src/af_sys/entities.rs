use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::{Element, HtmlElement, Node};

use crate::af_sys::scenes::AScene;
use crate::browser::document;

#[wasm_bindgen]
extern "C" {
	#[wasm_bindgen(extends = HtmlElement, extends = Element, extends = Node)]
	#[derive(Clone)]
	pub type AEntity;

	#[wasm_bindgen(method, getter, js_name = sceneEl)]
	pub fn a_scene(this: &AEntity) -> AScene;
}

impl From<&str> for AEntity {
	fn from(value: &str) -> Self {
		a_entity_create(value).unwrap()
	}
}

pub fn a_entity_create(tag: impl AsRef<str>) -> Result<AEntity, JsValue> {
	document()
		.create_element(tag.as_ref())
		.map(|element| element.unchecked_into())
}
