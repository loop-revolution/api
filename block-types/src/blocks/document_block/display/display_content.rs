use crate::{
	blocks::{document_block::DocumentBlock, text_block::TextBlock},
	delegation::display::delegate_embed_display,
};
use block_tools::{
	auth::permissions::{has_perm_level, PermLevel},
	blocks::Context,
	display_api::component::{
		atomic::{icon::Icon, text::TextComponent},
		layout::displaylist::{DisplayListComponent, DisplayListItem},
		menus::menu::{CustomMenuItem, MenuComponent},
		DisplayComponent,
	},
	models::{Block, NewBlock},
	LoopError,
};

impl DocumentBlock {
	pub fn display_content(
		user_id: Option<i32>,
		block: &Block,
		items: Vec<Block>,
		context: &Context,
	) -> Result<DisplayComponent, LoopError> {
		let conn = &context.conn()?;
		let mut list = DisplayListComponent {
			color: block.color.clone(),
			..Default::default()
		};
		for item in items {
			let component: DisplayComponent;
			if item.block_type == "text" {
				let mut text = TextBlock::rightfully_editable_richtext(user_id, &item);
				text.on_enter = Some(Self::build_add_method_object(block.id));
				component = text.into();
			} else {
				component = delegate_embed_display(&item, context);
			}
			let mut display_item = DisplayListItem::new(component);
			if let Some(user_id) = user_id {
				display_item.menu = Some(MenuComponent::from_block(&item, user_id));
			}
			list.items.push(display_item)
		}

		if list.items.is_empty() {
			if let Some(user_id) = user_id {
				let new_item = NewBlock {
					block_data: None,
					..NewBlock::new("text", user_id)
				}
				.insert(conn)?;
				block.make_property("item", new_item.id).insert(conn)?;
				let item = DisplayListItem {
					menu: Some(MenuComponent::from_block(&new_item, user_id)),
					..DisplayListItem::new(
						TextBlock::rightfully_editable_richtext(Some(user_id), &new_item).into(),
					)
				};
				list.items.push(item);
			} else {
				list.items.push(DisplayListItem::new(
					TextComponent::new("Empty document").into(),
				));
			}
		}

		// Adds "+" buttons to all parts of the list, but because of styling issues if left out
		// for now.
		if let Some(user_id) = user_id {
			if has_perm_level(user_id, block, PermLevel::Edit) {
				let mut custom = CustomMenuItem::new("Add a Block", Icon::Plus);
				let action = Self::build_add_block_action_object(block.id);
				custom.interact = Some(action);
				custom.listed = Some(true);

				list.items = list
					.items
					.into_iter()
					.map(|mut item| {
						item.menu = item.menu.map(|menu| MenuComponent {
							custom: Some(vec![custom.clone()]),
							..menu
						});
						item
					})
					.collect();
			}
		}

		Ok(list.into())
	}
}
