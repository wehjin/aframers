use std::ops::Deref;

/// Structs with the trait can be applied to an AEntity to add or update
/// a component on the entity.
pub trait ComponentAttribute {
	/// The name of the attribute.
	fn as_attribute_name(&self) -> impl AsRef<str>;
	/// The attribute's value
	fn as_attribute_str(&self) -> impl AsRef<str>;
}


#[deprecated(note = "use ComponentAttribute instead")]
pub trait ComponentValue {
	fn component_name(&self) -> &str;
	fn component_value(&self) -> impl AsRef<str>;
	fn as_attribute_str(&self) -> impl AsRef<str> {
		format!("{}: {}", self.component_name(), self.component_value().as_ref())
	}
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


