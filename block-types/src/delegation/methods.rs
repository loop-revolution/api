use crate::blocks::*;
use crate::types::BlockTypes;
use block_tools::blocks::BlockType;
use block_tools::{blocks::Context, models::Block, BlockError, Error};

pub fn delegate_create(
	block_type: &str,
	input: String,
	context: &Context,
	user_id: i32,
) -> Result<Block, Error> {
	let block_type: BlockTypes = block_type.to_string().into();
	match block_type {
		BlockTypes::Data => data_block::DataBlock::create(input, context, user_id),
		BlockTypes::Text => text_block::TextBlock::create(input, context, user_id),
		BlockTypes::Group => group_block::GroupBlock::create(input, context, user_id),
		BlockTypes::Invalid(name) => Err(BlockError::TypeExist(name).into()),
	}
}

pub fn delegate_method(
	context: &Context,
	block_type: String,
	args: String,
	name: String,
	block_id: i64,
) -> Result<Block, Error> {
	let block_type: BlockTypes = block_type.into();
	match block_type {
		BlockTypes::Data => data_block::DataBlock::method_delegate(context, name, block_id, args),
		BlockTypes::Text => text_block::TextBlock::method_delegate(context, name, block_id, args),
		BlockTypes::Group => {
			group_block::GroupBlock::method_delegate(context, name, block_id, args)
		}
		BlockTypes::Invalid(name) => Err(BlockError::TypeExist(name).into()),
	}
}
