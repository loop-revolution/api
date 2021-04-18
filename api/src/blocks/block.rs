use crate::{graphql::ContextData, users::user::UserObject};
use async_graphql::*;
use block_tools::{
	auth::{
		optional_token, optional_validate_token, permissions::can_view, require_token,
		validate_token,
	},
	dsl::prelude::*,
	models::{Block, Comment, User},
	schema::comments,
};
use block_types::delegation::display::{delegate_embed_display, delegate_page_display};
use chrono::{DateTime, Utc};
use std::time::SystemTime;

use super::{
	breadcrumb::{gen_breadcrumb, BreadCrumb},
	comments::CommentObject,
};

/// A block on Loop. Currently the best documentation is in this schema.
pub struct BlockObject {
	// Basic data
	pub block_data: Option<String>,
	pub block_type: String,
	pub id: i64,
	pub color: Option<String>,
	// Dates
	pub created_at: SystemTime,
	pub updated_at: SystemTime,
	// Permissions
	pub owner_id: i32,
	pub perm_edit: Vec<i32>,
	pub perm_full: Vec<i32>,
	pub perm_view: Vec<i32>,
	pub public: bool,
	// Notifications
	pub notif_enabled: Vec<i32>,
	pub stars: Vec<i32>,
}

#[Object]
impl BlockObject {
	/// A unique identifier for the block. This doesn't change
	async fn id(&self) -> i64 {
		self.id
	}

	/// Basic data that belongs to the block. This will only be needed
	/// for very niche purposes. Only basic blocks like `data` use the
	/// `data` field because block type parameters is usually from its properties.
	async fn data(&self) -> Option<String> {
		self.block_data.clone()
	}

	/// The block type name (as a string). This is for display purposes because
	/// display of the block comes in its display fields.
	async fn r#type(&self) -> String {
		self.block_type.clone()
	}

	/// When was the block created?
	async fn created_at(&self) -> DateTime<Utc> {
		self.created_at.into()
	}

	/// When was the block last updated?
	async fn updated_at(&self) -> DateTime<Utc> {
		self.updated_at.into()
	}

	/// Anybody can view public blocks
	async fn public(&self) -> bool {
		self.public
	}

	/// How many users have starred this block?
	async fn star_count(&self) -> usize {
		self.stars.len()
	}

	/// The block's color as an RGB string `rgb(123,123,123)`
	async fn color(&self) -> Option<String> {
		self.color.clone()
	}

	/// This returns true if the authenticated user has starred the block
	async fn starred(&self, context: &Context<'_>) -> Result<bool> {
		let context = &context.data::<ContextData>()?.other();
		let user_id = validate_token(&require_token(context)?)?;
		Ok(self.stars.contains(&user_id))
	}

	/// This returns true if the authenticated user has notifications enbled for the block
	async fn notif_enabled(&self, context: &Context<'_>) -> Result<bool> {
		let context = &context.data::<ContextData>()?.other();
		let user_id = validate_token(&require_token(context)?)?;
		Ok(self.notif_enabled.contains(&user_id))
	}

	/// The user that owns the block
	async fn owner(&self, context: &Context<'_>) -> Result<UserObject> {
		let (_, conn) = &ContextData::parse(context)?;
		let user = User::by_id(self.owner_id, &conn)?;

		Ok(user.unwrap().into())
	}

	/// A list of users that have the specific level provided.
	async fn perm_full(&self, context: &Context<'_>, level: PermLevel) -> Result<Vec<UserObject>> {
		let (_, conn) = &ContextData::parse(context)?;

		let mut users: Vec<UserObject> = vec![];
		let id_list = match level {
			PermLevel::View => self.perm_view.clone(),
			PermLevel::Edit => self.perm_edit.clone(),
			PermLevel::Full => self.perm_full.clone(),
		};

		for id in id_list {
			let user = User::by_id(id, &conn)?;
			if let Some(user) = user {
				users.push(user.into());
			}
		}

		Ok(users)
	}

	/// The JSON string for page display, a DisplayObject
	async fn page_display(&self, context: &Context<'_>) -> Result<String> {
		let context = &context.data::<ContextData>()?.other();
		let display = delegate_page_display(&self.other(), context)?;
		Ok(serde_json::to_string(&display)?)
	}

	/// The JSON string for embedded display as a DisplayComponent
	async fn embed_display(&self, context: &Context<'_>) -> Result<String> {
		let context = &context.data::<ContextData>()?.other();
		let display = delegate_embed_display(&self.other(), context);
		Ok(serde_json::to_string(&display)?)
	}

	/// The breadcrumb for this block. More documentation is on the BreadCrumb typing.
	async fn breadcrumb(&self, context: &Context<'_>) -> Result<Vec<BreadCrumb>> {
		let context = &context.data::<ContextData>()?.other();
		Ok(gen_breadcrumb(context, &self.other())?)
	}

	/// All the comments that the user can see on this block
	async fn comments(&self, context: &Context<'_>) -> Result<Vec<CommentObject>> {
		let (context, conn) = &ContextData::parse(context)?;
		let user_id = optional_validate_token(optional_token(context))?;
		let comments: Vec<Comment> = comments::dsl::comments
			.filter(comments::block_id.eq(self.id))
			.get_results(conn)?;
		let mut comment_objects: Vec<CommentObject> = vec![];

		for comment in comments {
			if let Some(block) = Block::by_id(comment.content_id, &conn)? {
				if can_view(user_id, &block) {
					comment_objects.push(comment.into())
				}
			}
		}

		Ok(comment_objects)
	}

	/// The total number of comments for this block. This includes blocks that the user doesn't
	/// have access to.
	async fn comments_count(&self, context: &Context<'_>) -> Result<i64> {
		let (_, conn) = &ContextData::parse(context)?;
		let count: i64 = comments::dsl::comments
			.filter(comments::block_id.eq(self.id))
			.count()
			.get_result(conn)?;

		Ok(count)
	}
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
/// This is the level of permissions that a user has
pub enum PermLevel {
	/// Users with this level can view the block, and nothing more.
	View,
	/// Users with this level are able to edit the blocks with certain methods as well as
	/// everything that comes with the `VIEW` level
	Edit,
	/// Users with this level are able to edit the block permissions as well as everything
	/// that comes with the `EDIT` level
	Full,
}

