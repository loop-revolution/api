use block_tools::{
	auth::{
		optional_token, optional_validate_token,
		permissions::{has_perm_level, PermLevel},
	},
	blocks::Context,
	display_api::{
		component::{
			atomic::text::TextComponent, form::input::InputSize, menus::menu::MenuComponent,
			DisplayComponent,
		},
		DisplayMeta, DisplayObject, PageMeta,
	},
	models::Block,
	LoopError,
};

use crate::blocks::data_block::DataBlock;

impl DataBlock {
	pub fn handle_page_display(
		block: &Block,
		context: &Context,
	) -> Result<DisplayObject, LoopError> {
		let conn = &context.conn()?;
		let user_id = optional_validate_token(optional_token(context))?;

		// Make access to data details easier
		let data = block.block_data.clone().unwrap_or_default();

		// Display API to render
		let mut component: DisplayComponent = TextComponent::new(&data).into();
		let mut page = PageMeta {
			title: Some("Data".to_string()),
			header: Some(format!("Data Block #{}", block.id)),
			..Default::default()
		};

		if let Some(user_id) = user_id {
			let mut menu = MenuComponent::from_block(block, user_id);
			menu.load_comments(conn)?;
			// Add a menu to the page
			page.menu = Some(menu);
			// If the user can edit it the data, make it possible to edit
			if has_perm_level(user_id, block, PermLevel::Edit) {
				let mut input = Self::masked_editable_data(
					block.id.to_string(),
					block.block_data.clone(),
					false,
				);
				input.initial_value = Some(data);
				input.label = Some("Data".to_string());
				input.size = Some(InputSize::MultiLine);
				component = input.into();
			}
		}

		let meta = DisplayMeta {
			page: Some(page),
			..DisplayMeta::default()
		};
		Ok(DisplayObject {
			meta: Some(meta),
			..DisplayObject::new(component)
		})
	}
}
