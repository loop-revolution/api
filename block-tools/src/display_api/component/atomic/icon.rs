use crate::display_api::{colors::ColorScheme, component::DisplayComponent};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IconComponent {
	pub icon: Icon,
	pub color: Option<String>,
	pub color_scheme: Option<ColorScheme>,
	pub size: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Icon {
	Anchor,
	Archive,
	Award,
	Book,
	Bookmark,
	Box,
	Briefcase,
	Calendar,
	Camera,
	Edit,
	Eye,
	Feed,
	File,
	FileText,
	Film,
	Filter,
	Flag,
	Folder,
	Gift,
	Heart,
	Image,
	Info,
	Key,
	Lock,
	Map,
	MapPin,
	Message,
	Minus,
	Plus,
	Send,
	TaskComplete,
	ThumbsDown,
	ThumbsUp,
	Trash,
	Type,
	Unlock,
}

impl IconComponent {
	pub fn new(icon: Icon) -> Self {
		Self {
			icon,
			color: None,
			color_scheme: None,
			size: None,
		}
	}
}

impl fmt::Display for Icon {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{:?}", self)
	}
}

impl From<Icon> for IconComponent {
	fn from(icon: Icon) -> Self {
		Self {
			icon,
			color: None,
			color_scheme: None,
			size: None,
		}
	}
}

impl From<Icon> for String {
	fn from(icon: Icon) -> Self {
		icon.to_string()
	}
}

impl From<IconComponent> for DisplayComponent {
	fn from(component: IconComponent) -> Self {
		DisplayComponent::Icon(component)
	}
}