impl BlockObject {
	pub fn other(&self) -> Block {
		Block {
			id: self.id,
			created_at: self.created_at,
			updated_at: self.updated_at,
			block_data: self.block_data.clone(),
			color: self.color.clone(),
			block_type: self.block_type.clone(),
			owner_id: self.owner_id,
			public: self.public,
			perm_full: self.perm_full.clone(),
			perm_edit: self.perm_edit.clone(),
			perm_view: self.perm_view.clone(),
			stars: self.stars.clone(),
			notif_enabled: self.notif_enabled.clone(),
		}
	}
}

impl From<Block> for BlockObject {
	fn from(blockd: Block) -> Self {
		BlockObject {
			id: blockd.id,
			created_at: blockd.created_at,
			updated_at: blockd.updated_at,
			block_data: blockd.block_data,
			color: blockd.color,
			block_type: blockd.block_type,
			owner_id: blockd.owner_id,
			public: blockd.public,
			perm_full: blockd.perm_full,
			perm_edit: blockd.perm_edit,
			perm_view: blockd.perm_view,
			stars: blockd.stars,
			notif_enabled: blockd.notif_enabled,
		}
	}
}

impl From<&Block> for BlockObject {
	fn from(blockd: &Block) -> Self {
		BlockObject {
			id: blockd.id,
			created_at: blockd.created_at,
			updated_at: blockd.updated_at,
			block_data: blockd.block_data.clone(),
			block_type: blockd.block_type.clone(),
			color: blockd.color.clone(),
			owner_id: blockd.owner_id,
			public: blockd.public,
			perm_full: blockd.perm_full.clone(),
			perm_edit: blockd.perm_edit.clone(),
			perm_view: blockd.perm_view.clone(),
			stars: blockd.stars.clone(),
			notif_enabled: blockd.notif_enabled.clone(),
		}
	}
}
