use crate::blocks::task_block::{TaskBlock, BLOCK_NAME};
use block_tools::{
	auth::{
		permissions::{has_perm_level, PermLevel},
		require_token, validate_token,
	},
	blocks::Context,
	display_api::{ActionObject, MethodObject},
	models::{Block, NewBlock},
	BlockError, LoopError,
};
use serde::{Deserialize, Serialize};

impl TaskBlock {
	pub fn set_status_method(
		context: &Context,
		block_id: i64,
		args: String,
	) -> Result<Block, LoopError> {
		let conn = &context.pool.get()?;
		let user_id = validate_token(&require_token(context)?)?;

		let access_err: LoopError =
			BlockError::TypeGenericError("Cannot edit status".into()).into();

		let block = match Block::by_id(block_id, conn)? {
			Some(b) => b,
			None => return Err(access_err),
		};
		if !has_perm_level(user_id, &block, PermLevel::Edit) {
			return Err(access_err);
		}
		let invalid_err: LoopError = BlockError::InputParse.into();
		let input = match serde_json::from_str::<MethodArgs>(&args) {
			Ok(input) => input,
			Err(_) => return Err(invalid_err),
		};
		let Self { status, .. } = Self::from_id(block_id, Some(user_id), conn)?;
		let mut new_data = input.status;
		if new_data.is_negative() || new_data > 2 {
			new_data = 0
		}
		let new_data = new_data.to_string();
		if let Some(status) = status {
			status.update_data(&new_data, conn)?;
		} else {
			let score = NewBlock {
				block_data: Some(new_data),
				..NewBlock::new("data", user_id)
			}
			.insert(conn)?;
			block.make_property("status", score.id).insert(conn)?;
		}
		Ok(block)
	}
}

#[derive(Serialize, Deserialize, Debug)]
struct MethodArgs {
	status: i32,
}

impl TaskBlock {
	pub fn build_set_status_action_object(block_id: i64) -> ActionObject {
		let method = MethodObject {
			block_type: BLOCK_NAME.into(),
			block_id: block_id.to_string(),
			method_name: "set_status".to_string(),
			arg_template: r#"{"status":$[STATUS]$}"#.into(),
		};
		ActionObject::method(method)
	}
}
