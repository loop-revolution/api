use std::time::SystemTime;

#[derive(Queryable, Clone)]
pub struct UpdateModel {
	pub id: i32,
	pub created_at: SystemTime,
	pub display: String,
}
