use block_tools::{
	auth::permissions::{has_perm_level, PermLevel},
	display_api::{
		component::{
			atomic::icon::Icon,
			form::input::{InputComponent, InputSize},
			menus::menu::{CustomMenuItem, MenuComponent},
		},
		PageMeta,
	},
	models::Block,
};

use crate::blocks::{data_block::DataBlock, document_block::DocumentBlock};

impl DocumentBlock {
	pub fn page_meta(block: &Block, user_id: Option<i32>, name: Option<Block>) -> PageMeta {
		let mut page = PageMeta {
			header: Some(
				name.clone()
					.and_then(|block| block.block_data)
					.unwrap_or_else(|| "Untitled Document".into()),
			),
			..Default::default()
		};

		if let Some(user_id) = user_id {
			let mut menu = MenuComponent::from_block(block, user_id);
			if let Some(name) = name {
				// If the user can edit the name
				if has_perm_level(user_id, &name, PermLevel::Edit) {
					// Make the heading (which is the name) an input
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
					// Remove the backup
					page.header = None;
					// Add the "+" button
					let mut item = CustomMenuItem::new("Add a Section", Icon::Plus);
					let action = Self::build_add_action_object(block.id);
					item.interact = Some(action);
					menu.custom = Some(vec![item]);
				}
			}
			page.menu = Some(menu);
		}
		page
	}
}
