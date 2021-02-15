use super::block::BlockObject;
use crate::graphql::ContextData;
use async_graphql::{Context, Error, Object};
use block_tools::{
	auth::{
		permissions::{has_perm_level, PermLevel},
		require_token, validate_token,
	},
	models::{Block, NewNotification, User},
	NoAccessSubject, UserError,
};
use block_types::delegation::display::delegate_block_name;

#[derive(Default)]
pub struct BlockPermMutations;

#[Object]
impl BlockPermMutations {
	/// Updates whether or not the block is public or not. The user must have
	/// full permissions or higher.
	pub async fn update_visibility(
		&self,
		context: &Context<'_>,
		public: bool,
		block_id: i64,
	) -> Result<BlockObject, Error> {
		let (context, conn) = &ContextData::parse(context)?;

		let user_id = validate_token(&require_token(context)?)?;

		let access_err: Error =
			UserError::NoAccess(NoAccessSubject::UpdatePermissions(block_id)).into();

		let block = match Block::by_id(block_id, conn)? {
			Some(block) => block,
			None => return Err(access_err),
		};

		if !has_perm_level(user_id, &block, PermLevel::Full) {
			return Err(access_err);
		}

		let block = block.update_public(public, conn)?;

		// If the user is not the owner, notify the owner
		if user_id != block.owner_id {
			// Not the username: display name or username
			let user_name = User::by_id(user_id, conn)?
				.and_then(|user| user.display_name.or(Some(user.username)))
				.unwrap();

			let visibility = match public {
				true => "public",
				false => "private",
			};

			let notif = NewNotification::new(
				format!("{} made your block {}", user_name, visibility),
				format!("{} changed the visibility of a block you own.", user_name),
			)
			.recipients(vec![block.owner_id])
			.link(block_id);
			notif.send(conn)?;
		}
		Ok(block.into())
	}

	/// Set the permissions for the users with full permissions, edit permissions, and view permissions.
	pub async fn set_perms(
		&self,
		context: &Context<'_>,
		#[graphql(default)] perm_full: Vec<i32>,
		#[graphql(default)] perm_edit: Vec<i32>,
		#[graphql(default)] perm_view: Vec<i32>,
		block_id: i64,
	) -> Result<BlockObject, Error> {
		let (context, conn) = &ContextData::parse(context)?;

		let user_id = validate_token(&require_token(context)?)?;

		let access_err: Error =
			UserError::NoAccess(NoAccessSubject::UpdatePermissions(block_id)).into();

		let block = match Block::by_id(block_id, conn)? {
			Some(block) => block,
			None => return Err(access_err),
		};

		if !has_perm_level(user_id, &block, PermLevel::Full) {
			return Err(access_err);
		}

		let block = block.update_perms(perm_full, perm_edit, perm_view, conn)?;

		// If the user is not the owner, send a notification
		if user_id != block.owner_id {
			let user_name = User::by_id(user_id, conn)?
				.and_then(|user| user.display_name.or(Some(user.username)))
				.unwrap();
			let block_name = delegate_block_name(context, &block.block_type, &block)?;

			let notif = NewNotification::new(
				format!("{} updated the permissions of your block", user_name),
				format!("{} updated \"{}\".", user_name, block_name),
			)
			.recipients(vec![block.owner_id])
			.link(block_id);
			notif.send(conn)?;
		}

		Ok(block.into())
	}
}
