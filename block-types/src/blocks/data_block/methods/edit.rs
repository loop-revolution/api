use block_tools::{
	auth::{
		permissions::{has_perm_level, PermLevel},
		require_token, validate_token,
	},
	blocks::Context,
	models::Block,
	BlockError, LoopError,
};

use crate::blocks::data_block::DataBlock;

impl DataBlock {
	pub fn edit_method(context: &Context, block_id: i64, args: String) -> Result<Block, LoopError> {
		let conn = &context.pool.get()?;
		let user_id = validate_token(&require_token(context)?)?;
		let access_err: LoopError =
			BlockError::TypeGenericError(format!("Cannot edit data block {}", block_id)).into();
		let block = Block::by_id(block_id, conn)?;
		let block = match block {
			Some(b) => b,
			None => return Err(access_err),
		};
		if !has_perm_level(user_id, &block, PermLevel::Edit) {
			return Err(access_err);
		}
		let mut input = args;
		input.remove(0);
		input.pop();
		let block = block.update_data(&input, conn)?;
		Ok(block)
	}
}
