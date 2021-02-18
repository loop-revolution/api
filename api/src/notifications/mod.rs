pub mod queries;
pub mod sub;

use crate::{blocks::block::BlockObject, graphql::ContextData, users::user::UserObject};
use async_graphql::*;
use block_tools::{
	auth::{
		permissions::{has_perm_level, PermLevel},
		require_token, validate_token,
	},
	models::{Block, NewNotification, Notification, User},
	NoAccessSubject, UserError,
};
use block_types::delegation::display::delegate_block_name;
use chrono::{DateTime, Utc};

#[derive(SimpleObject, Clone)]
/// A Loop notification
pub struct NotificationObject {
	/// Name of the notification, has a recognizable, short message
	pub name: String,
	/// More information about the notification
	pub description: String,
	/// The block that clicking the notification will redirect to
	pub block_link: Option<i64>,
	/// When the notification was sent
	pub time: Option<DateTime<Utc>>,
	#[graphql(skip)]
	pub recipients: Vec<i32>,
}
#[derive(Default)]
pub struct NotificationMutations {}

#[Object]
impl NotificationMutations {
	/// A developer mutation, sends a notification to the sender.
	async fn self_notif(
		&self,
		context: &Context<'_>,
		name: String,
		description: String,
	) -> Result<NotificationObject> {
		let (context, conn) = &ContextData::parse(context)?;

		let user_id = validate_token(&require_token(context)?)?;

		let notif = NewNotification::new(name, description)
			.recipients(vec![user_id])
			.send(conn)?
			.into();
		Ok(notif)
	}

	/// Stars or unstars a block
	pub async fn set_starred(
		&self,
		context: &Context<'_>,
		block_id: i64,
		starred: bool,
	) -> Result<BlockObject, Error> {
		let (context, conn) = &ContextData::parse(context)?;

		let user_id = validate_token(&require_token(context)?)?;

		let access_err: Error = UserError::NoAccess(NoAccessSubject::NotifBlock(block_id)).into();
		let block = match Block::by_id(block_id, conn)? {
			Some(block) => block,
			None => return Err(access_err),
		};
		if !has_perm_level(user_id, &block, PermLevel::View) {
			return Err(access_err);
		}

		let block = block.update_starred(starred, user_id, conn)?;

		// Send a notification
		if starred && block.owner_id != user_id {
			let user_name = User::by_id(user_id, conn)?
				.and_then(|user| user.display_name.or(Some(user.username)))
				.unwrap_or_else(|| "A user".into());
			let block_name = delegate_block_name(context, &block.block_type, &block)?;

			let notif = NewNotification::new(
				format!("{} starred \"{}\"", user_name, block_name),
				format!("{} starred a block that you own.", user_name),
			)
			.recipients(vec![block.owner_id])
			.link(block_id);
			notif.send(conn)?;
		}

		Ok(block.into())
	}

	/// Enable and disable notifications for a block
	pub async fn set_notifs(
		&self,
		context: &Context<'_>,
		block_id: i64,
		enabled: bool,
	) -> Result<BlockObject, Error> {
		let (context, conn) = &ContextData::parse(context)?;

		let user_id = validate_token(&require_token(context)?)?;

		let access_err: Error = UserError::NoAccess(NoAccessSubject::NotifBlock(block_id)).into();
		let block = match Block::by_id(block_id, conn)? {
			Some(block) => block,
			None => return Err(access_err),
		};
		if !has_perm_level(user_id, &block, PermLevel::View) {
			return Err(access_err);
		}

		Ok(block.update_notifs(enabled, user_id, conn)?.into())
	}

	/// Set a user's expo tokens to recieve push notifications
	pub async fn update_expo_tokens(
		&self,
		context: &Context<'_>,
		tokens: Vec<String>,
	) -> Result<UserObject, Error> {
		let (context, conn) = &ContextData::parse(context)?;

		let user_id = validate_token(&require_token(context)?)?;

		let user = match User::by_id(user_id, conn)? {
			Some(user) => user,
			None => return Err(UserError::JwtGeneric.into()),
		};

		Ok(user.update_expo_tokens(tokens, conn)?.into())
	}
}

impl From<Notification> for NotificationObject {
	fn from(n: Notification) -> Self {
		NotificationObject {
			name: n.name,
			description: n.description,
			recipients: n.recipients,
			block_link: n.block_link,
			time: n.time.map(|time| time.into()),
		}
	}
}
