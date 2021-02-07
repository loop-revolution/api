use crate::{blocks::Context, models::Block, UserError};

use super::{optional_token, optional_validate_token};

pub fn can_view(user_id: Option<i32>, block: &Block) -> bool {
	let mut allowed = block.public || Some(block.owner_id) == user_id;
	if !allowed {
		if let Some(user_id) = user_id {
			if block.perm_view.contains(&user_id)
				|| block.perm_edit.contains(&user_id)
				|| block.perm_full.contains(&user_id)
			{
				allowed = true
			}
		}
	}
	if allowed {
		true
	} else {
		false
	}
}

pub fn use_view(context: &Context, block: Block) -> Result<Option<Block>, UserError> {
	let token = optional_token(context);
	let user_id = optional_validate_token(token)?;
	if can_view(user_id, &block) {
		Ok(Some(block))
	} else {
		Ok(None)
	}
}

pub fn maybe_use_view(context: &Context, block: Option<Block>) -> Result<Option<Block>, UserError> {
	match block {
		Some(block) => use_view(context, block),
		None => Ok(None),
	}
}

pub fn has_perm_level(user_id: i32, block: &Block, level: PermLevel) -> bool {
	if block.owner_id == user_id {
		return true;
	}
	if let PermLevel::Owner = level {
		return false;
	}
	if block.perm_full.contains(&user_id) {
		return true;
	}
	if let PermLevel::Full = level {
		return false;
	}
	if block.perm_edit.contains(&user_id) {
		return true;
	}
	if let PermLevel::Edit = level {
		return false;
	}
	if block.perm_view.contains(&user_id) {
		return true;
	}
	false
}

pub enum PermLevel {
	View,
	Edit,
	Full,
	Owner,
}
