use block_tools::{
	blocks::Context,
	models::{Block, User},
	LoopError,
};

use super::super::GroupBlock;
use super::create::{CreationArgs, Item};

pub fn create_root(context: &Context, user: User, first_block_id: i64) -> Result<Block, LoopError> {
	let conn = context.conn()?;

	let mut args = CreationArgs {
		name: Some("Dashboard".into()),
		..CreationArgs::default()
	};
	args.items.push(Item::from(first_block_id));

	let block = GroupBlock::handle_create(args, context, user.id)?;
	user.update_root(Some(block.id), &conn)?;

	Ok(block)
}
