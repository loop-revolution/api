use async_graphql::*;
use block_tools::{auth::validate_token, models::Notification, notifications::broker::Broker};
use futures::{Stream, StreamExt};

use super::NotificationObject;

pub struct Notifications;

#[Subscription]
impl Notifications {
	/// Subscribes to new notifications for a user. This takes the user's token as a parameter,
	/// not an authentication heading.
	async fn notifications(&self, token: String) -> Result<impl Stream<Item = NotificationObject>> {
		let user_id = validate_token(&token)?;
		Ok(
			Broker::<Notification>::subscribe().filter_map(move |notif| {
				let show = if notif.recipients.contains(&user_id) {
					Some(notif.into())
				} else {
					None
				};
				async move { show }
			}),
		)
	}
}
