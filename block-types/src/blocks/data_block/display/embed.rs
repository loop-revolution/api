use block_tools::{
	auth::{optional_token, optional_validate_token},
	blocks::Context,
	display_api::component::{
		atomic::{icon::Icon, text::TextComponent},
		layout::card::{CardComponent, CardHeader},
		menus::menu::MenuComponent,
		DisplayComponent,
	},
	models::Block,
	LoopError,
};

use crate::blocks::data_block::DataBlock;

impl DataBlock {
	pub fn handle_embed_display(
		block: &Block,
		context: &Context,
	) -> Result<DisplayComponent, LoopError> {
		let conn = &context.conn()?;
		let user_id = optional_validate_token(optional_token(context)).unwrap();
		let data: Option<String> = block.block_data.clone();

		let card_content = TextComponent::new(&data.unwrap_or_else(|| "".into()));

		let mut header = CardHeader {
			icon: Some(Icon::Box),
			block_id: Some(block.id.to_string()),
			..CardHeader::new("Data")
		};
		if let Some(user_id) = user_id {
			let mut menu = MenuComponent::from_block(block, user_id);
			menu.load_comments(conn)?;
			header.menu = Some(menu);
		}

		Ok(CardComponent {
			color: block.color.clone(),
			header: Some(header),
			..CardComponent::new(card_content)
		}
		.into())
	}
}
