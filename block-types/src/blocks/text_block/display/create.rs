use block_tools::{
	display_api::{
		component::{atomic::text::TextComponent, misc::richtext::RichTextComponent},
		CreationObject,
	},
	LoopError,
};

use super::super::TextBlock;

impl TextBlock {
	pub fn handle_create_display() -> Result<CreationObject, LoopError> {
		let header = TextComponent::heading("New Text Block");
		let main = RichTextComponent {
			editable: Some(true),
			name: Some("DATA".into()),
			..RichTextComponent::default()
		};
		let object = CreationObject {
			header_component: header.into(),
			main_component: main.into(),
			input_template: r#"{"content":$[DATA]$}"#.into(),
		};
		Ok(object)
	}
}
