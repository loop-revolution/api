use super::HabitBlock;
use block_tools::{
	blocks::{BlockType, Context},
	models::Block,
	BlockError, LoopError,
};
mod create;
mod general_perm_update;
mod minus;
mod plus;
mod visibility_update;

impl HabitBlock {
	pub fn handle_method_delegate(
		context: &Context,
		name: String,
		block_id: i64,
		_args: String,
	) -> Result<Block, LoopError> {
		match name.as_str() {
			"plus" => Self::plus_method(context, block_id),
			"minus" => Self::minus_method(context, block_id),
			_ => Err(BlockError::MethodExist(name, Self::name()).into()),
		}
	}
}
