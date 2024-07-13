use std::ops::Deref;

pub trait ComponentValue {
	fn component_name(&self) -> &str;
	fn component_value(&self) -> impl AsRef<str>;
}

pub trait ToPropertyValue {
	fn to_property_value(&self) -> String;
}
impl ToPropertyValue for Box<dyn ToPropertyValue> {
	fn to_property_value(&self) -> String {
		self.deref().to_property_value()
	}
}
impl ToPropertyValue for String {
	fn to_property_value(&self) -> String {
		self.clone()
	}
}
impl ToPropertyValue for bool {
	fn to_property_value(&self) -> String {
		format!("{}", self)
	}
}
impl ToPropertyValue for u32 {
	fn to_property_value(&self) -> String {
		format!("{}", self)
	}
}


