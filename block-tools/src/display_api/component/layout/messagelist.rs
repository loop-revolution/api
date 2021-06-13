use crate::display_api::{
	component::{menus::menu::CustomMenuItem, DisplayComponent},
	MethodObject,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct MessageListComponent {
	pub messages: Vec<MessageListMessage>,
	pub send_method: Option<MethodObject>,
	pub input_name: Option<String>,
	pub input_placeholder: Option<String>,
	pub color: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageListMessage {
	pub component: DisplayComponent,
	pub author_display_name: String,
	pub author_username: String,
	pub sent_at: DateTime<Utc>,
	pub stars: Option<i32>,
	pub menu: Option<MessageListMessageMenu>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct MessageListMessageMenu {
	pub open_block_page: Option<String>,
	pub delete_block: Option<String>,
	pub custom: Option<Vec<CustomMenuItem>>,
}

impl MessageListMessage {
	pub fn new(
		component: impl Into<DisplayComponent>,
		author_display_name: String,
		author_username: String,
	) -> Self {
		MessageListMessage {
			component: component.into(),
			author_display_name,
			author_username,
			sent_at: Utc::now(),
			stars: None,
			menu: None,
		}
	}
}

impl From<MessageListComponent> for DisplayComponent {
	fn from(c: MessageListComponent) -> Self {
		DisplayComponent::MessageList(c)
	}
}
