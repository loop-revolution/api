use component::DisplayComponent;
use serde::Serialize;
pub mod component;
pub mod method;
pub use method::*;
pub mod meta;
pub use meta::*;
pub mod action;
pub use action::*;
pub mod colors;

#[derive(Serialize)]
pub struct DisplayObject {
	pub display: Box<dyn DisplayComponent>,
	pub meta: Option<DisplayMeta>,
}

impl DisplayObject {
	pub fn new(component: Box<dyn DisplayComponent>) -> Self {
		DisplayObject {
			display: component,
			meta: None,
		}
	}

	pub fn meta(self, meta: DisplayMeta) -> Self {
		DisplayObject {
			meta: Some(meta),
			..self
		}
	}
}
