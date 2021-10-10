use block_tools::{blocks::Context, LoopError};

use crate::blocks::task_block::TaskBlock;

impl TaskBlock {
	pub fn handle_visibility_update(
		context: &Context,
		block_id: i64,
		public: bool,
	) -> Result<(), LoopError> {
		let conn = &context.conn()?;
		let Self {
			name,
			description,
			status,
			assignee,
			..
		} = Self::from_id_admin(block_id, conn)?;

		if let Some(name) = name {
			name.update_public(public, conn)?;
		}

		if let Some(assignee) = assignee {
			assignee.update_public(public, conn)?;
		}
		if let Some(description) = description {
			description.update_public(public, conn)?;
		}
		if let Some(status) = status {
			status.update_public(public, conn)?;
		}

		Ok(())
	}
}
