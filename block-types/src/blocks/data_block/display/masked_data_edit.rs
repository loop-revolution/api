use crate::blocks::data_block::DataBlock;
use block_tools::display_api::MethodObject;
use block_tools::{
	blocks::BlockType,
	display_api::component::{
		atomic::text::{TextComponent, TextPreset},
		form::input::InputComponent,
	},
};

impl DataBlock {
	/// A re-usable masked data input
	pub fn masked_editable_data(
		block_id: String,
		value: Option<String>,
		heading: bool,
	) -> InputComponent {
		let name = format!("DATA{}", block_id);
		let method = MethodObject {
			block_type: DataBlock::name(),
			arg_template: format!("$[{}]$", name),
			block_id,
			method_name: "edit".into(),
		};
		let mut input = InputComponent {
			name: Some(name),
			..Default::default()
		};
		input.with_confirm(method.into());
		if let Some(value) = value {
			let mut mask = TextComponent::new(value.clone());
			if heading {
				mask.preset = Some(TextPreset::Heading);
			}
			input.initial_value = Some(value);
			input.mask = Some(mask);
		}
		input
	}
}
