use crate::db::schema::users;
use juniper::GraphQLObject;

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
}
