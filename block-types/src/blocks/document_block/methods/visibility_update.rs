use block_tools::{blocks::Context, LoopError};

use crate::blocks::document_block::DocumentBlock;

impl DocumentBlock {
	pub fn handle_visibility_update(
		context: &Context,
		block_id: i64,
		public: bool,
	) -> Result<(), LoopError> {
		let conn = &context.conn()?;
		let Self { name, items } = Self::from_id_admin(block_id, conn)?;

		if let Some(name) = name {
			name.update_public(public, conn)?;
		}

		for item in items {
			if item.block_type == "text" {
				item.update_public(public, conn)?;
			}
		}

		Ok(())
	}
}
