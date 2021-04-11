use crate::display_api::{component::DisplayComponent, MethodObject};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RichTextComponent {
	/// Should only be a Text or Link component
	pub content: Vec<DisplayComponent>,
	pub editable: Option<bool>,
	pub name: Option<String>,
	pub save: Option<MethodObject>,
	pub on_enter: Option<MethodObject>,
}

impl Default for RichTextComponent {
	fn default() -> Self {
		Self {
			content: vec![],
			editable: None,
			name: None,
			save: None,
			on_enter: None,
		}
	}
}

impl From<RichTextComponent> for DisplayComponent {
	fn from(c: RichTextComponent) -> Self {
		Self::RichText(c)
	}
}
