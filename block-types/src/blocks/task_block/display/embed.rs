use crate::blocks::task_block::TaskBlock;
use block_tools::{
	auth::{
		optional_token, optional_validate_token,
		permissions::{has_perm_level, PermLevel},
	},
	blocks::Context,
	display_api::{
		component::{
			atomic::{badge::BadgeComponent, icon::Icon, text::TextComponent},
			form::dropdown::DropdownComponent,
			interact::{
				button::{ButtonComponent, ButtonSize, ButtonVariant},
				link::LinkComponent,
			},
			layout::{
				card::{CardComponent, DetachedMenu},
				stack::{AlignYOptions, SpacingOptions, StackComponent},
			},
			menus::menu::MenuComponent,
			DisplayComponent,
		},
		ActionObject, RedirectObject,
	},
	models::Block,
	LoopError,
};

impl TaskBlock {
	pub fn handle_embed_display(
		block: &Block,
		context: &Context,
	) -> Result<DisplayComponent, LoopError> {
		let conn = &context.conn()?;
		let user_id = optional_validate_token(optional_token(context))?;

		let task = Self::from_id(block.id, user_id, conn)?;
		let Self {
			name,
			description,
			status,
			deps,
			..
		} = task.clone();
		let status_index = Self::status_index(&status);

		let mut icon_col = StackComponent::vertical();
		let mut content_col = StackComponent::vertical();

		let mut first_row = StackComponent {
			spacing: Some(SpacingOptions::Between),
			..StackComponent::fit()
		};

		let mut name_badge_stack = StackComponent::fit();
		name_badge_stack.align_y = Some(AlignYOptions::Middle);

		let name = name
			.and_then(|block| block.block_data)
			.unwrap_or_else(|| "Untitled Task".into());
		let text = TextComponent {
			bold: Some(true),
			strikethrough: if status_index == 2 { Some(true) } else { None },
			..TextComponent::new(name)
		};
		let link = LinkComponent {
			app_path: Some(format!("/b/{}", block.id)),
			no_style: Some(true),
			..LinkComponent::new(text)
		};
		name_badge_stack.push(link);

		// Dependency logic
		let mut blocked_by = vec![];
		for dep in deps {
			let Self { status, name, .. } = Self::from_id(dep.id, user_id, conn)?;
			let status_index = Self::status_index(&status);
			// If it's not done, add to deps list
			if status_index != 2 {
				let name = name
					.and_then(|name| name.block_data)
					.unwrap_or_else(|| "Untitled Task".to_string());
				let redirect = RedirectObject::app_path(format!("b/{}", dep.id));
				let action = ActionObject::redirect(redirect);
				let button = ButtonComponent {
					icon: Some(Icon::TaskComplete),
					interact: Some(action),
					variant: Some(ButtonVariant::Outline),
					size: Some(ButtonSize::Small),
					..ButtonComponent::new(name)
				};
				blocked_by.push(button);
			}
		}
		if !blocked_by.is_empty() {
			let blocked = BadgeComponent::new("Blocked");
			name_badge_stack.push(blocked);
		}

		let mut info_stack = StackComponent::vertical();

		info_stack.push(name_badge_stack);

		info_stack.push(Self::assigned_to_tag(task.assigned_user(conn)?, block.id));

		let mut status_dropdown = DropdownComponent {
			disabled: Some(true),
			..Self::status(&status, block.id)
		};

		let mut detached_menu = None;

		if let Some(user_id) = user_id {
			if has_perm_level(user_id, block, PermLevel::Edit) {
				status_dropdown.disabled = Some(false)
			}
			let mut menu = MenuComponent::from_block(block, user_id);
			menu.load_comments(conn)?;
			detached_menu = Some(DetachedMenu::bottom_right(menu));
		}
		first_row.push(info_stack);
		first_row.push(status_dropdown);
		content_col.push(first_row);

		if let Some(desc) = Self::description(&description, false) {
			content_col.push(desc)
		}

		if !blocked_by.is_empty() {
			let mut blocked_row = StackComponent::fit();
			blocked_row.align_y = Some(AlignYOptions::Middle);
			blocked_row.push(TextComponent::info("Blocked by"));
			for button in blocked_by {
				blocked_row.push(button)
			}
			content_col.push(blocked_row);
		}

		icon_col.push(Self::icon(status_index));

		let mut content = StackComponent::horizontal();
		content.push(icon_col);
		content.push(content_col);

		Ok(CardComponent {
			color: if status_index == 2 {
				Some("#393939".to_string())
			} else {
				block.color.clone()
			},
			detached_menu,
			..CardComponent::new(content)
		}
		.into())
	}
}
