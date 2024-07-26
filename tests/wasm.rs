use wasm_bindgen_test::wasm_bindgen_test;

use aframers::af_sys::entities::AEntity;
use aframers::components::core::ComponentAttribute;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

pub struct MyAttr;
impl ComponentAttribute for MyAttr {
	fn as_attribute_name(&self) -> impl AsRef<str> { "myAttr" }
	fn as_attribute_str(&self) -> impl AsRef<str> { "1" }
}

#[wasm_bindgen_test]
fn exercise_set_component_attribute() {
	let entity = AEntity::new();
	entity.set_component_attribute(MyAttr);
	assert_eq!(entity.get_attribute("myAttr"), Some("1".into()));
}