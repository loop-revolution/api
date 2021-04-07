use super::super::schema::email_confirm;
use std::time::SystemTime;

#[derive(Queryable)]
pub struct EmailConfirm {
	pub id: i32,
	pub new_email: String,
	pub session_code: String,
	pub verification_code: String,
	pub user_id: i32,
	pub created_at: SystemTime,
}

#[derive(Insertable)]
#[table_name = "email_confirm"]
pub struct NewEmailConfirm {
	pub new_email: String,
	pub session_code: String,
	pub verification_code: String,
	pub user_id: i32,
	pub created_at: SystemTime,
}
