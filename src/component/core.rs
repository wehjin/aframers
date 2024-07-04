use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::{Element, js_sys};

pub trait ComponentValue {
	fn component_name(&self) -> &str;
	fn component_value(&self) -> impl AsRef<str>;
}

#[wasm_bindgen(js_namespace = AFRAME)]
extern "C" {
	#[wasm_bindgen(js_name = registerComponent)]
	pub fn register_component(name: &str, registration: &js_sys::Map);
}

#[wasm_bindgen]
extern "C" {
	pub type Component;

	#[wasm_bindgen(method, getter, structural)]
	pub fn data(this: &Component) -> JsValue;

	#[wasm_bindgen(method, getter, structural)]
	pub fn el(this: &Component) -> Element;
}

#[wasm_bindgen(
	inline_js = "export function component_registration(f) { return  { init: function () { f(this); } }; }"
)]
extern "C" {
	pub fn component_registration(closure: &Closure<dyn Fn(Component)>) -> js_sys::Object;
}

#[wasm_bindgen(
	inline_js = "export function component_registration_with_schema(o, f) { return  { schema: o, init: function () { f(this); } }; }"
)]
extern "C" {
	pub fn component_registration_with_schema(schema: js_sys::Object, closure: &Closure<dyn Fn(Component)>) -> js_sys::Object;
}
