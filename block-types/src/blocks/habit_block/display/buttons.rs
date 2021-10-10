use crate::blocks::habit_block::HabitBlock;
use block_tools::{
	display_api::{
		colors::ColorScheme,
		component::{
			atomic::icon::Icon,
			interact::button::{ButtonComponent, ButtonSize, ButtonVariant},
			layout::stack::StackComponent,
		},
	},
	models::Block,
};

impl HabitBlock {
	pub fn buttons_stack(impact: Option<Block>, block_id: i64) -> StackComponent {
		let impact = impact
			.and_then(|block| block.block_data)
			.unwrap_or_else(|| "positive".into());
		let plus_button = ButtonComponent {
			icon: Some(Icon::ThumbsUp),
			color_scheme: Some(ColorScheme::Green),
			variant: Some(ButtonVariant::Outline),
			size: Some(ButtonSize::Small),
			interact: Some(Self::build_plus_action_object(block_id)),
			..ButtonComponent::new("+1")
		};
		let minus_button = ButtonComponent {
			icon: Some(Icon::ThumbsDown),
			color_scheme: Some(ColorScheme::Red),
			variant: Some(ButtonVariant::Outline),
			size: Some(ButtonSize::Small),
			interact: Some(Self::build_minus_action_object(block_id)),
			..ButtonComponent::new("-1")
		};
		let mut buttons_stack = StackComponent::fit();
		match impact.as_str() {
			"negative" => {
				buttons_stack.push(minus_button);
			}
			"either" => {
				buttons_stack.push(plus_button);
				buttons_stack.push(minus_button);
			}
			_ => {
				buttons_stack.push(plus_button);
			}
		}
		buttons_stack
	}
}
