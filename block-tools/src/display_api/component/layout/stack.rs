use crate::display_api::component::{DisplayComponent, WrappedComponent};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StackComponent {
	pub direction: Option<StackDirection>,
	pub items: Vec<WrappedComponent>,
	pub align_y: Option<AlignYOptions>,
	pub align_x: Option<AlignXOptions>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum StackDirection {
	Horizontal,
	Vertical,
	Fit,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum AlignYOptions {
	Top,
	Middle,
	Bottom,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum AlignXOptions {
	Left,
	Middle,
	Right,
}

impl Default for StackComponent {
	fn default() -> Self {
		Self {
			direction: None,
			items: vec![],
			align_y: None,
			align_x: None,
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
	pub fn push(&mut self, component: impl Into<DisplayComponent>) {
		self.items.push(WrappedComponent::from(component.into()));
	}
}

impl StackComponent {
	pub fn fit() -> Self {
		Self {
			direction: Some(StackDirection::Fit),
			..Default::default()
		}
	}
	pub fn vertical() -> Self {
		Self {
			direction: Some(StackDirection::Vertical),
			..Default::default()
		}
	}
	pub fn horizontal() -> Self {
		Self {
			direction: Some(StackDirection::Horizontal),
			..Default::default()
		}
	}
}

impl From<StackComponent> for DisplayComponent {
	fn from(c: StackComponent) -> Self {
		Self::Stack(c)
	}
}
