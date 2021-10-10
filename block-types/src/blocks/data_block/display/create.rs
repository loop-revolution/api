use block_tools::{
	display_api::{
		component::{atomic::text::TextComponent, form::input::InputComponent},
		CreationObject,
	},
	LoopError,
};

use crate::blocks::data_block::DataBlock;

impl DataBlock {
	pub fn handle_create_display() -> Result<CreationObject, LoopError> {
		let header = TextComponent::heading("New Data Block");

		let main = InputComponent {
			label: Some("Data".to_string()),
			name: Some("DATA".to_string()),
			..Default::default()
		};

		let object = CreationObject {
			header_component: header.into(),
			main_component: main.into(),
			input_template: "$[DATA]$".into(),
		};
		Ok(object)
	}
}
