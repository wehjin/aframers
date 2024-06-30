use wasm_bindgen::prelude::*;
use web_sys::{Document, window};

pub fn document() -> Document {
	let document = window()
		.and_then(|win| win.document())
		.expect("Could not access the document");
	document
}

#[wasm_bindgen]
extern "C" {
	#[wasm_bindgen(js_namespace = console)]
	pub fn log(s: &str);
}

