use block_tools::{
	blocks::Context, db::schema::blocks, dsl, dsl::prelude::*, models::Block, LoopError,
};

use crate::blocks::text_block::TextBlock;

impl TextBlock {
	pub fn edit_method(context: &Context, block_id: i64, args: String) -> Result<Block, LoopError> {
		let conn = &context.pool.get()?;

		let display = Self::data_to_display(&args);
		let data = Self::display_to_data(display);

		let block: Block = dsl::update(blocks::dsl::blocks.filter(blocks::id.eq(block_id)))
			.set((
				blocks::block_data.eq(Some(data)),
				blocks::updated_at.eq(std::time::SystemTime::now()),
			))
			.get_result(conn)?;

		Ok(block)
	}
}
