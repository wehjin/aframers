use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::js_sys::Object;

use crate::af_sys::scenes::AScene;

#[wasm_bindgen(js_namespace = AFRAME)]
extern "C" {
	#[wasm_bindgen(js_name = registerSystem)]
	pub fn register_system(name: &str, definition: &Object);
}

#[wasm_bindgen]
extern "C" {
	pub type ASystem;
	#[wasm_bindgen(method, getter)]
	pub fn data(this: &ASystem) -> JsValue;
	#[wasm_bindgen(method, getter, js_name = el)]
	pub fn a_scene(this: &ASystem) -> AScene;
}

