use crate::{blocks::Context, models::Block, UserError};

use super::{optional_token, optional_validate_token};

pub fn can_view(user_id: Option<i32>, block: &Block) -> bool {
	if block.public || Some(block.owner_id) == user_id {
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
