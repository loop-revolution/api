use crate::blocks::*;
use crate::types::BlockTypes;
use block_tools::{
	blocks::BlockType,
	display_api::component::atomic::{icon::Icon, text::TextComponent},
};
use block_tools::{
	blocks::Context,
	display_api::{component::DisplayComponent, CreationObject, DisplayObject},
	models::Block,
	BlockError, LoopError,
};

pub fn delegate_page_display(block: &Block, context: &Context) -> Result<DisplayObject, LoopError> {
	let block_type: BlockTypes = block.block_type.clone().into();
	match block_type {
		BlockTypes::Chat => chat_block::ChatBlock::page_display(block, context),
		BlockTypes::Data => data_block::DataBlock::page_display(block, context),
		BlockTypes::Text => text_block::TextBlock::page_display(block, context),
		BlockTypes::Group => group_block::GroupBlock::page_display(block, context),
		BlockTypes::Document => document_block::DocumentBlock::page_display(block, context),
		BlockTypes::Habit => habit_block::HabitBlock::page_display(block, context),
		BlockTypes::Task => task_block::TaskBlock::page_display(block, context),
		BlockTypes::Invalid(name) => Ok(DisplayObject::new(TextComponent {
			color: Some("#ff0000".to_string()),
			..TextComponent::new(format!("Invalid block type '{}'", name))
		})),
	}
}

pub fn delegate_embed_display(block: &Block, context: &Context) -> DisplayComponent {
	let block_type: BlockTypes = block.block_type.clone().into();
	match block_type {
		BlockTypes::Data => data_block::DataBlock::embed_display(block, context),
		BlockTypes::Chat => chat_block::ChatBlock::embed_display(block, context),
		BlockTypes::Text => text_block::TextBlock::embed_display(block, context),
		BlockTypes::Group => group_block::GroupBlock::embed_display(block, context),
		BlockTypes::Document => document_block::DocumentBlock::embed_display(block, context),
		BlockTypes::Habit => habit_block::HabitBlock::embed_display(block, context),
		BlockTypes::Task => task_block::TaskBlock::embed_display(block, context),
		BlockTypes::Invalid(name) => TextComponent {
			color: Some("#ff0000".to_string()),
			..TextComponent::new(format!("Invalid block type '{}'", name))
		}
		.into(),
	}
}

pub fn delegate_creation_display(
	context: &Context,
	block_type: &str,
	user_id: i32,
) -> Result<CreationObject, LoopError> {
	let block_type: BlockTypes = block_type.to_string().into();
	match block_type {
		BlockTypes::Data => data_block::DataBlock::create_display(context, user_id),
		BlockTypes::Chat => chat_block::ChatBlock::create_display(context, user_id),
		BlockTypes::Text => text_block::TextBlock::create_display(context, user_id),
		BlockTypes::Group => group_block::GroupBlock::create_display(context, user_id),
		BlockTypes::Document => document_block::DocumentBlock::create_display(context, user_id),
		BlockTypes::Habit => habit_block::HabitBlock::create_display(context, user_id),
		BlockTypes::Task => task_block::TaskBlock::create_display(context, user_id),
		BlockTypes::Invalid(name) => Err(BlockError::TypeExist(name).into()),
	}
}

pub fn delegate_block_name(
	context: &Context,
	block_type: &str,
	block: &Block,
) -> Result<String, LoopError> {
	let block_type: BlockTypes = block_type.to_string().into();
	match block_type {
		BlockTypes::Data => data_block::DataBlock::block_name(block, context),
		BlockTypes::Chat => chat_block::ChatBlock::block_name(block, context),
		BlockTypes::Text => text_block::TextBlock::block_name(block, context),
		BlockTypes::Group => group_block::GroupBlock::block_name(block, context),
		BlockTypes::Document => document_block::DocumentBlock::block_name(block, context),
		BlockTypes::Habit => habit_block::HabitBlock::block_name(block, context),
		BlockTypes::Task => task_block::TaskBlock::block_name(block, context),
		BlockTypes::Invalid(name) => Err(BlockError::TypeExist(name).into()),
	}
}

pub fn delegate_block_icon(block_type: impl ToString) -> Option<Icon> {
	let block_type: BlockTypes = block_type.to_string().into();
	Some(match block_type {
		BlockTypes::Data => data_block::DataBlock::info().icon,
		BlockTypes::Chat => chat_block::ChatBlock::info().icon,
		BlockTypes::Text => text_block::TextBlock::info().icon,
		BlockTypes::Group => group_block::GroupBlock::info().icon,
		BlockTypes::Document => document_block::DocumentBlock::info().icon,
		BlockTypes::Habit => habit_block::HabitBlock::info().icon,
		BlockTypes::Task => task_block::TaskBlock::info().icon,
		BlockTypes::Invalid(_) => return None,
	})
}
