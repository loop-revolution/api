use block_tools::{
	blocks::Context,
	display_api::{
		component::{
			atomic::text::TextComponent,
			form::input::{InputComponent, InputSize},
			layout::stack::StackComponent,
			misc::richtext::RichTextComponent,
		},
		CreationObject,
	},
	LoopError,
};

use crate::blocks::habit_block::HabitBlock;

impl HabitBlock {
	pub fn handle_create_display(
		_context: &Context,
		_user_id: i32,
	) -> Result<CreationObject, LoopError> {
		let header = InputComponent {
			label: Some("Summary".to_string()),
			name: Some("NAME".to_string()),
			size: Some(InputSize::Large),
			..InputComponent::default()
		};
		// let pos_button = StickyToggleButtonComponent {
		// 	default_value: Some(true),
		// 	name: Some("POS".into()),
		// 	..StickyToggleButtonComponent::new(ButtonComponent {
		// 		icon: Some(Icon::ThumbsUp),
		// 		color_scheme: Some(ColorScheme::Green),
		// 		..ButtonComponent::new("Positive")
		// 	})
		// };
		// let neg_button = StickyToggleButtonComponent {
		// 	default_value: Some(false),
		// 	name: Some("NEG".into()),
		// 	..StickyToggleButtonComponent::new(ButtonComponent {
		// 		icon: Some(Icon::ThumbsDown),
		// 		color_scheme: Some(ColorScheme::Red),
		// 		..ButtonComponent::new("Negative")
		// 	})
		// };
		// let mut impact_buttons = StackComponent::fit();
		// impact_buttons.push(pos_button);
		// impact_buttons.push(neg_button);
		// let mut impact_section = StackComponent::vertical();
		// impact_section.push(TextComponent {
		// 	bold: Some(true),
		// 	..TextComponent::new("Effect")
		// });
		// impact_section.push(impact_buttons);

		let desc_input = RichTextComponent {
			editable: Some(true),
			name: Some("DESC".to_string()),
			bordered: Some(true),
			..Default::default()
		};
		let mut desc_section = StackComponent::vertical();
		desc_section.push(TextComponent {
			bold: Some(true),
			..TextComponent::new("Description")
		});
		desc_section.push(desc_input);

		let mut main = StackComponent::vertical();
		// main.push(impact_section);
		main.push(desc_section);

		let template: String = r#"{"name": $[NAME]$,"desc":$[DESC]$}"#.to_string();

		let object = CreationObject {
			header_component: header.into(),
			main_component: main.into(),
			input_template: template,
		};
		Ok(object)
	}
}
