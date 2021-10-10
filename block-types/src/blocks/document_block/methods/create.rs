use block_tools::{
	blocks::Context,
	models::{Block, NewBlock},
	BlockError, LoopError,
};
use serde::{Deserialize, Serialize};

use crate::blocks::{data_block, document_block::DocumentBlock};

impl DocumentBlock {
	pub fn handle_create_raw(
		input: String,
		context: &Context,
		user_id: i32,
	) -> Result<Block, LoopError> {
		let input = serde_json::from_str::<CreationArgs>(&input);
		let input: CreationArgs = input.map_err(|_| BlockError::InputParse)?;

		Self::handle_create(input, context, user_id)
	}
}

impl DocumentBlock {
	pub fn handle_create(
		input: CreationArgs,
		context: &Context,
		user_id: i32,
	) -> Result<Block, LoopError> {
		let conn = &context.conn()?;

		let block = NewBlock::new("document", user_id).insert(conn)?;

		if let Some(name) = input.name {
			let name_block = NewBlock {
				block_data: Some(name),
				..NewBlock::new(data_block::BLOCK_NAME, user_id)
			}
			.insert(conn)?;

			block.make_property("name", name_block.id).insert(conn)?;
		}

		Ok(block)
	}
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreationArgs {
	pub name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
	pub id: String,
}

impl Default for CreationArgs {
	fn default() -> Self {
		Self {
			name: Some("".into()),
		}
	}
}

impl From<i64> for Item {
	fn from(id: i64) -> Self {
		Self { id: id.to_string() }
	}
}
