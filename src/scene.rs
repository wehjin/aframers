use web_sys::Element;
use web_sys::wasm_bindgen::JsValue;

use crate::browser::document;

pub fn create_scene() -> Result<Element, JsValue> {
	let element = document().create_element("a-scene");
	element
}

