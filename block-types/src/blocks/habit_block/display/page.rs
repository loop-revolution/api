use crate::blocks::habit_block::HabitBlock;
use block_tools::{
	auth::{
		optional_token, optional_validate_token,
		permissions::{has_perm_level, PermLevel},
	},
	blocks::Context,
	display_api::{
		component::{
			atomic::text::TextComponent, layout::stack::StackComponent,
			misc::richtext::RichTextComponent,
		},
		DisplayMeta, DisplayObject,
	},
	models::Block,
	LoopError,
};

impl HabitBlock {
	pub fn handle_page_display(
		block: &Block,
		context: &Context,
	) -> Result<DisplayObject, LoopError> {
		let conn = &context.conn()?;
		let user_id = optional_validate_token(optional_token(context))?;

		// Build the properties
		let data = Self::from_id(block.id, user_id, conn)?;

		let mut content = StackComponent::vertical();

		let mut editable = false;
		if let Some(user_id) = user_id {
			if has_perm_level(user_id, block, PermLevel::Edit) {
				editable = true;
			}
		}

		content.push(Self::streak(data.streak.clone()));
		content.push(TextComponent {
			bold: Some(true),
			..TextComponent::new("Description")
		});

		if let Some(desc) = Self::description(&data.description, editable) {
			content.push(RichTextComponent {
				bordered: Some(true),
				..desc
			})
		}

		let meta = DisplayMeta {
			page: Some(data.page_meta(block, user_id)),
			..Default::default()
		};
		Ok(DisplayObject {
			meta: Some(meta),
			..DisplayObject::new(content)
		})
	}
}
