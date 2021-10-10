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
	pub fn plus_method(context: &Context, block_id: i64) -> Result<Block, LoopError> {
		let conn = &context.pool.get()?;
		let user_id = validate_token(&require_token(context)?)?;

		let access_err: LoopError =
			BlockError::TypeGenericError(String::from("Cannot edit the habit's score")).into();

		let block = match Block::by_id(block_id, conn)? {
			Some(b) => b,
			None => return Err(access_err),
		};
		let Self { score, streak, .. } = Self::from_id(block_id, Some(user_id), conn)?;
		let streak_num = streak
			.clone()
			.and_then(|block| block.block_data)
			.unwrap_or_default()
			.parse::<i32>()
			.unwrap_or_default();
		if let Some(score) = score {
			if has_perm_level(user_id, &score, PermLevel::Edit) {
				let score_num = score
					.block_data
					.clone()
					.unwrap_or_default()
					.parse::<i32>()
					.unwrap_or_default();
				let mut new_score = score_num + 2 + streak_num;
				if new_score > 100 {
					new_score = 100;
				}
				score.update_data(&new_score.to_string(), conn)?;
				if let Some(streak) = streak {
					if has_perm_level(user_id, &score, PermLevel::Edit) {
						let new_streak = streak_num + 1;
						streak.update_data(&new_streak.to_string(), conn)?;
					}
				}
			}
		} else if has_perm_level(user_id, &block, PermLevel::Edit) {
			let score = NewBlock {
				block_data: Some("2".into()),
				..NewBlock::new("data", user_id)
			}
			.insert(conn)?;
			block.make_property("score", score.id).insert(conn)?;
			let streak = NewBlock {
				block_data: Some("1".into()),
				..NewBlock::new("data", user_id)
			}
			.insert(conn)?;
			block.make_property("streak", streak.id).insert(conn)?;
		}

		Ok(block)
	}
}

impl HabitBlock {
	pub fn build_plus_action_object(block_id: i64) -> ActionObject {
		let method = MethodObject {
			block_type: BLOCK_NAME.into(),
			block_id: block_id.to_string(),
			method_name: "plus".to_string(),
			arg_template: "{}".into(),
		};
		ActionObject::method(method)
	}
}
