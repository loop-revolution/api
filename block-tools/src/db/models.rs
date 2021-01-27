use super::schema::{blocks, potential_users, properties, users};
use std::time::SystemTime;

#[derive(Queryable)]
pub struct UserD {
	/// Auto-incrementing unique ID for a user
	pub id: i32,
	/// Unique alphanumeric username for easy identification
	pub username: String,
	/// Username (standardized)
	pub localuname: String,
	pub password: String,
	pub email: String,
	pub credits: i32,
}

#[derive(Queryable)]
pub struct BlockD {
	pub id: i64,
	pub block_type: String,
	pub created_at: SystemTime,
	pub updated_at: SystemTime,
	pub block_data: Option<String>,
	pub owner_id: i32,
}

#[derive(Queryable)]
pub struct PropertyD {
	pub id: i64,
	pub property_name: String,
	pub parent_id: i64,
	pub value_id: i64,
	pub annotation: Option<String>,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
	/// Username must be unique
	pub username: &'a str,
	/// Localized username, must be unique
	pub localuname: &'a str,
	/// Will be securely stored
	pub password: &'a str,
	pub email: &'a str,
	pub credits: i32,
}

#[derive(Insertable)]
#[table_name = "blocks"]
pub struct NewBlock<'a> {
	pub block_type: &'a str,
	pub created_at: SystemTime,
	pub updated_at: SystemTime,
	pub block_data: Option<&'a str>,
	pub owner_id: i32,
}

#[derive(Insertable)]
#[table_name = "properties"]
pub struct NewProperty<'a> {
	pub property_name: &'a str,
	pub parent_id: i64,
	pub value_id: i64,
	pub annotation: Option<&'a str>,
}

#[derive(Queryable)]
pub struct PotentialUser {
	pub id: i32,
	pub email: String,
	pub session_code: String,
	pub verification_code: String,
	pub username: String,
	pub password: String,
	pub created_at: SystemTime,
}

#[derive(Insertable)]
#[table_name = "potential_users"]
pub struct NewPotentialUser<'a> {
	pub email: &'a str,
	pub session_code: &'a str,
	pub verification_code: &'a str,
	pub username: &'a str,
	pub password: &'a str,
	pub created_at: SystemTime,
}
