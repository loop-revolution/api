use async_graphql::*;
use block_tools::{
	auth::{optional_token, optional_validate_token, require_token, validate_token},
	db::schema::updates,
	dsl,
	dsl::*,
	models::update_models::UpdateModel,
	LoopError,
};
use chrono::{DateTime, Utc};

use crate::graphql::ContextData;

#[derive(Clone)]
/// A Loop notification
pub struct Update {
	/// Unique ID of the update (not for the user)
	pub id: i32,
	/// The display API of the content to sxhow
	pub display: String,
	/// When was the update posted
	pub created_at: DateTime<Utc>,
	pub seen: Vec<i32>,
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
		let (user_ctx, _) = &ContextData::parse(context)?;
		let user_id = optional_validate_token(optional_token(user_ctx))?;
		Ok(user_id.map(|id| self.seen.contains(&id)))
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
	async fn set_update_seen(
		&self,
		context: &Context<'_>,
		seen: bool,
		update_id: i32,
	) -> Result<Update> {
		let (context, conn) = &ContextData::parse(context)?;
		let user_id = validate_token(&require_token(context)?)?;

		let update: Option<UpdateModel> = updates::dsl::updates
			.filter(updates::dsl::id.eq(update_id))
			.first(conn)
			.optional()?;
		let update = if let Some(update) = update {
			update
		} else {
			return Err(LoopError::GenericError.into());
		};

		let mut new_seen = update.seen;
		if seen && !new_seen.contains(&user_id) {
			new_seen.push(user_id);
		} else {
			new_seen = new_seen.into_iter().filter(|id| id != &user_id).collect();
		}
		let update: UpdateModel =
			dsl::update(updates::dsl::updates.filter(updates::id.eq(update_id)))
				.set((updates::seen.eq(new_seen),))
				.get_result(conn)?;
		Ok(update.into())
	}
}

impl From<UpdateModel> for Update {
	fn from(model: UpdateModel) -> Self {
		Self {
			id: model.id,
			display: model.display,
			created_at: model.created_at.into(),
			seen: model.seen,
		}
	}
}
