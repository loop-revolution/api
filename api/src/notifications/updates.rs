use async_graphql::*;
use block_tools::{
	auth::{optional_token, optional_validate_token, require_token, validate_token},
	db::schema::updates,
	dsl,
	dsl::*,
	models::{update_models::UpdateModel, User},
	schema::users,
};
use chrono::{DateTime, Utc};

use crate::{graphql::ContextData, users::user::UserObject};

#[derive(Clone)]
/// A Loop notification
pub struct Update {
	/// Unique ID of the update (not for the user)
	pub id: i32,
	/// The display API of the content to sxhow
	pub display: String,
	/// When was the update posted
	pub created_at: DateTime<Utc>,
}

#[Object]
impl Update {
	pub async fn id(&self) -> i32 {
		self.id
	}
	pub async fn display(&self) -> String {
		self.display.clone()
	}
	pub async fn created_at(&self) -> DateTime<Utc> {
		self.created_at
	}
	pub async fn seen(&self, context: &Context<'_>) -> Result<Option<bool>> {
		let (user_ctx, conn) = &ContextData::parse(context)?;
		let user_id = optional_validate_token(optional_token(user_ctx))?;
		let user = if let Some(user_id) = user_id {
			if let Some(user) = User::by_id(user_id, conn)? {
				user
			} else {
				return Ok(None);
			}
		} else {
			return Ok(None);
		};
		if let Some(id) = user.latest_update_seen_id {
			Ok(Some(id >= self.id))
		} else {
			Ok(Some(false))
		}
	}
}

#[derive(Default)]
pub struct UpdateQueries {}

#[Object]
impl UpdateQueries {
	/// All the updates that have been published
	async fn all_updates(&self, context: &Context<'_>) -> Result<Vec<Update>> {
		let (_, conn) = &ContextData::parse(context)?;
		let updates: Vec<UpdateModel> = updates::dsl::updates.load::<UpdateModel>(conn)?;
		Ok(updates.into_iter().map(Update::from).collect())
	}
}

#[derive(Default)]
pub struct UpdateMutations {}

#[Object]
impl UpdateMutations {
	/// Mark an update as seen/unseen
	async fn set_latest_seen(
		&self,
		context: &Context<'_>,
		latest_update_id: i32,
	) -> Result<UserObject> {
		let (context, conn) = &ContextData::parse(context)?;
		let user_id = validate_token(&require_token(context)?)?;

		let user: User = dsl::update(users::dsl::users.filter(users::id.eq(user_id)))
			.set((users::latest_update_seen_id.eq(latest_update_id),))
			.get_result(conn)?;
		Ok(user.into())
	}
}

impl From<UpdateModel> for Update {
	fn from(model: UpdateModel) -> Self {
		Self {
			id: model.id,
			display: model.display,
			created_at: model.created_at.into(),
		}
	}
}
