use block_tools::{
	blocks::BlockType,
	display_api::{
		component::{misc::richtext::RichTextComponent, DisplayComponent},
		MethodObject,
	},
};

use crate::blocks::text_block::TextBlock;

impl TextBlock {
	/// A re-usable richtext component
	pub fn editable_component(
		block_id: String,
		value: Option<Vec<DisplayComponent>>,
	) -> RichTextComponent {
		let name = format!("DATA{}", block_id);
		let method = MethodObject {
			block_type: Self::name(),
			arg_template: format!(r#"{{"content":$[{}]$}}"#, name),
			block_id,
			method_name: "edit".into(),
		};
		let mut field = RichTextComponent {
			editable: Some(true),
			name: Some(name),
			save: Some(method),
			..RichTextComponent::default()
		};
		if let Some(value) = value {
			field.content = value
		}
		field
	}
}
