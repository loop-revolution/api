use crate::db::schema::{potential_users, users};
use juniper::GraphQLObject;
use std::time::SystemTime;

#[derive(Queryable, GraphQLObject)]
pub struct User {
	/// Auto-incrementing unique ID for a user
	pub id: i32,
	/// Unique alphanumeric username for easy identification
	pub username: String,
	#[graphql(skip)]
	/// Username (standardized)
	pub localuname: String,
	#[graphql(skip)]
	pub password: String,
	#[graphql(skip)]
	pub email: String,
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

#[derive(GraphQLObject)]
pub struct EmailConfirm {
	pub email: String,
	pub session_code: String,
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
