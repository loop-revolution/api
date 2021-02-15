use crate::graphql::ContextData;
use async_graphql::*;
use block_tools::use_diesel::*;
use block_tools::{
	auth::{require_token, validate_token},
	models::Notification,
	schema::notifications,
};

use super::NotificationObject;

#[derive(Default)]
pub struct NotificationQueries {}

#[Object]
impl NotificationQueries {
	/// All the notifications that the user has not cleared
	async fn notifications(&self, context: &Context<'_>) -> Result<Vec<NotificationObject>> {
		let (context, conn) = &ContextData::parse(context)?;

		let user_id = validate_token(&require_token(context)?)?;

		let notifs: Vec<Notification> = notifications::dsl::notifications
			.filter(notifications::dsl::recipients.contains(vec![user_id]))
			.load::<Notification>(conn)?;

		Ok(notifs.into_iter().map(|notif| notif.into()).collect())
	}
}
