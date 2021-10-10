use super::TextBlock;
use block_tools::{blocks::Context, models::Block, BlockError, LoopError};
pub mod create;
mod edit;
use block_tools::blocks::BlockType;

impl TextBlock {
	pub fn handle_method_delegate(
		context: &Context,
		name: String,
		block_id: i64,
		args: String,
	) -> Result<Block, LoopError> {
		match name.as_str() {
			"edit" => Self::edit_method(context, block_id, args),
			_ => Err(BlockError::MethodExist(name, Self::name()).into()),
		}
	}
}
