use block_tools::{
	auth::{
		optional_token, optional_validate_token,
		permissions::{has_perm_level, PermLevel},
	},
	blocks::Context,
	display_api::{
		component::{
			atomic::{icon::Icon, text::TextComponent},
			layout::{
				card::{CardComponent, CardHeader, DetachedMenu},
				stack::StackComponent,
			},
			menus::menu::{CustomMenuItem, MenuComponent},
			DisplayComponent, WrappedComponent,
		},
		ActionObject,
	},
	models::Block,
	LoopError,
};

use crate::{blocks::group_block::GroupBlock, delegation::display::delegate_embed_display};

impl GroupBlock {
	pub fn handle_embed_display(
		block: &Block,
		context: &Context,
	) -> Result<DisplayComponent, LoopError> {
		let conn = &context.conn()?;
		let user_id = optional_validate_token(optional_token(context))?;

		let Self {
			name,
			description,
			items,
		} = Self::from_id(block.id, user_id, conn)?;

		let name = name
			.and_then(|block| block.block_data)
			.unwrap_or_else(|| "Untitled Group".into());
		let description = description.and_then(|block| block.block_data);

		let mut wrapped_items = vec![];
		for (item, property_id) in items {
			let component = delegate_embed_display(&item, context);
			if let DisplayComponent::Card(card) = component {
				let remove_method = Self::build_remove_method_object(block.id, property_id);
				let mut remove_item = CustomMenuItem::new("Remove this item", Icon::Minus);
				remove_item.interact = Some(ActionObject::method(remove_method));
				remove_item.listed = Some(true);
				if let Some(header) = card.header {
					if let Some(menu) = header.menu {
						if let Some(custom) = menu.custom {
							let mut custom = custom;
							custom.push(remove_item);
							wrapped_items.push(WrappedComponent::from(
								CardComponent {
									header: Some(CardHeader {
										menu: Some(MenuComponent {
											custom: Some(custom),
											..menu
										}),
										..header
									}),
									..card
								}
								.into(),
							));
						} else {
							let custom = vec![remove_item];
							wrapped_items.push(WrappedComponent::from(
								CardComponent {
									header: Some(CardHeader {
										menu: Some(MenuComponent {
											custom: Some(custom),
											..menu
										}),
										..header
									}),
									..card
								}
								.into(),
							));
						}
					} else {
						wrapped_items.push(WrappedComponent::from(
							CardComponent {
								header: Some(header),
								..card
							}
							.into(),
						));
					}
				} else if let Some(detached) = card.detached_menu {
					if let Some(custom) = detached.menu.custom {
						let mut custom = custom;
						custom.push(remove_item);
						wrapped_items.push(WrappedComponent::from(
							CardComponent {
								detached_menu: Some(DetachedMenu {
									menu: MenuComponent {
										custom: Some(custom),
										..detached.menu
									},
									..detached
								}),
								..card
							}
							.into(),
						));
					} else {
						let custom = vec![remove_item];
						wrapped_items.push(WrappedComponent::from(
							CardComponent {
								detached_menu: Some(DetachedMenu {
									menu: MenuComponent {
										custom: Some(custom),
										..detached.menu
									},
									..detached
								}),
								..card
							}
							.into(),
						));
					}
				} else {
					wrapped_items.push(WrappedComponent::from(card.into()));
				}
			} else {
				wrapped_items.push(WrappedComponent::from(component));
			}
		}

		let stack: DisplayComponent = if wrapped_items.is_empty() {
			TextComponent::info("No items in group").into()
		} else {
			StackComponent {
				items: wrapped_items,
				..StackComponent::fit()
			}
			.into()
		};
		let mut content = StackComponent::vertical();

		if let Some(description) = description {
			content.push(TextComponent::new(description))
		}
		content.push(stack);

		let mut header = CardHeader {
			block_id: Some(block.id.to_string()),
			icon: Some(Icon::Folder),
			..CardHeader::new(name)
		};

		if let Some(user_id) = user_id {
			let mut menu = MenuComponent::from_block(block, user_id);
			menu.load_comments(conn)?;
			if has_perm_level(user_id, block, PermLevel::Edit) {
				let add_action = Self::build_add_action_object(block.id);
				let mut add_item = CustomMenuItem::new("Add a Block", Icon::Plus);
				add_item.interact = Some(add_action);
				menu.custom = Some(vec![add_item]);
			}
			header.menu = Some(menu);
		}

		Ok(CardComponent {
			color: block.color.clone(),
			header: Some(header),
			..CardComponent::new(content)
		}
		.into())
	}
}
