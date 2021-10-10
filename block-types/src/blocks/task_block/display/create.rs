use block_tools::{
	blocks::Context,
	display_api::{
		component::{
			atomic::text::TextComponent,
			form::input::{InputComponent, InputSize},
			layout::stack::StackComponent,
			misc::richtext::RichTextComponent,
		},
		CreationObject,
	},
	LoopError,
};

use crate::blocks::task_block::TaskBlock;

impl TaskBlock {
	pub fn handle_create_display(
		_context: &Context,
		_user_id: i32,
	) -> Result<CreationObject, LoopError> {
		let header = InputComponent {
			label: Some("Summary".to_string()),
			name: Some("NAME".to_string()),
			size: Some(InputSize::Large),
			..InputComponent::default()
		};
		let mut desc_section = StackComponent::vertical();
		let desc_input = RichTextComponent {
			editable: Some(true),
			name: Some("DESC".to_string()),
			bordered: Some(true),
			..Default::default()
		};
		desc_section.push(TextComponent {
			bold: Some(true),
			..TextComponent::new("Description")
		});
		desc_section.push(desc_input);

		let mut main = StackComponent::vertical();
		main.push(desc_section);

		let template: String = r#"{"name": $[NAME]$,"desc":$[DESC]$}"#.to_string();

		let object = CreationObject {
			header_component: header.into(),
			main_component: main.into(),
			input_template: template,
		};
		Ok(object)
	}
}
