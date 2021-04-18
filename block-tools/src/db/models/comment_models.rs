use super::super::schema::comments;
use crate::LoopError;
use diesel::prelude::*;
use std::time::SystemTime;

#[derive(Queryable, Clone)]
pub struct Comment {
	pub id: i64,
	pub author_id: i32,
	pub content_id: i64,
	pub block_id: i64,
	pub stars: Vec<i32>,
	pub created_at: SystemTime,
}

#[derive(Insertable)]
#[table_name = "comments"]
pub struct NewComment {
	pub author_id: i32,
	pub content_id: i64,
	pub block_id: i64,
	pub stars: Vec<i32>,
	pub created_at: SystemTime,
}

impl NewComment {
	pub fn new(block_id: i64, content_id: i64, author_id: i32) -> Self {
		Self {
			created_at: std::time::SystemTime::now(),
			author_id,
			content_id,
			block_id,
			stars: vec![],
		}
	}

	pub fn insert(self, conn: &PgConnection) -> Result<Comment, LoopError> {
		Ok(diesel::insert_into(comments::table)
			.values(self)
			.get_result(conn)?)
	}
}
