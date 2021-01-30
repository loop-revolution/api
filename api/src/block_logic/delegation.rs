use crate::graphql::{other_context, ContextData};
use block_tools::{
	display_api::{
		component::{text::TextComponent, DisplayComponent},
		CreationObject, DisplayObject,
	},
	models::Block,
	BlockError, Error as ToolsError,
};

use super::block::{to_blockd, BlockObject};
use async_graphql::Error;
use block_tools::blocks::BlockType;
use data_block::DataBlock;
use text_block::TextBlock;

pub async fn delegate_page_display(
	block: &BlockObject,
	context: &ContextData,
) -> Result<DisplayObject, Error> {
	let block_type: BlockTypes = block.block_type.clone().into();
	let wrapped = match block_type {
		BlockTypes::Data => {
			DataBlock::page_display(&to_blockd(block), &other_context(context)).await
		},
		BlockTypes::Text => {
			TextBlock::page_display(&to_blockd(block), &other_context(context)).await
		},
		BlockTypes::Invalid => {
			Ok(DisplayObject::new(Box::new(TextComponent::new("Invalid block type").color("#ff0000"))))
		},
	};
	Ok(wrapped?)
}

pub async fn delegate_embed_display(
	block: &BlockObject,
	context: &ContextData,
) -> Result<Box<dyn DisplayComponent>, Error> {
	let block_type: BlockTypes = block.block_type.clone().into();
	match block_type {
		BlockTypes::Data => {
			Ok(DataBlock::embed_display(&to_blockd(block), &other_context(context)).await?)
		}
		BlockTypes::Text => {
			Ok(TextBlock::embed_display(&to_blockd(block), &other_context(context)).await?)
		}
		BlockTypes::Invalid => Ok(Box::new(TextComponent {
			color: Some("#ff0000".into()),
			text: "Invalid block type".into(),
			preset: None,
		})),
	}
}

pub async fn delegate_create(
	block_type: &str,
	input: String,
	context: &ContextData,
	user_id: i32,
) -> Result<Block, Error> {
	let bt: BlockTypes = block_type.to_string().into();
	let wrapped = match bt {
		BlockTypes::Data => DataBlock::create(input, &other_context(context), user_id).await,
		BlockTypes::Text => TextBlock::create(input, &other_context(context), user_id).await,
		BlockTypes::Invalid => Err(ToolsError::BlockError(BlockError::TypeExist(
			block_type.to_string(),
		))),
	};
	Ok(wrapped?)
}

pub async fn delegate_creation_display(
	context: &ContextData,
	block_type: &str,
	user_id: i32,
) -> Result<CreationObject, Error> {
	let bt: BlockTypes = block_type.to_string().into();
	let wrapped = match bt {
		BlockTypes::Data => Ok(DataBlock::create_display(&other_context(context), user_id).await?),
		BlockTypes::Text => Ok(TextBlock::create_display(&other_context(context), user_id).await?),
		BlockTypes::Invalid => Err(ToolsError::BlockError(BlockError::TypeExist(
			block_type.to_string(),
		))),
	};
	Ok(wrapped?)
}

pub enum BlockTypes {
	Data,
	Text,
	Invalid,
}

impl From<String> for BlockTypes {
	fn from(s: String) -> Self {
		match s.as_str() {
			"data" => BlockTypes::Data,
			"text" => BlockTypes::Text,
			_ => BlockTypes::Invalid,
		}
	}
}