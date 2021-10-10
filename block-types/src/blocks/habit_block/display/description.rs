use crate::blocks::{habit_block::HabitBlock, text_block::TextBlock};
use block_tools::{display_api::component::misc::richtext::RichTextComponent, models::Block};

impl HabitBlock {
	pub fn description(description: &Option<Block>, editable: bool) -> Option<RichTextComponent> {
		if let Some(desc) = description {
			if let Some(data) = &desc.block_data {
				let content = TextBlock::data_to_display(data);
				if editable {
					return Some(TextBlock::editable_component(
						desc.id.to_string(),
						Some(content),
					));
				} else if !content.is_empty() {
					return Some(RichTextComponent {
						content,
						..Default::default()
					});
				}
			}
		}
		None
	}
}
