use crate::blocks::task_block::TaskBlock;
use block_tools::{
	display_api::component::form::dropdown::{DropdownComponent, DropdownOption},
	models::Block,
};

impl TaskBlock {
	pub fn status(status: &Option<Block>, block_id: i64) -> DropdownComponent {
		let status_index = Self::status_index(status);
		DropdownComponent {
			options: vec![
				DropdownOption::new("Not Started"),
				DropdownOption::new("In Progress"),
				DropdownOption::new("Done"),
			],
			name: Some("STATUS".to_string()),
			default: Some(status_index),
			on_change: Some(Self::build_set_status_action_object(block_id)),
			..DropdownComponent::default()
		}
	}
	pub fn status_index(status: &Option<Block>) -> u8 {
		status
			.as_ref()
			.and_then(|block| block.block_data.clone())
			.unwrap_or_default()
			.parse::<u8>()
			.unwrap_or_default()
	}
}
