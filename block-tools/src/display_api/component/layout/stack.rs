use crate::display_api::component::{DisplayComponent, WrappedComponent};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StackComponent {
	pub direction: Option<StackDirection>,
	pub items: Vec<WrappedComponent>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum StackDirection {
	Horizontal,
	Vertical,
	Fit,
}

impl Default for StackComponent {
	fn default() -> Self {
		Self {
			direction: None,
			items: vec![],
		}
	}
}

impl StackComponent {
	pub fn new(items: Vec<WrappedComponent>) -> Self {
		Self {
			items,
			..Default::default()
		}
	}
}

impl StackComponent {
	pub fn push(&mut self, component: DisplayComponent) {
		self.items.push(WrappedComponent::from(component));
	}
}

impl StackComponent {
	pub fn fit() -> Self {
		Self {
			direction: Some(StackDirection::Fit),
			items: vec![],
		}
	}
	pub fn vertical() -> Self {
		Self {
			direction: Some(StackDirection::Vertical),
			items: vec![],
		}
	}
	pub fn horizontal() -> Self {
		Self {
			direction: Some(StackDirection::Horizontal),
			items: vec![],
		}
	}
}

impl From<StackComponent> for DisplayComponent {
    fn from(c: StackComponent) -> Self {
        Self::Stack(c)
    }
}
