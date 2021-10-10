use crate::blocks::document_block::DocumentBlock;
use block_tools::{
	auth::{
		permissions::{has_perm_level, PermLevel},
		require_token, validate_token,
	},
	blocks::BlockType,
	blocks::Context,
	display_api::{ActionObject, MethodObject},
	models::{Block, NewBlock},
	BlockError, LoopError,
};

impl DocumentBlock {
	pub fn add_method(context: &Context, block_id: i64, _args: String) -> Result<Block, LoopError> {
		let conn = &context.pool.get()?;
		let user_id = validate_token(&require_token(context)?)?;

		let access_err: LoopError =
			BlockError::TypeGenericError(format!("Cannot add blocks to {}", block_id)).into();

		let block = match Block::by_id(block_id, conn)? {
			Some(b) => b,
			None => return Err(access_err),
		};
		if !has_perm_level(user_id, &block, PermLevel::Edit) {
			return Err(access_err);
		}
		let new_item = NewBlock {
			block_data: None,
			..NewBlock::new("text", user_id)
		}
		.insert(conn)?;
		block.make_property("item", new_item.id).insert(conn)?;
		Ok(block)
	}
}

impl DocumentBlock {
	pub fn build_add_action_object(block_id: i64) -> ActionObject {
		ActionObject::method(Self::build_add_method_object(block_id))
	}
	pub fn build_add_method_object(block_id: i64) -> MethodObject {
		MethodObject {
			block_type: Self::name(),
			block_id: block_id.to_string(),
			method_name: "add".to_string(),
			arg_template: String::new(),
		}
	}
}
