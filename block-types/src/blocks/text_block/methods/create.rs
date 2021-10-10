use crate::blocks::text_block::TextBlock;
use block_tools::{blocks::BlockType, display_api::component::DisplayComponent};
use block_tools::{
	blocks::Context,
	models::{Block, NewBlock},
	LoopError,
};

impl TextBlock {
	pub fn handle_create(
		input: String,
		context: &Context,
		user_id: i32,
	) -> Result<Block, LoopError> {
		let display = Self::data_to_display(&input);

		Self::handle_create_vec(display, context, user_id)
	}
	pub fn handle_create_vec(
		display: Vec<DisplayComponent>,
		context: &Context,
		user_id: i32,
	) -> Result<Block, LoopError> {
		let conn = &context.pool.get()?;

		let data = Self::display_to_data(display);

		let block = NewBlock {
			block_data: Some(data),
			..NewBlock::new(Self::name(), user_id)
		};

		block.insert(conn)
	}
}
