use block_tools::{blocks::Context, LoopError};

use crate::blocks::group_block::GroupBlock;

impl GroupBlock {
	pub fn handle_visibility_update(
		context: &Context,
		block_id: i64,
		public: bool,
	) -> Result<(), LoopError> {
		let conn = &context.conn()?;
		let Self {
			name, description, ..
		} = Self::from_id_admin(block_id, conn)?;

		if let Some(name) = name {
			name.update_public(public, conn)?;
		}

		if let Some(desc) = description {
			desc.update_public(public, conn)?;
		}

		Ok(())
	}
}
