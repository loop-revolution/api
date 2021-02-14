use async_graphql::*;
use block_tools::{auth::validate_token, models::Notification, notifications::broker::Broker};
use futures::{Stream, StreamExt};

use super::NotificationObject;

pub struct Notifications;

#[Subscription]
impl Notifications {
	async fn notifications(&self, token: String) -> Result<impl Stream<Item = NotificationObject>> {
		let id = validate_token(&token)?;
		Ok(
			Broker::<Notification>::subscribe().filter_map(move |notif| {
				let show = if notif.recipients.contains(&id) {
					Some(notif.into())
				} else {
					None
				};
				async move { show }
			}),
		)
	}
}
