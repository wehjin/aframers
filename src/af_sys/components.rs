use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::js_sys;

use crate::af_sys::entities::AEntity;

#[wasm_bindgen(js_namespace = AFRAME)]
extern "C" {
	#[wasm_bindgen(js_name = registerComponent)]
	pub fn register_component(name: &str, registration: &js_sys::Object);
}

#[wasm_bindgen]
extern "C" {
	pub type AComponent;

	#[wasm_bindgen(method, getter)]
	pub fn data(this: &AComponent) -> JsValue;

	#[wasm_bindgen(method, getter, js_name = el)]
	pub fn a_entity(this: &AComponent) -> AEntity;

	#[wasm_bindgen(method, getter)]
	pub fn id(this: &AComponent) -> String;
}

#[wasm_bindgen(
	inline_js = "export function component_registration(f) { return  { init: function () { f(this); } }; }"
)]
extern "C" {
	pub fn component_registration(closure: &Closure<dyn Fn(AComponent)>) -> js_sys::Object;
}

#[wasm_bindgen(
	inline_js = "export function component_registration_with_schema(o, f) { return  { schema: o, init: function () { f(this); } }; }"
)]
extern "C" {
	pub fn component_registration_with_schema(schema: js_sys::Object, closure: &Closure<dyn Fn(AComponent)>) -> js_sys::Object;
}
