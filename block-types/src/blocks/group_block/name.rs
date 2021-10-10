use block_tools::{
	auth::{optional_token, optional_validate_token},
	blocks::Context,
	models::Block,
	LoopError,
};

use super::GroupBlock;

impl GroupBlock {
	/// Gets the name of a specific block (name prop) or use a default
	pub fn handle_block_name(block: &Block, context: &Context) -> Result<String, LoopError> {
		let conn = &context.conn()?;
		let user_id = optional_validate_token(optional_token(context))?;
		let name = Self::from_id(block.id, user_id, conn)?
			.name
			.and_then(|block| block.block_data)
			.unwrap_or_else(|| "Group Block".into());

		Ok(name)
	}
}
