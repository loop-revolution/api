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

impl StackComponent {
	pub fn new(direction: StackDirection) -> Self {
		StackComponent {
			direction,
			items: Vec::new(),
		}
	}

	pub fn append(self, component: Box<dyn DisplayComponent>) -> Self {
		let mut items = self.items;
		items.push(WrappedComponent::from(component));
		StackComponent {
			direction: self.direction,
			items,
		}
	}
}

#[derive(Serialize)]
pub enum StackDirection {
	Horizontal,
	Vertical,
	Fit,
}
