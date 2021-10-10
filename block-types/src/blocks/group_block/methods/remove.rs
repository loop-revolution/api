use block_tools::{
	auth::{
		permissions::{has_perm_level, PermLevel},
		require_token, validate_token,
	},
	blocks::Context,
	display_api::MethodObject,
	dsl,
	dsl::prelude::*,
	models::Block,
	schema::properties,
	BlockError, LoopError,
};
use serde::{Deserialize, Serialize};

use crate::blocks::group_block::{GroupBlock, BLOCK_NAME};

impl GroupBlock {
	pub fn remove_method(
		context: &Context,
		block_id: i64,
		args: String,
	) -> Result<Block, LoopError> {
		let conn = &context.pool.get()?;
		let user_id = validate_token(&require_token(context)?)?;

		let access_err: LoopError =
			BlockError::TypeGenericError(format!("Cannot remove items from {}", block_id)).into();

		let block = match Block::by_id(block_id, conn)? {
			Some(b) => b,
			None => return Err(access_err),
		};
		if !has_perm_level(user_id, &block, PermLevel::Edit) {
			return Err(access_err);
		}
		let invalid_err: LoopError = BlockError::InputParse.into();
		let input = match serde_json::from_str::<RemoveArgs>(&args) {
			Ok(input) => input,
			Err(_) => return Err(invalid_err),
		};
		dsl::delete(properties::dsl::properties.filter(properties::dsl::id.eq(input.prop_id)))
			.execute(conn)?;

		Ok(block)
	}
}

#[derive(Serialize, Deserialize, Debug)]
struct RemoveArgs {
	prop_id: i64,
}

impl GroupBlock {
	pub fn build_remove_method_object(block_id: i64, prop_id: i64) -> MethodObject {
		MethodObject {
			block_type: BLOCK_NAME.into(),
			block_id: block_id.to_string(),
			method_name: "remove".to_string(),
			arg_template: format!(r#"{{"prop_id":{}}}"#, prop_id),
		}
	}
}
