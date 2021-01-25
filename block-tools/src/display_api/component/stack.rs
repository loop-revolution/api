use super::{DisplayComponent, WrappedComponent};
use erased_serde::Serialize as Serializable;
use serde::Serialize;

#[derive(Serialize)]
pub struct StackComponent {
	pub direction: StackDirection,
	pub items: Vec<WrappedComponent>,
}

impl DisplayComponent for StackComponent {
	fn cid(&self) -> &str {
		"stack"
	}

	fn args(&self) -> &dyn Serializable {
		self
	}
}

#[derive(Serialize)]
pub enum StackDirection {
	Horizontal,
	Vertical,
	Fit,
}
