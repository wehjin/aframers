use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen::closure::Closure;
use web_sys::HtmlScriptElement;

use crate::browser::document;

pub mod browser;
pub mod component;
pub mod entity;
pub mod scene;
const SCRIPT_URL: &str = "https://aframe.io/releases/1.6.0/aframe.min.js";

pub fn init(f: impl FnOnce() -> Result<(), JsValue> + 'static) -> Result<(), JsValue> {
	let document = document();
	let script: HtmlScriptElement = document.create_element("script")?.dyn_into()?;
	script.set_attribute("src", SCRIPT_URL)?;
	script.add_event_listener_with_callback("load", Closure::once_into_js(f).as_ref().unchecked_ref())?;
	document.head().expect("Head exists").append_child(&script)?;
	Ok(())
}