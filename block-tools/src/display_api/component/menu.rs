use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct MenuComponent {
	pub block_id: i64,
	pub star_button: Option<StarButton>,
	pub notifications_enabled: Option<bool>,
	pub delete: Option<bool>,
	pub permissions: Option<PermissionsList>,
}

#[derive(Serialize, Debug)]
pub struct StarButton {
	pub starred: bool,
	pub count: i32,
}

#[derive(Serialize, Debug)]
pub struct PermissionsList {
	pub full: i32,
	pub edit: i32,
	pub view: i32,
}
