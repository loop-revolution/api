use block_tools::{
	auth::permissions::{has_perm_level, PermLevel},
	display_api::{
		colors::ColorScheme,
		component::{
			atomic::{badge::BadgeComponent, text::TextComponent},
			form::input::{InputComponent, InputSize},
			layout::stack::{AlignYOptions, StackComponent},
			menus::menu::MenuComponent,
		},
		PageMeta,
	},
	models::Block,
};

use crate::blocks::{data_block::DataBlock, task_block::TaskBlock};

impl TaskBlock {
	pub fn page_meta(&self, block: &Block, user_id: Option<i32>, blocked: bool) -> PageMeta {
		let mut page = PageMeta {
			header: Some(
				self.name
					.clone()
					.and_then(|block| block.block_data)
					.unwrap_or_else(|| "Untitled Task".into()),
			),
			..Default::default()
		};

		if let Some(user_id) = user_id {
			let menu = MenuComponent::from_block(block, user_id);
			if let Some(name) = &self.name {
				// If the user can edit the name
				if has_perm_level(user_id, name, PermLevel::Edit) {
					let mut header = StackComponent {
						align_y: Some(AlignYOptions::Middle),
						..StackComponent::fit()
					};
					let status_index = Self::status_index(&self.status);
					header.push(Self::icon(status_index));
					let mut input = InputComponent {
						label: Some("Task Name".into()),
						size: Some(InputSize::Medium),
						..DataBlock::masked_editable_data(
							name.id.to_string(),
							name.block_data.clone(),
							true,
						)
					};
					if status_index == 2 {
						if let Some(mask) = input.mask {
							input.mask = Some(TextComponent {
								strikethrough: Some(true),
								..mask
							});
						}
					}
					header.push(input);
					if blocked {
						header.push(BadgeComponent {
							color_scheme: Some(ColorScheme::Orange),
							..BadgeComponent::new("Blocked")
						});
					}
					header.push(Self::status(&self.status, block.id));
					// Make the heading (which is the name) an input
					page.header_component = Some(header.into());
					// Remove the backup
					page.header = None;
				}
			}
			page.menu = Some(menu);
		}
		page
	}
}
