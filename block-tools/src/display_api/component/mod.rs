use erased_serde::Serialize as ErasedSeralize;
use serde::{
	ser::{SerializeStruct, Serializer},
	Deserialize, Serialize,
};

pub mod atomic;
pub mod data;
pub mod form;
pub mod interact;
pub mod layout;
pub mod menu;
pub mod misc;

#[derive(Deserialize, Debug, Clone)]
pub enum DisplayComponent {
	// Atomic
	Badge(atomic::badge::BadgeComponent),
	Icon(atomic::icon::IconComponent),
	Text(atomic::text::TextComponent),
	// Data Display
	Progress(data::progress::ProgressComponent),
	// Form
	Blocklist(form::blocklist::BlocklistComponent),
	CheckBox(form::checkbox::CheckboxComponent),
	Dropdown(form::dropdown::DropdownComponent),
	Input(form::input::InputComponent),
	StickyToggleButton(form::stickytogglebutton::StickyToggleButtonComponent),
	// Interact
	Button(interact::button::ButtonComponent),
	Link(interact::link::LinkComponent),
	// Layout
	Card(layout::card::CardComponent),
	DisplayList(layout::displaylist::DisplayListComponent),
	Stack(layout::stack::StackComponent),
	// Menu
	ActionPopover(menu::actionpopover::ActionPopoverComponent),
	// Misc
	RichText(misc::richtext::RichTextComponent),
}

impl Serialize for DisplayComponent {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		let mut state = serializer.serialize_struct("Component", 2)?;
		state.serialize_field("cid", &self.cid())?;
		state.serialize_field("args", &self.args())?;
		state.end()
	}
}

impl DisplayComponent {
	pub fn cid(&self) -> &str {
		match self {
			Self::ActionPopover(_) => "actionpopover",
			Self::Badge(_) => "badge",
			Self::Blocklist(_) => "blocklist",
			Self::Button(_) => "button",
			Self::Card(_) => "card",
			Self::CheckBox(_) => "checkbox",
			Self::DisplayList(_) => "displaylist",
			Self::Dropdown(_) => "dropdown",
			Self::Icon(_) => "icon",
			Self::Input(_) => "input",
			Self::Link(_) => "link",
			Self::Progress(_) => "progress",
			Self::RichText(_) => "richtext",
			Self::Stack(_) => "stack",
			Self::StickyToggleButton(_) => "stickytogglebutton",
			Self::Text(_) => "text",
		}
	}
	pub fn args(&self) -> &dyn ErasedSeralize {
		match self {
			Self::Badge(a) => a,
			Self::Icon(a) => a,
			Self::ActionPopover(a) => a,
			Self::Blocklist(a) => a,
			Self::Button(a) => a,
			Self::Card(a) => a,
			Self::CheckBox(a) => a,
			Self::DisplayList(a) => a,
			Self::Dropdown(a) => a,
			Self::Input(a) => a,
			Self::Link(a) => a,
			Self::Progress(a) => a,
			Self::RichText(a) => a,
			Self::Stack(a) => a,
			Self::StickyToggleButton(a) => a,
			Self::Text(a) => a,
		}
	}
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WrappedComponent {
	pub component: DisplayComponent,
}

impl WrappedComponent {
	pub fn from(component: DisplayComponent) -> Self {
		WrappedComponent { component }
	}
}
