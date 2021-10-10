use block_tools::{
	auth::{
		permissions::{has_perm_level, PermLevel},
		require_token, validate_token,
	},
	blocks::Context,
	display_api::{
		component::misc::search::{SearchComponent, SearchType},
		ActionObject, MethodObject,
	},
	models::{Block, NewBlock, NewNotification},
	BlockError, LoopError,
};
use serde::{Deserialize, Serialize};

use crate::blocks::task_block::{TaskBlock, BLOCK_NAME};

impl TaskBlock {
	pub fn assign_method(
		context: &Context,
		block_id: i64,
		args: String,
	) -> Result<Block, LoopError> {
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
		let invalid_err: LoopError = BlockError::InputParse.into();
		let input = match serde_json::from_str::<AssignArgs>(&args) {
			Ok(input) => input,
			Err(_) => return Err(invalid_err),
		};

		// Add the assignee data block
		let new_block = NewBlock {
			block_data: Some(input.id.clone()),
			..NewBlock::new("data", user_id)
		}
		.insert(conn)?;
		block.make_property("assignee", new_block.id).insert(conn)?;

		// Get the assignee's ID
		let assignee_id: i32 = match input.id.parse() {
			Ok(id) => id,
			Err(_) => return Err(invalid_err),
		};

		// Give the assignee edit access
		let mut new_edit_perms = block.perm_edit.clone();
		new_edit_perms.push(assignee_id);
		block.update_perms(
			block.perm_full.clone(),
			new_edit_perms.clone(),
			block.perm_view.clone(),
			conn,
		)?;
		Self::handle_general_perm_update(
			context,
			block_id,
			block.perm_full.clone(),
			new_edit_perms,
			block.perm_view.clone(),
		)?;

		// Send a notification
		let block_name = Self::handle_block_name(&block, context)?;
		let notif = NewNotification::new(
			"You've been assigned to a task",
			format!("You've been assigned \"{}\"", block_name),
		)
		.recipients(vec![assignee_id])
		.link(block_id);
		notif.send(conn)?;

		Ok(block)
	}
}

#[derive(Serialize, Deserialize, Debug)]
struct AssignArgs {
	id: String,
}

impl TaskBlock {
	pub fn build_assign_action_object(block_id: i64) -> ActionObject {
		let method = MethodObject {
			block_type: BLOCK_NAME.into(),
			block_id: block_id.to_string(),
			method_name: "assign".to_string(),
			arg_template: r#"{"id":$[ASSIGNEE]$}"#.into(),
		};
		let search = SearchComponent {
			action_text: Some("Choose a user to assign".to_string()),
			search_type: Some(SearchType::User),
			name: Some("ASSIGNEE".into()),
			then: Some(ActionObject::method(method)),
			..Default::default()
		};
		ActionObject::search(search)
	}
}
