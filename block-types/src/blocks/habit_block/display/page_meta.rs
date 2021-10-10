use block_tools::{
	auth::permissions::{has_perm_level, PermLevel},
	display_api::{
		component::{
			form::input::{InputComponent, InputSize},
			layout::stack::{AlignYOptions, StackComponent},
			menus::menu::MenuComponent,
		},
		PageMeta,
	},
	models::Block,
};

use crate::blocks::{data_block::DataBlock, habit_block::HabitBlock};

impl HabitBlock {
	pub fn page_meta(&self, block: &Block, user_id: Option<i32>) -> PageMeta {
		let mut page = PageMeta {
			header: Some(
				self.name
					.clone()
					.and_then(|block| block.block_data)
					.unwrap_or_else(|| "Untitled Habit".into()),
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
					header.push(Self::score_circle(self.score.clone(), block));
					header.push(InputComponent {
						label: Some("Habit Name".into()),
						size: Some(InputSize::Medium),
						..DataBlock::masked_editable_data(
							name.id.to_string(),
							name.block_data.clone(),
							true,
						)
					});
					header.push(Self::buttons_stack(self.impact.clone(), block.id));
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
