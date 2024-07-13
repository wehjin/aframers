use std::ops::Deref;

pub trait ComponentValue {
	fn component_name(&self) -> &str;
	fn component_value(&self) -> impl AsRef<str>;
}

pub trait AsPropertyValue<T: AsRef<str>> {
	fn as_property_value(&self) -> T;
}
impl<T: AsRef<str>> AsPropertyValue<T> for Box<dyn AsPropertyValue<T>> {
	fn as_property_value(&self) -> T {
		self.deref().as_property_value()
	}
}
