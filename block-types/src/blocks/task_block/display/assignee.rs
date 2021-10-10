use block_tools::{
	display_api::{
		colors::ColorScheme,
		component::{
			atomic::text::TextComponent,
			interact::button::{ButtonComponent, ButtonVariant},
			layout::stack::{AlignYOptions, StackComponent},
			menus::actionpopover::{ActionPopoverAction, ActionPopoverComponent},
		},
		ActionObject, RedirectObject,
	},
	models::User,
	use_diesel::PgConnection,
	LoopError,
};

use crate::blocks::task_block::TaskBlock;

impl TaskBlock {
	pub fn assigned_user(&self, conn: &PgConnection) -> Result<Option<User>, LoopError> {
		if let Some(assignee) = self.assignee.clone() {
			if let Some(user_id) = assignee.block_data {
				let user_id = user_id.parse::<i32>();
				if let Ok(user_id) = user_id {
					return User::by_id(user_id, conn);
				}
			}
		}
		Ok(None)
	}
}

impl TaskBlock {
	pub fn assignee_actionpopover(user: User, block_id: i64) -> ActionPopoverComponent {
		let view_profile = ActionPopoverAction {
			interact: Some(ActionObject::redirect(RedirectObject::app_path(format!(
				"u/{}",
				user.username
			)))),
			..ActionPopoverAction::new("View Profile")
		};
		let reassign = ActionPopoverAction {
			interact: Some(Self::build_assign_action_object(block_id)),
			..ActionPopoverAction::new("Reassign")
		};
		let trigger = ButtonComponent {
			variant: Some(ButtonVariant::Link),
			color_scheme: Some(ColorScheme::Blue),
			..ButtonComponent::new(user.username)
		};
		ActionPopoverComponent {
			trigger: Some(box trigger.into()),
			actions: vec![view_profile, reassign],
		}
	}
}

impl TaskBlock {
	pub fn assigned_to_tag(user: Option<User>, block_id: i64) -> StackComponent {
		let mut stack = StackComponent::fit();
		stack.align_y = Some(AlignYOptions::Middle);
		if let Some(user) = user {
			stack.push(TextComponent::info("Assigned to "));
			stack.push(Self::assignee_actionpopover(user, block_id));
		} else {
			stack.push(TextComponent::info("Unassigned - "));
			stack.push(ButtonComponent {
				variant: Some(ButtonVariant::Link),
				color_scheme: Some(ColorScheme::Blue),
				interact: Some(Self::build_assign_action_object(block_id)),
				..ButtonComponent::new("Assign")
			});
		}
		stack
	}
}
