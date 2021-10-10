use crate::blocks::task_block::TaskBlock;
use block_tools::display_api::component::atomic::icon::{Icon, IconComponent};

impl TaskBlock {
	pub fn icon(status_index: u8) -> IconComponent {
		let progress_color = match status_index {
			2 => Some("#393939".to_string()),
			1 => Some("#ffa31d".to_string()),
			_ => None,
		};
		IconComponent {
			color: progress_color,
			..IconComponent::new(Icon::TaskComplete)
		}
	}
}
