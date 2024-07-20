use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::{Element, HtmlElement, Node};
use web_sys::js_sys::Object;

use crate::af_sys::entities::AEntity;

#[wasm_bindgen]
extern "C" {
	#[wasm_bindgen(extends = AEntity, extends = HtmlElement, extends = Element, extends = Node)]
	pub type AScene;

	#[wasm_bindgen(method, getter)]
	pub fn systems(this: &AScene) -> Object;
}
