use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::{Element, HtmlElement, Node};

#[wasm_bindgen]
extern "C" {
	#[wasm_bindgen(extends = HtmlElement, extends = Element, extends = Node)]
	pub type AScene;
}
