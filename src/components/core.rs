use std::ops::Deref;

/// Structs with the trait can be applied to an AEntity to add or update
/// a component on the entity.
pub trait ComponentAttribute {
	/// The name of the attribute.
	fn as_attribute_name(&self) -> impl AsRef<str>;
	/// The attribute's value
	fn as_attribute_str(&self) -> impl AsRef<str>;
}

/// Allows struct with the trait to be combined with others
/// to form a multi-property component's attribute.
pub trait ComponentSetting {
	/// The name of the setting.
	fn as_setting_name(&self) -> impl AsRef<str>;

	/// The value of the setting as a str.
	fn as_setting_str(&self) -> impl AsRef<str>;

	/// The name and value of the setting in one string.
	fn as_setting_declaration(&self) -> String {
		format!("{}: {}", self.as_setting_name().as_ref(), self.as_setting_str().as_ref())
	}
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


