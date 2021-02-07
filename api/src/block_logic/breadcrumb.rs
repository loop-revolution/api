use async_graphql::*;
use block_tools::{auth::permissions::maybe_use_view, use_diesel::prelude::*};
use block_tools::{
	blocks::Context,
	models::{Block, Property},
	schema::properties,
	Error,
};
use block_types::{blocks::group_block, delegation::display::delegate_block_name};

#[derive(SimpleObject, Clone)]
pub struct BreadCrumb {
	pub block_id: i64,
	pub name: String,
}

pub fn gen_breadcrumb(context: &Context, block: &Block) -> Result<Vec<BreadCrumb>, Error> {
	let mut crumbs = cycle(context, block, vec![], vec![])?.0;
	crumbs.reverse();
	Ok(crumbs)
}

fn cycle(
	context: &Context,
	block: &Block,
	mut crumbs: Vec<BreadCrumb>,
	mut blocks_added: Vec<i64>,
) -> Result<(Vec<BreadCrumb>, Vec<i64>), Error> {
	let conn = &context.pool.get()?;
	let parent_props: Vec<Property> = properties::dsl::properties
		.filter(properties::value_id.eq(block.id))
		.get_results(conn)?
		.into_iter()
		.filter(|prop: &Property| !blocks_added.contains(&prop.parent_id))
		.collect();
	let (parent_prop, parent) = allowed_parent(parent_props, context)?;
	let add_this = |mut crumbs: Vec<BreadCrumb>, blocks_added: Vec<i64>| {
		crumbs.push(BreadCrumb {
			block_id: block.id,
			name: delegate_block_name(context, &block.block_type, block)?,
		});
		Ok((crumbs, blocks_added))
	};
	let parent = match parent {
		Some(parent) => parent,
		None => return add_this(crumbs, blocks_added),
	};
	let parent_prop = match parent_prop {
		Some(prop) => prop,
		None => return add_this(crumbs, blocks_added),
	};
	if parent.block_type == group_block::BLOCK_NAME {
		crumbs.push(BreadCrumb {
			block_id: block.id,
			name: delegate_block_name(context, &block.block_type, block)?,
		})
	} else {
		crumbs.push(BreadCrumb {
			block_id: block.id,
			name: parent_prop.property_name.clone(),
		})
	}
	blocks_added.push(block.id);
	let next_cycle = cycle(context, &parent, crumbs, blocks_added)?;
	Ok((next_cycle.0, next_cycle.1))
}

fn allowed_parent(
	parent_props: Vec<Property>,
	context: &Context,
) -> Result<(Option<Property>, Option<Block>), Error> {
	let conn = &context.pool.get()?;
	let mut parent: Option<Block> = None;
	let mut parent_prop: Option<Property> = None;
	for candidate in parent_props {
		let block = Block::by_id(candidate.parent_id, conn)?;
		let block = maybe_use_view(context, block)?;
		if let Some(block) = block {
			parent = Some(block);
			parent_prop = Some(candidate);
			break;
		}
	}

	Ok((parent_prop, parent))
}
