use async_graphql::*;
use block_tools::auth::{require_token, validate_token};
use broker::Broker;
use futures::{Stream, StreamExt};

use crate::graphql::ContextData;
mod broker;

pub struct Notifications;

#[derive(SimpleObject, Clone)]
pub struct Notification {
	pub name: String,
	pub description: String,
	pub block_link: i64,
	pub recipients: Vec<i32>,
}

#[Subscription]
impl Notifications {
	async fn notifications(&self, token: String) -> Result<impl Stream<Item = Notification>> {
		let id = validate_token(token)?;
		Ok(Broker::<Notification>::subscribe().filter(move |notif| {
			let show = if notif.recipients.contains(&id) {
				true
			} else {
				false
			};
			async move { show }
		}))
	}
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
	) -> Result<Notification> {
		let context = &context.data::<ContextData>()?.other();
		let user_id = validate_token(require_token(context)?)?;
		let notif = Notification {
			name,
			description,
			block_link: 0,
			recipients: vec![user_id],
		};
		Broker::publish(notif.clone());
		Ok(notif)
	}
}
