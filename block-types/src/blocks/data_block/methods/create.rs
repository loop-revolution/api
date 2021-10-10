use crate::blocks::data_block::DataBlock;
use block_tools::{blocks::BlockType, models::NewBlock};
use block_tools::{blocks::Context, models::Block, LoopError};

impl DataBlock {
	pub fn handle_create(
		input: String,
		context: &Context,
		user_id: i32,
	) -> Result<Block, LoopError> {
		let conn = &context.pool.get()?;
		let mut input = input;
		input.remove(0);
		input.pop();

		let block = NewBlock {
			block_data: Some(input),
			..NewBlock::new(Self::name(), user_id)
		};

		block.insert(conn)
	}
}
