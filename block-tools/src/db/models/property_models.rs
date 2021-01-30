use diesel::{prelude::*, PgConnection};

use crate::Error;
use super::super::schema::properties;

#[derive(Queryable)]
pub struct Property {
	pub id: i64,
	pub property_name: String,
	pub parent_id: i64,
	pub value_id: i64,
	pub annotation: Option<String>,
}

#[derive(Insertable)]
#[table_name = "properties"]
pub struct NewProperty {
	pub property_name: String,
	pub parent_id: i64,
	pub value_id: i64,
	pub annotation: Option<String>,
}

impl NewProperty {
	pub fn annotate(self, annotation: &str) -> Self {
		NewProperty {
			annotation: Some(annotation.to_string()),
			..self
		}
	}

	pub fn insert(self, conn: &PgConnection) -> Result<Property, Error> {
		Ok(diesel::insert_into(properties::table)
			.values(&self)
			.get_result(conn)?)
	}
}
