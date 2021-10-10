use crate::blocks::habit_block::HabitBlock;
use block_tools::{display_api::component::atomic::text::TextComponent, models::Block};

impl HabitBlock {
	pub fn streak(streak: Option<Block>) -> TextComponent {
		let streak = streak
			.and_then(|block| block.block_data)
			.unwrap_or_else(|| "0".into());
		TextComponent::new(format!("Streak: {} days", streak))
	}
}
