use crate::{blocks::block::BlockObject, graphql::ContextData};
use async_graphql::*;
use block_tools::{
	auth::{
		optional_token, optional_validate_token,
		permissions::{can_view, maybe_use_view},
		require_token, validate_token,
	},
	dsl::prelude::*,
	models::{Block, User},
	schema::{blocks, users},
};

/// GraphQL object for Loop users
pub struct UserObject {
	pub id: i32,
	pub username: String,
	pub display_name: Option<String>,
	pub root_id: Option<i64>,
	pub featured_id: Option<i64>,
	pub email: String,
	pub credits: i32,
}

#[Object]
impl UserObject {
	/// The number of credits the user has. Will only show if the user matches the authentication
	/// token. If not (or no token) it will be null.
	async fn credits(&self, context: &Context<'_>) -> Result<Option<i32>, Error> {
		let (context, _) = &ContextData::parse(context)?;
		let token = require_token(context)?;

		if self.id != validate_token(&token)? {
			return Ok(None);
		}

		Ok(Some(self.credits))
	}

	/// The user's email. Will only show if the user matches the authentication
	/// token. If not (or no token) it will be null.
	async fn email(&self, context: &Context<'_>) -> Result<Option<String>, Error> {
		let (context, _) = &ContextData::parse(context)?;
		let token = require_token(context)?;

		if self.id != validate_token(&token)? {
			return Ok(None);
		}

		Ok(Some(self.email.clone()))
	}

	/// The user's unique ID. Use this whenever unique identification is needed
	/// (not username) because it cannot be changed.
	async fn id(&self) -> i32 {
		self.id
	}

	/// The user's unique username. This can be changed, and no alphanumerically similar
	/// usernames may exist.
	async fn username(&self) -> String {
		self.username.clone()
	}

	/// An optional display name users may have. This is for more free-form identification
	/// of users for humans, not robots. Can contain more than alphanumeric characters.
	async fn display_name(&self) -> Option<String> {
		self.display_name.clone()
	}

	/// All blocks that a user owns
	async fn blocks(&self, context: &Context<'_>) -> Result<Vec<BlockObject>, Error> {
		let (context, conn) = &ContextData::parse(context)?;
		let user_id = optional_validate_token(optional_token(context))?;

		let blocks: Vec<BlockObject> = blocks::dsl::blocks
			.filter(blocks::dsl::owner_id.eq(self.id))
			.load::<Block>(conn)?
			.iter()
			// Filter out the blocks that the user has no access to
			.filter(|block| can_view(user_id, block))
			.map(BlockObject::from)
			.collect();

		Ok(blocks)
	}

	/// The user's root block. This block is what is shown to the user on their home page,
	/// and considered their main block for their content.
	async fn root(&self, context: &Context<'_>) -> Result<Option<BlockObject>> {
		let root_id = match self.root_id {
			Some(id) => id,
			None => return Ok(None),
		};
		let (context, conn) = &ContextData::parse(context)?;
		let root = Block::by_id(root_id, conn)?;
		// Return the block if it can be accessed
		Ok(maybe_use_view(context, root)?.map(|block| block.into()))
	}

	/// The user's featured block, which can be a bio. This is what is featured on the user's
	/// profile page.
	async fn featured(&self, context: &Context<'_>) -> Result<Option<BlockObject>> {
		let featured_id = match self.featured_id {
			Some(id) => id,
			None => return Ok(None),
		};
		let (context, conn) = &ContextData::parse(context)?;
		let root = Block::by_id(featured_id, conn)?;
		// Return the block if it can be accessed
		Ok(maybe_use_view(context, root)?.map(|block| block.into()))
	}
}

impl From<User> for UserObject {
	fn from(userd: User) -> Self {
		UserObject {
			id: userd.id,
			username: userd.username,
			display_name: userd.display_name,
			root_id: userd.root_id,
			featured_id: userd.featured_id,
			email: userd.email,
			credits: userd.credits,
		}
	}
}

#[derive(Default)]
pub struct UserQueries;

#[Object]
impl UserQueries {
	/// How many users there are in the database
	async fn user_count(&self, context: &Context<'_>) -> Result<i32> {
		let (_, conn) = &ContextData::parse(context)?;

		let num: i64 = users::dsl::users.count().get_result(conn)?;
		Ok(num as i32)
	}
}
