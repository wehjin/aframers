use core::ComponentValue;

use crate::components::core::ToPropertyValue;

pub mod core;

#[derive(Debug, Clone)]
pub struct Light {
	pub(crate) color: Color,
	pub(crate) intensity: f32,
}

impl ComponentValue for Light {
	fn component_name(&self) -> &str { "light" }

	fn component_value(&self) -> impl AsRef<str> {
		format!("color: {}; intensity: {}", self.color.component_value().as_ref(), self.intensity)
	}
}

#[derive(Debug, Copy, Clone, Default)]
pub struct Wireframe(pub bool);

impl ComponentValue for Wireframe {
	fn component_name(&self) -> &str { "wireframe" }

	fn component_value(&self) -> impl AsRef<str> {
		if self.0 {
			"true"
		} else {
			"false"
		}
	}
}

#[derive(Debug, Copy, Clone, Default)]
pub struct RadiusTop(pub f32);

impl ComponentValue for RadiusTop {
	fn component_name(&self) -> &str { "radius-top" }

	fn component_value(&self) -> impl AsRef<str> {
		format!("{}", self.0)
	}
}

#[derive(Debug, Copy, Clone, Default)]
pub struct RadiusBottom(pub f32);

impl ComponentValue for RadiusBottom {
	fn component_name(&self) -> &str { "radius-bottom" }

	fn component_value(&self) -> impl AsRef<str> {
		format!("{}", self.0)
	}
}

#[derive(Debug, Copy, Clone, Default)]
pub struct SegmentsRadial(pub u32);

impl ComponentValue for SegmentsRadial {
	fn component_name(&self) -> &str { "segments-radial" }

	fn component_value(&self) -> impl AsRef<str> {
		format!("{}", self.0)
	}
}

#[derive(Debug, Copy, Clone, Default)]
pub struct SegmentsHeight(pub u32);

impl ComponentValue for SegmentsHeight {
	fn component_name(&self) -> &str { "segments-height" }

	fn component_value(&self) -> impl AsRef<str> {
		format!("{}", self.0)
	}
}

#[derive(Debug, Copy, Clone, Default)]
pub struct ThetaLength(pub f32);

impl ComponentValue for ThetaLength {
	fn component_name(&self) -> &str { "theta-length" }

	fn component_value(&self) -> impl AsRef<str> {
		format!("{}", self.0)
	}
}

#[derive(Debug, Copy, Clone, Default)]
pub struct ThetaStart(pub f32);

impl ComponentValue for ThetaStart {
	fn component_name(&self) -> &str { "theta-start" }

	fn component_value(&self) -> impl AsRef<str> {
		format!("{}", self.0)
	}
}

#[derive(Debug, Copy, Clone, Default)]
pub struct Width(pub f32);

impl ComponentValue for Width {
	fn component_name(&self) -> &str { "width" }

	fn component_value(&self) -> impl AsRef<str> {
		format!("{}", self.0)
	}
}

#[derive(Debug, Copy, Clone, Default)]
pub struct Depth(pub f32);

impl ComponentValue for Depth {
	fn component_name(&self) -> &str { "depth" }

	fn component_value(&self) -> impl AsRef<str> {
		format!("{}", self.0)
	}
}


#[derive(Debug, Copy, Clone, Default)]
pub struct Height(pub f32);

impl ComponentValue for Height {
	fn component_name(&self) -> &str { "height" }

	fn component_value(&self) -> impl AsRef<str> {
		format!("{}", self.0)
	}
}


#[derive(Debug, Copy, Clone, Default)]
pub struct Radius(pub f32);

impl ComponentValue for Radius {
	fn component_name(&self) -> &str { "radius" }

	fn component_value(&self) -> impl AsRef<str> {
		format!("{}", self.0)
	}
}

#[derive(Debug, Copy, Clone, Default)]
pub struct Rotation(pub f32, pub f32, pub f32);

impl ComponentValue for Rotation {
	fn component_name(&self) -> &str { "rotation" }

	fn component_value(&self) -> impl AsRef<str> {
		format!("{} {} {}", self.0, self.1, self.2)
	}
}

#[derive(Debug, Copy, Clone, Default)]
pub struct Scale(pub f32, pub f32, pub f32);

impl ComponentValue for Scale {
	fn component_name(&self) -> &str { "scale" }

	fn component_value(&self) -> impl AsRef<str> {
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

impl ComponentValue for Position {
	fn component_name(&self) -> &str { "position" }

	fn component_value(&self) -> impl AsRef<str> {
		self.to_property_value()
	}
}

#[derive(Debug, Clone)]
pub enum Color {
	Web(String)
}

impl ComponentValue for Color {
	fn component_name(&self) -> &str { "color" }

	fn component_value(&self) -> impl AsRef<str> {
		match self {
			Color::Web(s) => s
		}
	}
}