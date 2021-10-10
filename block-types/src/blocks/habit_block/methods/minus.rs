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

use crate::blocks::habit_block::{HabitBlock, BLOCK_NAME};

impl HabitBlock {
	pub fn minus_method(context: &Context, block_id: i64) -> Result<Block, LoopError> {
		let conn = &context.pool.get()?;
		let user_id = validate_token(&require_token(context)?)?;

		let access_err: LoopError =
			BlockError::TypeGenericError(String::from("Cannot edit the habit's score")).into();

		let block = match Block::by_id(block_id, conn)? {
			Some(b) => b,
			None => return Err(access_err),
		};
		let Self { score, streak, .. } = Self::from_id(block_id, Some(user_id), conn)?;
		if let Some(score) = score {
			if has_perm_level(user_id, &score, PermLevel::Edit) {
				let score_num = score
					.block_data
					.clone()
					.unwrap_or_default()
					.parse::<i32>()
					.unwrap_or_default();
				let mut new_score = score_num - 5;
				if new_score.is_negative() {
					new_score = 0;
				}
				score.update_data(&new_score.to_string(), conn)?;
				if let Some(streak) = streak {
					if has_perm_level(user_id, &score, PermLevel::Edit) {
						streak.update_data("0", conn)?;
					}
				}
			}
		} else if has_perm_level(user_id, &block, PermLevel::Edit) {
			let score = NewBlock {
				block_data: Some("0".into()),
				..NewBlock::new("data", user_id)
			}
			.insert(conn)?;
			block.make_property("score", score.id).insert(conn)?;
			let streak = NewBlock {
				block_data: Some("0".into()),
				..NewBlock::new("data", user_id)
			}
			.insert(conn)?;
			block.make_property("streak", streak.id).insert(conn)?;
		}

		Ok(block)
	}
}

impl HabitBlock {
	pub fn build_minus_action_object(block_id: i64) -> ActionObject {
		let method = MethodObject {
			block_type: BLOCK_NAME.into(),
			block_id: block_id.to_string(),
			method_name: "minus".to_string(),
			arg_template: "{}".into(),
		};
		ActionObject::method(method)
	}
}
