use crate::blocks::document_block::DocumentBlock;
use block_tools::{
	auth::{optional_token, optional_validate_token},
	blocks::Context,
	display_api::{DisplayMeta, DisplayObject},
	models::Block,
	LoopError,
};

impl DocumentBlock {
	pub fn handle_page_display(
		block: &Block,
		context: &Context,
	) -> Result<DisplayObject, LoopError> {
		let conn = &context.conn()?;
		let user_id = optional_validate_token(optional_token(context))?;

		// Build the properties
		let Self { name, items } = Self::from_id(block.id, user_id, conn)?;

		let meta = DisplayMeta {
			page: Some(Self::page_meta(block, user_id, name)),
			..Default::default()
		};
		Ok(DisplayObject {
			meta: Some(meta),
			..DisplayObject::new(Self::display_content(user_id, block, items, context)?)
		})
	}
}
