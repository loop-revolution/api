use crate::{
	blocks::{data_block::DataBlock, group_block::GroupBlock},
	delegation::display::delegate_embed_display,
};
use block_tools::{
	auth::{
		optional_token, optional_validate_token,
		permissions::{has_perm_level, PermLevel},
	},
	blocks::Context,
	display_api::{
		component::{
			atomic::{icon::Icon, text::TextComponent},
			form::input::{InputComponent, InputSize},
			layout::{
				card::{CardComponent, CardHeader, DetachedMenu},
				stack::StackComponent,
			},
			menus::menu::{CustomMenuItem, MenuComponent},
			DisplayComponent, WrappedComponent,
		},
		ActionObject, DisplayMeta, DisplayObject, PageMeta,
	},
	models::{Block, User},
	LoopError,
};

impl GroupBlock {
	pub fn handle_page_display(
		block: &Block,
		context: &Context,
	) -> Result<DisplayObject, LoopError> {
		let conn = &context.conn()?;
		let user_id = optional_validate_token(optional_token(context))?;
		let user = if let Some(id) = user_id {
			User::by_id(id, conn)?
		} else {
			None
		};
		let is_root = match &user {
			None => false,
			Some(user) => user.root_id == Some(block.id),
		};

		// Get all the blocks properties
		let Self {
			name,
			description,
			items,
		} = Self::from_id(block.id, user_id, conn)?;

		let name_string = name.clone().and_then(|block| block.block_data);
		let desc = description.clone().and_then(|block| block.block_data);

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
				..StackComponent::masonry()
			}
			.into()
		};
		let mut content = StackComponent::vertical();

		if !is_root {
			if let Some(desc) = desc {
				let block = description.unwrap();
				content.push(InputComponent {
					label: Some("Description".into()),
					size: Some(InputSize::MultiLine),
					..DataBlock::masked_editable_data(block.id.to_string(), Some(desc), false)
				})
			}
		}
		content.push(stack);

		let mut page = PageMeta::default();
		let header_backup = name_string.unwrap_or_else(|| "Untitled Group".into());

		if let Some(user) = user {
			let mut menu = MenuComponent::from_block(block, user.id);
			menu.load_comments(conn)?;
			page.menu = Some(menu);
			if !is_root {
				if let Some(name) = name {
					if has_perm_level(user.id, &name, PermLevel::Edit) {
						page.header_component = Some(
							InputComponent {
								label: Some("Group Name".into()),
								size: Some(InputSize::Medium),
								..DataBlock::masked_editable_data(
									name.id.to_string(),
									name.block_data,
									true,
								)
							}
							.into(),
						);
					} else {
						page.header = Some(header_backup)
					}
				}
			}
			if let Some(mut menu) = page.menu.clone() {
				if has_perm_level(user.id, block, PermLevel::Edit) {
					let action = Self::build_add_action_object(block.id);
					let item = CustomMenuItem {
						interact: Some(action),
						..CustomMenuItem::new("Add a Block", Icon::Plus)
					};
					menu.custom = Some(vec![item]);
					page.menu = Some(menu)
				}
			}
		} else {
			page.header = Some(header_backup)
		}

		let meta = DisplayMeta {
			page: Some(page),
			..Default::default()
		};
		Ok(DisplayObject {
			meta: Some(meta),
			..DisplayObject::new(content)
		})
	}
}
