use crate::blocks::habit_block::HabitBlock;
use block_tools::{display_api::component::data::progress::ProgressComponent, models::Block};

impl HabitBlock {
	pub fn score_circle(score: Option<Block>, habit: &Block) -> ProgressComponent {
		let score = score
			.and_then(|block| {
				block
					.block_data
					.map(|data| data.parse::<i32>().unwrap_or_default())
			})
			.unwrap_or_default();
		ProgressComponent {
			inner_label: Some("Score".into()),
			color: habit.color.clone(),
			..ProgressComponent::new(score)
		}
	}
}
