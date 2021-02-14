use crate::graphql::ContextData;
use async_graphql::*;
use block_tools::{
	auth::{require_token, validate_token},
	models::{NewNotification, Notification},
};
pub mod queries;
pub mod sub;

#[derive(SimpleObject, Clone)]
pub struct QLNotification {
	pub name: String,
	pub description: String,
	pub block_link: Option<i64>,
	#[graphql(skip)]
	pub recipients: Vec<i32>,
}
#[derive(Default)]
pub struct NotificationMutations {}

#[Object]
impl NotificationMutations {
	async fn self_notif(
		&self,
		context: &Context<'_>,
		name: String,
		description: String,
	) -> Result<QLNotification> {
		let context = &context.data::<ContextData>()?.other();
		let conn = &context.pool.get()?;
		let user_id = validate_token(&require_token(context)?)?;
		let notif = NewNotification::new(name, description)
			.recipients(vec![user_id])
			.send(conn)?
			.into();
		Ok(notif)
	}
}

impl From<Notification> for QLNotification {
	fn from(n: Notification) -> Self {
		QLNotification {
			name: n.name,
			description: n.description,
			recipients: n.recipients,
			block_link: n.block_link,
		}
	}
}
