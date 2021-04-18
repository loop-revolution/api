use crate::blocks::*;
use crate::types::BlockTypes;
use block_tools::blocks::BlockType;
use block_tools::{blocks::Context, models::Block, BlockError, LoopError};

pub fn delegate_create(
	block_type: &str,
	input: String,
	context: &Context,
	user_id: i32,
) -> Result<Block, LoopError> {
	let block_type: BlockTypes = block_type.to_string().into();
	match block_type {
		BlockTypes::Data => data_block::DataBlock::create(input, context, user_id),
		BlockTypes::Text => text_block::TextBlock::create(input, context, user_id),
		BlockTypes::Group => group_block::GroupBlock::create(input, context, user_id),
		BlockTypes::Document => document_block::DocumentBlock::create(input, context, user_id),
		BlockTypes::Habit => habit_block::HabitBlock::create(input, context, user_id),
		BlockTypes::Invalid(name) => Err(BlockError::TypeExist(name).into()),
	}
}

pub fn delegate_method(
	context: &Context,
	block_type: String,
	args: String,
	name: String,
	block_id: i64,
) -> Result<Block, LoopError> {
	let block_type: BlockTypes = block_type.into();
	match block_type {
		BlockTypes::Data => data_block::DataBlock::method_delegate(context, name, block_id, args),
		BlockTypes::Text => text_block::TextBlock::method_delegate(context, name, block_id, args),
		BlockTypes::Habit => {
			habit_block::HabitBlock::method_delegate(context, name, block_id, args)
		}
		BlockTypes::Document => {
			document_block::DocumentBlock::method_delegate(context, name, block_id, args)
		}
		BlockTypes::Group => {
			group_block::GroupBlock::method_delegate(context, name, block_id, args)
		}
		BlockTypes::Invalid(name) => Err(BlockError::TypeExist(name).into()),
	}
}

pub fn delegate_visibility_update(
	context: &Context,
	block_type: &str,
	block_id: i64,
	public: bool,
) -> Result<(), LoopError> {
	let block_type: BlockTypes = block_type.to_string().into();
	match block_type {
		BlockTypes::Data => data_block::DataBlock::visibility_update(context, block_id, public),
		BlockTypes::Text => text_block::TextBlock::visibility_update(context, block_id, public),
		BlockTypes::Group => group_block::GroupBlock::visibility_update(context, block_id, public),
		BlockTypes::Habit => habit_block::HabitBlock::visibility_update(context, block_id, public),
		BlockTypes::Document => {
			document_block::DocumentBlock::visibility_update(context, block_id, public)
		}
		BlockTypes::Invalid(name) => Err(BlockError::TypeExist(name).into()),
	}
}
