use super::GroupBlock;
use block_tools::{blocks::Context, models::Block, BlockError, LoopError};
pub mod add;
pub mod create;
pub mod remove;
pub mod root;
pub mod visibility_update;
use block_tools::blocks::BlockType;
mod general_perm_update;

impl GroupBlock {
	pub fn handle_method_delegate(
		context: &Context,
		name: String,
		block_id: i64,
		args: String,
	) -> Result<Block, LoopError> {
		match name.as_str() {
			"add" => Self::add_method(context, block_id, args),
			"remove" => Self::remove_method(context, block_id, args),
			_ => Err(BlockError::MethodExist(name, GroupBlock::name()).into()),
		}
	}
}
