use super::{
	block_types::{type_list, BlockType},
	breadcrumb::{gen_breadcrumb, BreadCrumb},
};
use crate::graphql::ContextData;
use async_graphql::{Context, Error, Object};
use block_tools::{
	auth::{optional_token, optional_validate_token, permissions::can_view},
	dsl::prelude::*,
	models::Block,
	schema::blocks,
};
use strsim::normalized_levenshtein;

#[derive(Default)]
pub struct BlockSearchQueries;

#[Object]
impl BlockSearchQueries {
	/// All the block types in the system
	async fn block_types(&self) -> Vec<BlockType> {
		type_list()
	}

	/// Finds blocks that are similar to the query provided. Matches against
	/// block breadcrumbs and sorts them by similarity. Needs some performance fixes
	async fn search_blocks(
		&self,
		context: &Context<'_>,
		query: String,
	) -> Result<Vec<Vec<BreadCrumb>>, Error> {
		let (context, conn) = &ContextData::parse(context)?;

		let user_id = optional_validate_token(optional_token(context))?;

		let mut helpers = blocks::dsl::blocks
			.load::<Block>(conn)?
			.into_iter()
			// Only blocks user has access to
			.filter(|block| can_view(user_id, block))
			.map(|block| {
				let crumbs = gen_breadcrumb(context, &block).unwrap_or_default();
				let crumb_string = crumbs
					.iter()
					.map(|crumb| crumb.name.as_str())
					.collect::<Vec<&str>>()
					.join("/");

				let mut sim = normalized_levenshtein(&crumb_string, &query);

				// If the block is data, make it less influencial
				if block.block_type == "data" {
					sim /= 2.;
				}
				BlockSortHelper {
					breadcrumb: crumbs,
					strsim: sim,
				}
			})
			.filter(|helper| helper.strsim != 0.)
			.collect::<Vec<BlockSortHelper>>();
		helpers.sort_by(|a, b| b.strsim.partial_cmp(&a.strsim).unwrap());

		Ok(helpers
			.into_iter()
			.map(|helper| helper.breadcrumb)
			.collect())
	}
}

struct BlockSortHelper {
	breadcrumb: Vec<BreadCrumb>,
	strsim: f64,
}
