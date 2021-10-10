use block_tools::{
	auth::{optional_token, optional_validate_token},
	blocks::Context,
	display_api::{component::menus::menu::MenuComponent, DisplayMeta, DisplayObject, PageMeta},
	models::Block,
	LoopError,
};

use crate::blocks::text_block::TextBlock;

impl TextBlock {
	pub fn handle_page_display(
		block: &Block,
		context: &Context,
	) -> Result<DisplayObject, LoopError> {
		let conn = &context.conn()?;
		let user_id = optional_validate_token(optional_token(context))?;

		// Display API to render
		let component = Self::rightfully_editable_richtext(user_id, block);

		let mut page = PageMeta {
			title: Some("Text".to_string()),
			header: Some(format!("Text Block #{}", block.id)),
			..Default::default()
		};

		if let Some(user_id) = user_id {
			let mut menu = MenuComponent::from_block(block, user_id);
			menu.load_comments(conn)?;
			// Add a menu to the page
			page.menu = Some(menu);
		}

		let meta = DisplayMeta {
			page: Some(page),
			..Default::default()
		};
		Ok(DisplayObject {
			meta: Some(meta),
			..DisplayObject::new(component)
		})
	}
}
