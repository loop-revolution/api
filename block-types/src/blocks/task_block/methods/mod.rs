use super::TaskBlock;
use block_tools::{
	blocks::{BlockType, Context},
	models::Block,
	BlockError, LoopError,
};
mod add_dep;
mod assign;
mod create;
mod general_perm_update;
mod set_status;
mod visibility_update;

impl TaskBlock {
	pub fn handle_method_delegate(
		context: &Context,
		name: String,
		block_id: i64,
		args: String,
	) -> Result<Block, LoopError> {
		match name.as_str() {
			"set_status" => Self::set_status_method(context, block_id, args),
			"add_dep" => Self::add_method(context, block_id, args),
			"assign" => Self::assign_method(context, block_id, args),
			_ => Err(BlockError::MethodExist(name, Self::name()).into()),
		}
	}
}
