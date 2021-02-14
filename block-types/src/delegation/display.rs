use crate::blocks::*;
use crate::types::BlockTypes;
use block_tools::blocks::BlockType;
use block_tools::{
	blocks::Context,
	display_api::{
		component::{text::TextComponent, DisplayComponent},
		CreationObject, DisplayObject,
	},
	models::Block,
	BlockError, Error,
};

pub fn delegate_page_display(block: &Block, context: &Context) -> Result<DisplayObject, Error> {
	let block_type: BlockTypes = block.block_type.clone().into();
	match block_type {
		BlockTypes::Data => data_block::DataBlock::page_display(block, context),
		BlockTypes::Text => text_block::TextBlock::page_display(block, context),
		BlockTypes::Group => group_block::GroupBlock::page_display(block, context),
		BlockTypes::Invalid(name) => Ok(DisplayObject::new(Box::new(
			TextComponent::new(&format!("Invalid block type '{}'", name)).color("#ff0000"),
		))),
	}
}

pub fn delegate_embed_display(block: &Block, context: &Context) -> Box<dyn DisplayComponent> {
	let block_type: BlockTypes = block.block_type.clone().into();
	match block_type {
		BlockTypes::Data => data_block::DataBlock::embed_display(block, context),
		BlockTypes::Text => text_block::TextBlock::embed_display(block, context),
		BlockTypes::Group => group_block::GroupBlock::embed_display(block, context),
		BlockTypes::Invalid(name) => {
			Box::new(TextComponent::new(&format!("Invalid block type '{}'", name)).color("#ff0000"))
		}
	}
}

pub fn delegate_creation_display(
	context: &Context,
	block_type: &str,
	user_id: i32,
) -> Result<CreationObject, Error> {
	let block_type: BlockTypes = block_type.to_string().into();
	match block_type {
		BlockTypes::Data => data_block::DataBlock::create_display(context, user_id),
		BlockTypes::Text => text_block::TextBlock::create_display(context, user_id),
		BlockTypes::Group => group_block::GroupBlock::create_display(context, user_id),
		BlockTypes::Invalid(name) => Err(BlockError::TypeExist(name).into()),
	}
}

pub fn delegate_block_name(
	context: &Context,
	block_type: &str,
	block: &Block,
) -> Result<String, Error> {
	let block_type: BlockTypes = block_type.to_string().into();
	match block_type {
		BlockTypes::Data => data_block::DataBlock::block_name(block, context),
		BlockTypes::Text => text_block::TextBlock::block_name(block, context),
		BlockTypes::Group => group_block::GroupBlock::block_name(block, context),
		BlockTypes::Invalid(name) => Err(BlockError::TypeExist(name).into()),
	}
}
