use crate::{blocks::task_block::TaskBlock, delegation::display::delegate_embed_display};
use block_tools::{
	auth::{
		optional_token, optional_validate_token,
		permissions::{has_perm_level, PermLevel},
	},
	blocks::Context,
	display_api::{
		colors::ColorScheme,
		component::{
			atomic::{icon::Icon, text::TextComponent},
			interact::button::{ButtonComponent, ButtonSize, ButtonVariant},
			layout::stack::{AlignYOptions, StackComponent},
			misc::richtext::RichTextComponent,
		},
		DisplayMeta, DisplayObject,
	},
	models::Block,
	LoopError,
};

impl TaskBlock {
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

		content.push(Self::assigned_to_tag(data.assigned_user(conn)?, block.id));

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

		// Dependency logic
		let mut blocked_by = vec![];
		for dep in data.deps.clone() {
			blocked_by.push(delegate_embed_display(&dep, context));
		}
		let is_blocked = !blocked_by.is_empty();

		let mut dep_section_title = StackComponent::fit();
		dep_section_title.align_y = Some(AlignYOptions::Middle);
		dep_section_title.push(TextComponent {
			bold: Some(true),
			..TextComponent::new("Dependencies")
		});
		let add_dep_btn = ButtonComponent {
			icon: Some(Icon::Plus),
			interact: Some(Self::build_add_action_object(block.id)),
			size: Some(ButtonSize::Small),
			variant: Some(ButtonVariant::Outline),
			color_scheme: Some(ColorScheme::Red),
			..ButtonComponent::new("")
		};
		dep_section_title.push(add_dep_btn);

		content.push(dep_section_title);

		if is_blocked {
			let mut stack = StackComponent::fit();
			for task in blocked_by {
				stack.push(task);
			}
			content.push(stack);
		} else {
			content.push(TextComponent::info("No dependencies"))
		}

		let meta = DisplayMeta {
			page: Some(data.page_meta(block, user_id, is_blocked)),
			..Default::default()
		};
		Ok(DisplayObject {
			meta: Some(meta),
			..DisplayObject::new(content)
		})
	}
}
