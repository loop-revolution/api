use block_tools::{blocks::Context, LoopError};

use crate::blocks::habit_block::HabitBlock;

impl HabitBlock {
	pub fn handle_visibility_update(
		context: &Context,
		block_id: i64,
		public: bool,
	) -> Result<(), LoopError> {
		let conn = &context.conn()?;
		let Self {
			name,
			description,
			impact,
			score,
			streak,
		} = Self::from_id_admin(block_id, conn)?;

		if let Some(name) = name {
			name.update_public(public, conn)?;
		}
		if let Some(description) = description {
			description.update_public(public, conn)?;
		}
		if let Some(impact) = impact {
			impact.update_public(public, conn)?;
		}
		if let Some(score) = score {
			score.update_public(public, conn)?;
		}
		if let Some(streak) = streak {
			streak.update_public(public, conn)?;
		}

		Ok(())
	}
}
