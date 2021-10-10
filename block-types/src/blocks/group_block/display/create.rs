use block_tools::{
	blocks::Context,
	display_api::{
		component::{
			atomic::text::TextComponent, form::input::InputComponent, layout::stack::StackComponent,
		},
		CreationObject,
	},
	LoopError,
};

use crate::blocks::group_block::GroupBlock;
impl GroupBlock {
	pub fn handle_create_display(
		_context: &Context,
		_user_id: i32,
	) -> Result<CreationObject, LoopError> {
		let header = InputComponent {
			label: Some("Name".to_string()),
			name: Some("NAME".to_string()),
			..InputComponent::default()
		};
		let desc_input = InputComponent {
			label: Some("Description".to_string()),
			name: Some("DESC".to_string()),
			..Default::default()
		};
		let items_input = TextComponent::info("You will be able to add blocks after creation.");

		let mut main = StackComponent::vertical();
		main.push(desc_input);
		main.push(items_input);

		let template: String = r#"{
			"name": $[NAME]$,
			"desc": $[DESC]$,
			"items": []
		}"#
		.split_whitespace()
		.collect();
		let object = CreationObject {
			header_component: header.into(),
			main_component: main.into(),
			input_template: template,
		};
		Ok(object)
	}
}
