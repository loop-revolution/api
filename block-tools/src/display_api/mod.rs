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
	pub display: DisplayComponent,
	pub meta: Option<DisplayMeta>,
}

impl DisplayObject {
	pub fn new(component: impl Into<DisplayComponent>) -> Self {
		DisplayObject {
			display: component.into(),
			meta: None,
		}
	}
}
