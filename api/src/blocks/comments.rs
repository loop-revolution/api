use crate::{graphql::ContextData, users::user::UserObject};
use async_graphql::*;
use block_tools::{
	auth::{permissions::can_view, require_token, validate_token},
	dsl,
	dsl::prelude::*,
	models::{Block, Comment, NewComment, User},
	schema::comments,
	NoAccessSubject, UserError,
};
use chrono::{DateTime, Utc};
use std::time::SystemTime;

use super::block::BlockObject;

/// Comments are extra information that can be added to blocks
pub struct CommentObject {
	pub id: i64,
	pub author_id: i32,
	pub content_id: i64,
	pub stars: Vec<i32>,
	pub created_at: SystemTime,
}

#[Object]
impl CommentObject {
	/// A unique identifier for the comment.
	async fn id(&self) -> i64 {
		self.id
	}

	/// When was the comment created?
	async fn created_at(&self) -> DateTime<Utc> {
		self.created_at.into()
	}

	/// How many users have starred this block?
	async fn star_count(&self) -> usize {
		self.stars.len()
	}

	/// This returns true if the authenticated user has starred the comment
	async fn starred(&self, context: &Context<'_>) -> Result<bool> {
		let context = &context.data::<ContextData>()?.other();
		let user_id = validate_token(&require_token(context)?)?;
		Ok(self.stars.contains(&user_id))
	}

	/// Who made the comment?
	async fn author(&self, context: &Context<'_>) -> Result<UserObject> {
		let (_, conn) = &ContextData::parse(context)?;
		let user = User::by_id(self.author_id, &conn)?;

		Ok(user.unwrap().into())
	}

	/// The comment's block
	async fn block(&self, context: &Context<'_>) -> Result<BlockObject> {
		let (_, conn) = &ContextData::parse(context)?;
		let block = Block::by_id(self.content_id, &conn)?;

		Ok(block.unwrap().into())
	}
}

impl From<Comment> for CommentObject {
	fn from(comment: Comment) -> Self {
		CommentObject {
			id: comment.id,
			author_id: comment.author_id,
			content_id: comment.content_id,
			stars: comment.stars,
			created_at: comment.created_at,
		}
	}
}

#[derive(Default)]
pub struct CommentMutations;

#[Object]
impl CommentMutations {
	/// This mutation creates a comment for a block, given a comment content ID and the block to comment on.
	pub async fn create_comment(
		&self,
		context: &Context<'_>,
		#[graphql(desc = "ID of the block to add comment to")] block_id: i64,
		#[graphql(desc = "ID of the content of the comment")] content_id: i64,
	) -> Result<CommentObject, Error> {
		let (context, conn) = &ContextData::parse(context)?;

		let user_id = validate_token(&require_token(context)?)?;

		let access_err = Err(UserError::NoAccess(NoAccessSubject::ViewBlock(block_id)).into());
		let block_on = if let Some(block) = Block::by_id(block_id, conn)? {
			block
		} else {
			return access_err;
		};
		if !can_view(Some(user_id), &block_on) {
			return access_err;
		}
		let access_err = Err(UserError::NoAccess(NoAccessSubject::ViewBlock(content_id)).into());
		let content_block = if let Some(block) = Block::by_id(content_id, conn)? {
			block
		} else {
			return access_err;
		};
		if !can_view(Some(user_id), &content_block) {
			return access_err;
		}

		let comment = NewComment::new(block_on.id, content_block.id, user_id).insert(conn)?;

		Ok(comment.into())
	}

	/// Stars or unstars a comment
	pub async fn set_comment_starred(
		&self,
		context: &Context<'_>,
		comment_id: i64,
		starred: bool,
	) -> Result<CommentObject, Error> {
		let (context, conn) = &ContextData::parse(context)?;

		let user_id = validate_token(&require_token(context)?)?;

		let comment: Option<Comment> = comments::dsl::comments
			.filter(comments::id.eq(comment_id))
			.first(conn)
			.optional()?;

		let comment = match comment {
			Some(block) => block,
			None => {
				return Err(UserError::NoAccess(NoAccessSubject::ViewComment(comment_id)).into())
			}
		};

		let filter = comments::dsl::comments.filter(comments::id.eq(comment_id));
		let mut stars: Vec<i32> = comment.stars;
		if starred {
			stars.push(user_id);
		} else {
			stars = stars.into_iter().filter(|id| id != &user_id).collect();
		}
		let comment: Comment = dsl::update(filter)
			.set((comments::stars.eq(stars),))
			.get_result(conn)?;

		Ok(comment.into())
	}
}
