use async_graphql::*;
use block_tools::{auth::permissions::maybe_use_view, use_diesel::prelude::*};
use block_tools::{
	blocks::Context,
	models::{Block, Property},
	schema::properties,
	LoopError,
};
use block_types::delegation::display::delegate_block_name;

#[derive(SimpleObject, Clone)]
/// One section of a block's breadcrumb. There should be multiple of these
/// to put together. When returned, its ordered so that the block this stems
/// from is last. e.g 0/1/2/3
pub struct BreadCrumb {
	/// The id of the block in the breadcrumb
	pub block_id: i64,
	/// How to display the name of the block in the breadcrumb
	pub name: String,
}

/// Generates a vector of crumbs for a single block
pub fn gen_breadcrumb(context: &Context, block: &Block) -> Result<Vec<BreadCrumb>, LoopError> {
	// Generate the breadcrumbs (it's recursive)
	let mut crumbs = cycle(context, block, vec![], vec![])?.0;
	// Reverse the order of the breadcrumbs b/c it would otherwise
	// be often reversed on the frontend anyways.
	crumbs.reverse();
	Ok(crumbs)
}

/// A recursive function that builds a breadcrumb. There may or may not be
/// performance issues to solve
fn cycle(
	context: &Context,
	block: &Block,
	mut crumbs: Vec<BreadCrumb>,
	mut blocks_added: Vec<i64>,
) -> Result<(Vec<BreadCrumb>, Vec<i64>), LoopError> {
	let conn = &context.conn()?;

	let parent_props: Vec<Property> = properties::dsl::properties
		.filter(properties::value_id.eq(block.id))
		.get_results(conn)?
		.into_iter()
		// Remove the parents that have already been added
		.filter(|prop: &Property| !blocks_added.contains(&prop.parent_id))
		.collect();
	let (parent_prop, parent) = allowed_parent(parent_props, context)?;

	// Helper function for when there are no parents
	let add_this = |mut crumbs: Vec<BreadCrumb>, blocks_added: Vec<i64>| {
		// Add the final block that its on
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
	// If the property is an item (like in a group) then the name should
	// be the name of the block
	if parent_prop.property_name == "item" {
		crumbs.push(BreadCrumb {
			block_id: block.id,
			name: delegate_block_name(context, &block.block_type, block)?,
		})
	// If not, then the name of the block displayed should be the property name
	} else {
		crumbs.push(BreadCrumb {
			block_id: block.id,
			name: parent_prop.property_name,
		})
	}
	// Add the block to the list of blocks added
	blocks_added.push(block.id);
	// Cycle to further in the crumb cycle
	let next_cycle = cycle(context, &parent, crumbs, blocks_added)?;
	Ok((next_cycle.0, next_cycle.1))
}

/// From a list of properties, this returns the property & block that
/// the user has access to (if any)
fn allowed_parent(
	parent_props: Vec<Property>,
	context: &Context,
) -> Result<(Option<Property>, Option<Block>), LoopError> {
	let conn = &context.conn()?;

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
