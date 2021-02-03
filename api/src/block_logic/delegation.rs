use crate::graphql::ContextData;
use block_tools::{
	display_api::{
		component::{text::TextComponent, DisplayComponent},
		CreationObject, DisplayObject,
	},
	models::Block,
	BlockError,
};

use super::{
	block::{to_blockd, BlockObject},
	block_types::BlockTypes,
};
use async_graphql::Error;
use block_tools::blocks::BlockType;
use data_block::DataBlock;
use text_block::TextBlock;

pub async fn delegate_page_display(
	block: &BlockObject,
	context: &ContextData,
) -> Result<DisplayObject, Error> {
	let block_type: BlockTypes = block.block_type.clone().into();
	Ok(match block_type {
		BlockTypes::Data => DataBlock::page_display(&to_blockd(block), &context.other()).await?,
		BlockTypes::Text => TextBlock::page_display(&to_blockd(block), &context.other()).await?,
		BlockTypes::Invalid(name) => DisplayObject::new(Box::new(
			TextComponent::new(&format!("Invalid block type '{}'", name)).color("#ff0000"),
		)),
	})
}

pub async fn delegate_embed_display(
	block: &BlockObject,
	context: &ContextData,
) -> Result<Box<dyn DisplayComponent>, Error> {
	let block_type: BlockTypes = block.block_type.clone().into();
	Ok(match block_type {
		BlockTypes::Data => DataBlock::embed_display(&to_blockd(block), &context.other()).await?,
		BlockTypes::Text => TextBlock::embed_display(&to_blockd(block), &context.other()).await?,
		BlockTypes::Invalid(name) => {
			Box::new(TextComponent::new(&format!("Invalid block type '{}'", name)).color("#ff0000"))
		}
	})
}

pub async fn delegate_create(
	block_type: &str,
	input: String,
	context: &ContextData,
	user_id: i32,
) -> Result<Block, Error> {
	let block_type: BlockTypes = block_type.to_string().into();
	Ok(match block_type {
		BlockTypes::Data => DataBlock::create(input, &context.other(), user_id).await?,
		BlockTypes::Text => TextBlock::create(input, &context.other(), user_id).await?,
		BlockTypes::Invalid(name) => return Err(BlockError::TypeExist(name).into()),
	})
}

pub async fn delegate_method(
	context: &ContextData,
	block_type: String,
	args: String,
	name: String,
	block_id: i64,
) -> Result<Block, Error> {
	let block_type: BlockTypes = block_type.into();
	Ok(match block_type {
		BlockTypes::Data => {
			DataBlock::method_delegate(&context.other(), name, block_id, args).await?
		}
		BlockTypes::Text => {
			TextBlock::method_delegate(&context.other(), name, block_id, args).await?
		}
		BlockTypes::Invalid(name) => return Err(BlockError::TypeExist(name).into()),
	})
}

pub async fn delegate_creation_display(
	context: &ContextData,
	block_type: &str,
	user_id: i32,
) -> Result<CreationObject, Error> {
	let block_type: BlockTypes = block_type.to_string().into();
	Ok(match block_type {
		BlockTypes::Data => DataBlock::create_display(&context.other(), user_id).await?,
		BlockTypes::Text => TextBlock::create_display(&context.other(), user_id).await?,
		BlockTypes::Invalid(name) => return Err(BlockError::TypeExist(name).into()),
	})
}
