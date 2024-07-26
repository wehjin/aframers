use crate::components::core::{ComponentAttribute, ComponentSetting, ToPropertyValue};

pub mod core;

#[derive(Debug, Clone)]
pub struct Light {
	pub color: Color,
	pub intensity: f32,
}

impl ComponentSetting for Light {
	fn as_setting_name(&self) -> impl AsRef<str> { "light" }
	fn as_setting_str(&self) -> impl AsRef<str> {
		format!("color: {}; intensity: {}", self.color.as_setting_str().as_ref(), self.intensity)
	}
}

#[derive(Debug, Copy, Clone, Default)]
pub struct Wireframe(pub bool);

impl ComponentSetting for Wireframe {
	fn as_setting_name(&self) -> impl AsRef<str> { "wireframe" }
	fn as_setting_str(&self) -> impl AsRef<str> {
		match self.0 {
			true => "true",
			false => "false"
		}
	}
}

#[derive(Debug, Copy, Clone, Default)]
pub struct RadiusTop(pub f32);

impl ComponentSetting for RadiusTop {
	fn as_setting_name(&self) -> impl AsRef<str> { "radius-top" }
	fn as_setting_str(&self) -> impl AsRef<str> {
		format!("{}", self.0)
	}
}

#[derive(Debug, Copy, Clone, Default)]
pub struct RadiusBottom(pub f32);

impl ComponentSetting for RadiusBottom {
	fn as_setting_name(&self) -> impl AsRef<str> { "radius-bottom" }
	fn as_setting_str(&self) -> impl AsRef<str> {
		format!("{}", self.0)
	}
}

#[derive(Debug, Copy, Clone, Default)]
pub struct SegmentsRadial(pub u32);

impl ComponentSetting for SegmentsRadial {
	fn as_setting_name(&self) -> impl AsRef<str> { "segments-radial" }
	fn as_setting_str(&self) -> impl AsRef<str> {
		format!("{}", self.0)
	}
}

#[derive(Debug, Copy, Clone, Default)]
pub struct SegmentsHeight(pub u32);

impl ComponentSetting for SegmentsHeight {
	fn as_setting_name(&self) -> impl AsRef<str> { "segments-height" }
	fn as_setting_str(&self) -> impl AsRef<str> {
		format!("{}", self.0)
	}
}

#[derive(Debug, Copy, Clone, Default)]
pub struct ThetaLength(pub f32);

impl ComponentSetting for ThetaLength {
	fn as_setting_name(&self) -> impl AsRef<str> { "theta-length" }
	fn as_setting_str(&self) -> impl AsRef<str> {
		format!("{}", self.0)
	}
}

#[derive(Debug, Copy, Clone, Default)]
pub struct ThetaStart(pub f32);

impl ComponentSetting for ThetaStart {
	fn as_setting_name(&self) -> impl AsRef<str> { "theta-start" }
	fn as_setting_str(&self) -> impl AsRef<str> {
		format!("{}", self.0)
	}
}

#[derive(Debug, Copy, Clone, Default)]
pub struct Width(pub f32);
impl ComponentSetting for Width {
	fn as_setting_name(&self) -> impl AsRef<str> { "width" }
	fn as_setting_str(&self) -> impl AsRef<str> {
		format!("{}", self.0)
	}
}

#[derive(Debug, Copy, Clone, Default)]
pub struct Depth(pub f32);

impl ComponentSetting for Depth {
	fn as_setting_name(&self) -> impl AsRef<str> { "depth" }
	fn as_setting_str(&self) -> impl AsRef<str> {
		format!("{}", self.0)
	}
}


#[derive(Debug, Copy, Clone, Default)]
pub struct Height(pub f32);

impl ComponentSetting for Height {
	fn as_setting_name(&self) -> impl AsRef<str> { "height" }
	fn as_setting_str(&self) -> impl AsRef<str> {
		format!("{}", self.0)
	}
}


#[derive(Debug, Copy, Clone, Default)]
pub struct Radius(pub f32);

impl ComponentSetting for Radius {
	fn as_setting_name(&self) -> impl AsRef<str> { "radius" }
	fn as_setting_str(&self) -> impl AsRef<str> {
		format!("{}", self.0)
	}
}

#[derive(Debug, Copy, Clone, Default)]
pub struct Rotation(pub f32, pub f32, pub f32);

impl ComponentSetting for Rotation {
	fn as_setting_name(&self) -> impl AsRef<str> { "rotation" }
	fn as_setting_str(&self) -> impl AsRef<str> {
		format!("{} {} {}", self.0, self.1, self.2)
	}
}

#[derive(Debug, Copy, Clone, Default)]
pub struct Scale(pub f32, pub f32, pub f32);

impl ComponentSetting for Scale {
	fn as_setting_name(&self) -> impl AsRef<str> { "scale" }
	fn as_setting_str(&self) -> impl AsRef<str> {
		format!("{} {} {}", self.0, self.1, self.2)
	}
}

#[derive(Debug, Copy, Clone, Default)]
pub struct Position(pub f32, pub f32, pub f32);

impl ToPropertyValue for Position {
	fn to_property_value(&self) -> String {
		format!("{} {} {}", self.0, self.1, self.2)
	}
}

impl ComponentSetting for Position {
	fn as_setting_name(&self) -> impl AsRef<str> { "position" }
	fn as_setting_str(&self) -> impl AsRef<str> {
		self.to_property_value()
	}
}

#[derive(Debug, Clone)]
pub enum Color {
	Web(String),
	WebStr(&'static str),
}

impl ComponentSetting for Color {
	fn as_setting_name(&self) -> impl AsRef<str> { "color" }
	fn as_setting_str(&self) -> impl AsRef<str> {
		match self {
			Color::Web(s) => s,
			Color::WebStr(s) => *s,
		}
	}
}

impl ComponentAttribute for Color {
	fn as_attribute_name(&self) -> impl AsRef<str> {
		self.as_setting_name()
	}

	fn as_attribute_str(&self) -> impl AsRef<str> {
		self.as_setting_str()
	}
}
