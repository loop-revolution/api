use block_tools::{blocks::Context, LoopError};

use crate::blocks::document_block::DocumentBlock;

impl DocumentBlock {
	pub fn handle_general_perm_update(
		context: &Context,
		block_id: i64,
		perm_full: Vec<i32>,
		perm_edit: Vec<i32>,
		perm_view: Vec<i32>,
	) -> Result<(), LoopError> {
		let conn = &context.conn()?;
		let Self { name, items } = Self::from_id_admin(block_id, conn)?;

		for item in items {
			if item.block_type == "text" {
				item.update_perms(
					perm_full.clone(),
					perm_edit.clone(),
					perm_view.clone(),
					conn,
				)?;
			}
		}

		if let Some(name) = name {
			name.update_perms(perm_full, perm_edit, perm_view, conn)?;
		}

		Ok(())
	}
}
