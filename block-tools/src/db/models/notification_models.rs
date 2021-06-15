use super::super::schema::{notifications, users};
use crate::{notifications::broker::Broker, LoopError};
use diesel::prelude::*;
use expo_server_sdk::*;
use std::{str::FromStr, time::SystemTime};

#[derive(Queryable, Clone)]
pub struct Notification {
	pub id: i64,
	pub name: String,
	pub description: String,
	pub block_link: Option<i64>,
	pub recipients: Vec<i32>,
	pub time: Option<SystemTime>,
}

#[derive(Insertable)]
#[table_name = "notifications"]
pub struct NewNotification {
	pub name: String,
	pub description: String,
	pub block_link: Option<i64>,
	pub recipients: Vec<i32>,
	pub time: Option<SystemTime>,
}

impl NewNotification {
	pub fn send(self, conn: &PgConnection) -> Result<Notification, LoopError> {
		let notif: Notification = diesel::insert_into(notifications::table)
			.values(&self)
			.get_result(conn)?;
		Broker::publish(notif.clone());
		let mut tokens: Vec<String> = vec![];
		for user_id in &notif.recipients {
			let mut user_tokens: Vec<String> = users::dsl::users
				.select(users::dsl::expo_tokens)
				.filter(users::dsl::id.eq(user_id))
				.first(conn)?;
			tokens.append(&mut user_tokens);
		}
		for token in tokens {
			let token = PushToken::from_str(token.as_str()).unwrap();
			let msg = PushMessage::new(token)
				.body(&notif.description)
				.title(&notif.name);

			let push_notifier = PushNotifier::new().gzip_policy(GzipPolicy::Always);
			push_notifier.send_push_notification(&msg).unwrap();
		}
		Ok(notif)
	}

	pub fn new(name: impl ToString, description: impl ToString) -> Self {
		NewNotification {
			name: name.to_string(),
			description: description.to_string(),
			recipients: vec![],
			block_link: None,
			time: Some(SystemTime::now()),
		}
	}

	pub fn link(self, block_link: i64) -> Self {
		NewNotification {
			block_link: Some(block_link),
			..self
		}
	}

	pub fn recipients(self, recipients: Vec<i32>) -> Self {
		NewNotification { recipients, ..self }
	}
}
