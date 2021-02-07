use super::super::schema::notifications;
use crate::{notifications::broker::Broker, Error};
use diesel::prelude::*;

#[derive(Queryable, Clone)]
pub struct Notification {
	pub id: i64,
	pub name: String,
	pub description: String,
	pub block_link: Option<i64>,
	pub recipients: Vec<i32>,
}

#[derive(Insertable)]
#[table_name = "notifications"]
pub struct NewNotification {
	pub name: String,
	pub description: String,
	pub block_link: Option<i64>,
	pub recipients: Vec<i32>,
}

impl NewNotification {
	pub fn send(self, conn: &PgConnection) -> Result<Notification, Error> {
		let notif: Notification = diesel::insert_into(notifications::table)
			.values(&self)
			.get_result(conn)?;
		Broker::publish(notif.clone());
		Ok(notif)
	}

	pub fn new(name: String, description: String) -> Self {
		NewNotification {
			name,
			description,
			recipients: vec![],
			block_link: None,
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
