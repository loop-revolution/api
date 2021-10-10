use block_tools::{blocks::Context, LoopError};

use crate::blocks::habit_block::HabitBlock;

impl HabitBlock {
	pub fn handle_general_perm_update(
		context: &Context,
		block_id: i64,
		perm_full: Vec<i32>,
		perm_edit: Vec<i32>,
		perm_view: Vec<i32>,
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
			name.update_perms(
				perm_full.clone(),
				perm_edit.clone(),
				perm_view.clone(),
				conn,
			)?;
		}
		if let Some(description) = description {
			description.update_perms(
				perm_full.clone(),
				perm_edit.clone(),
				perm_view.clone(),
				conn,
			)?;
		}
		if let Some(impact) = impact {
			impact.update_perms(
				perm_full.clone(),
				perm_edit.clone(),
				perm_view.clone(),
				conn,
			)?;
		}
		if let Some(score) = score {
			score.update_perms(
				perm_full.clone(),
				perm_edit.clone(),
				perm_view.clone(),
				conn,
			)?;
		}
		if let Some(streak) = streak {
			streak.update_perms(perm_full, perm_edit, perm_view, conn)?;
		}

		Ok(())
	}
}
