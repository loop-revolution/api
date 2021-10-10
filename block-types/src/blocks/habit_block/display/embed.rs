use crate::blocks::habit_block::HabitBlock;
use block_tools::{
	auth::{
		optional_token, optional_validate_token,
		permissions::{has_perm_level, PermLevel},
	},
	blocks::Context,
	display_api::component::{
		atomic::text::TextComponent,
		interact::link::LinkComponent,
		layout::{
			card::{CardComponent, DetachedMenu},
			stack::{AlignXOptions, StackComponent},
		},
		menus::menu::MenuComponent,
		DisplayComponent,
	},
	models::Block,
	LoopError,
};

impl HabitBlock {
	pub fn handle_embed_display(
		block: &Block,
		context: &Context,
	) -> Result<DisplayComponent, LoopError> {
		let conn = &context.conn()?;
		let user_id = optional_validate_token(optional_token(context))?;

		let Self {
			name,
			description,
			impact,
			score,
			streak,
		} = Self::from_id(block.id, user_id, conn)?;

		let mut left_col = StackComponent::vertical();
		let mut middle_col = StackComponent::vertical();
		let mut right_col = StackComponent {
			align_x: Some(AlignXOptions::Right),
			..StackComponent::vertical()
		};

		let name = name
			.and_then(|block| block.block_data)
			.unwrap_or_else(|| "Untitled Habit".into());
		let text = TextComponent {
			bold: Some(true),
			..TextComponent::new(name)
		};
		let link = LinkComponent {
			app_path: Some(format!("/b/{}", block.id)),
			no_style: Some(true),
			..LinkComponent::new(text)
		};
		middle_col.push(link.clone());

		let mut desc_component = None;

		if let Some(desc) = Self::description(&description, false) {
			middle_col.push(desc.clone());
			desc_component = Some(desc);
		}

		let mut detached_menu = None;
		let mut action_buttons = None;

		if let Some(user_id) = user_id {
			if has_perm_level(user_id, block, PermLevel::Edit) {
				let buttons_stack = Self::buttons_stack(impact, block.id);
				right_col.push(buttons_stack.clone());
				action_buttons = Some(buttons_stack);
				let mut menu = MenuComponent::from_block(block, user_id);
				menu.load_comments(conn)?;
				detached_menu = Some(DetachedMenu::bottom_right(menu));
			}
		}
		let mut mobile_first_row = StackComponent::horizontal();
		let score_circle = Self::score_circle(score, block);
		let streak = Self::streak(streak);

		mobile_first_row.push(score_circle.clone());

		let mut mobile_info = StackComponent::vertical();
		mobile_info.push(link);
		mobile_info.push(streak.clone());
		mobile_first_row.push(mobile_info);

		right_col.push(streak);
		left_col.push(score_circle);

		let mut content = StackComponent::horizontal();
		content.push(left_col);
		content.push(middle_col);
		content.push(right_col);

		let mut mobile_content = StackComponent::vertical();
		mobile_content.push(mobile_first_row);
		if let Some(btns) = action_buttons {
			mobile_content.push(btns);
		}
		if let Some(desc) = desc_component {
			mobile_content.push(desc);
		}

		let mobile_card = CardComponent {
			color: block.color.clone(),
			detached_menu: detached_menu.clone(),
			..CardComponent::new(mobile_content)
		};

		Ok(CardComponent {
			color: block.color.clone(),
			detached_menu,
			mobile_override: Some(box mobile_card),
			..CardComponent::new(content)
		}
		.into())
	}
}
