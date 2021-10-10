use crate::blocks::{
	data_block,
	task_block::{TaskBlock, BLOCK_NAME},
	text_block::{self, data_convert::ComponentStruct},
};
use block_tools::{
	blocks::Context,
	display_api::component::DisplayComponent,
	models::{Block, NewBlock},
	BlockError, LoopError,
};
use serde::{Deserialize, Serialize};

impl TaskBlock {
	pub fn handle_create_raw(
		input: String,
		context: &Context,
		user_id: i32,
	) -> Result<Block, LoopError> {
		let input = serde_json::from_str::<CreationArgs>(&input);
		let input: CreationArgs = input.map_err(|e| {
			println!("Erro: {:?}", e);
			BlockError::InputParse
		})?;

		Self::handle_create(input, context, user_id)
	}
}

impl TaskBlock {
	pub fn handle_create(
		input: CreationArgs,
		context: &Context,
		user_id: i32,
	) -> Result<Block, LoopError> {
		let conn = &context.conn()?;

		let block = NewBlock::new(BLOCK_NAME, user_id).insert(conn)?;

		if let Some(name) = input.name {
			let name_block = NewBlock {
				block_data: Some(name),
				..NewBlock::new(data_block::BLOCK_NAME, user_id)
			}
			.insert(conn)?;
			block.make_property("name", name_block.id).insert(conn)?;
		}

		if let Some(desc) = input.desc {
			let display_vec: Vec<DisplayComponent> = desc
				.into_iter()
				.map(|component| component.args.into())
				.collect();
			let desc_block =
				text_block::TextBlock::handle_create_vec(display_vec, context, user_id)?;
			block.make_property("desc", desc_block.id).insert(conn)?;
		}

		let status_block = NewBlock {
			block_data: Some("0".into()),
			..NewBlock::new(data_block::BLOCK_NAME, user_id)
		}
		.insert(conn)?;
		block
			.make_property("status", status_block.id)
			.insert(conn)?;

		Ok(block)
	}
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct CreationArgs {
	pub name: Option<String>,
	pub desc: Option<Vec<ComponentStruct>>,
}
