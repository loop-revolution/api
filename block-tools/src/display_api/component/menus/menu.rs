use serde::{Deserialize, Serialize};

use crate::{
	auth::permissions::{has_perm_level, PermLevel},
	display_api::{component::atomic::icon::Icon, ActionObject},
	models::Block,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MenuComponent {
	pub block_id: i64,
	pub cid: String,
	pub delete: Option<bool>,
	pub notifications_enabled: Option<bool>,
	pub permissions: Option<PermissionsList>,
	pub star_button: Option<StarButton>,
	pub custom: Option<Vec<CustomMenuItem>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StarButton {
	pub starred: bool,
	pub count: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PermissionsList {
	pub full: usize,
	pub edit: usize,
	pub view: usize,
	pub public: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CustomMenuItem {
	pub icon: Icon,
	pub text: String,
	pub interact: Option<ActionObject>,
	pub disabled: Option<bool>,
}

impl CustomMenuItem {
	pub fn new(text: impl ToString, icon: Icon) -> Self {
		CustomMenuItem {
			icon,
			text: text.to_string(),
			interact: None,
			disabled: None,
		}
	}
}

impl MenuComponent {
	pub fn new(block_id: i64) -> Self {
		Self {
			cid: "menu".to_string(),
			block_id,
			notifications_enabled: None,
			delete: None,
			permissions: None,
			star_button: None,
			custom: None,
		}
	}
}

impl MenuComponent {
	pub fn from_block(block: &Block, user_id: i32) -> Self {
		let mut menu = MenuComponent::new(block.id);

		if has_perm_level(user_id, &block, PermLevel::View) {
			menu.notifications_enabled = Some(block.notif_enabled.contains(&user_id));
			menu.star_button = Some(StarButton {
				count: block.stars.len(),
				starred: block.stars.contains(&user_id),
			});
			let public = if has_perm_level(user_id, &block, PermLevel::Full) {
				Some(block.public)
			} else {
				None
			};
			menu.permissions = Some(PermissionsList {
				public,
				full: block.perm_full.len(),
				edit: block.perm_edit.len(),
				view: block.perm_view.len(),
			});
		}

		if has_perm_level(user_id, &block, PermLevel::Owner) {
			menu.delete = Some(true);
		}

		menu
	}
}
