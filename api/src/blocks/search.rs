use super::{
	block_types::{type_list, BlockType},
	breadcrumb::{gen_breadcrumb, BreadCrumb},
};
use crate::graphql::ContextData;
use async_graphql::{Context, Enum, Error, InputObject, Object, SimpleObject};
use block_tools::{
	auth::{optional_token, optional_validate_token, permissions::can_view},
	dsl::prelude::*,
	models::Block,
	schema::blocks,
};
use block_types::delegation::display::delegate_block_icon;
use std::time::SystemTime;
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
	/// block breadcrumbs and sorts them by similarity. Does not include `data` blocks
	/// by default.
	async fn search_blocks(
		&self,
		context: &Context<'_>,
		query: String,
		filters: Option<BlockSearchFilters>,
		#[graphql(desc = "Include data blocks with results?")] with_data: Option<bool>,
		sort_by: Option<BlockSortType>,
	) -> Result<Vec<BlockResult>, Error> {
		let (context, conn) = &ContextData::parse(context)?;

		let user_id = optional_validate_token(optional_token(context))?;
		let with_data = with_data.unwrap_or_default();
		let sort_by = sort_by.unwrap_or_default();

		let mut helpers = blocks::dsl::blocks
			.load::<Block>(conn)?
			.into_iter()
			// Only blocks user has access to
			.filter(|block| {
				if !with_data && block.block_type == "data" {
					return false;
				}
				if let Some(filters) = &filters {
					if let Some(block_type) = &filters.block_type {
						if &block.block_type != block_type {
							return false;
						}
					}
					if let Some(owner_id) = filters.owner_id {
						if block.owner_id != owner_id {
							return false;
						}
					}
					if let Some(user_id) = user_id {
						if let Some(only_show_starred) = filters.starred {
							if only_show_starred && !block.stars.contains(&user_id) {
								return false;
							}
						}
					}
				}
				can_view(user_id, block)
			})
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
				let result = BlockResult {
					crumbs,
					icon: delegate_block_icon(block.block_type).map(String::from),
					color: block.color,
					id: block.id,
				};
				BlockSortHelper {
					result,
					strsim: sim,
					star_count: block.stars.len(),
					updated_at: block.updated_at,
					created_at: block.created_at,
				}
			})
			.filter(|helper| helper.strsim != 0.)
			.collect::<Vec<BlockSortHelper>>();

		match sort_by {
			BlockSortType::Default => {
				helpers.sort_by(|a, b| b.strsim.partial_cmp(&a.strsim).unwrap());
			}
			BlockSortType::StarCount => {
				helpers.sort_by(|a, b| b.star_count.partial_cmp(&a.star_count).unwrap());
			}
			BlockSortType::Updated => {
				helpers.sort_by(|a, b| b.updated_at.partial_cmp(&a.updated_at).unwrap());
			}
			BlockSortType::Created => {
				helpers.sort_by(|a, b| b.created_at.partial_cmp(&a.created_at).unwrap());
			}
		}

		Ok(helpers.into_iter().map(|helper| helper.result).collect())
	}
}

struct BlockSortHelper {
	result: BlockResult,
	strsim: f64,
	star_count: usize,
	updated_at: SystemTime,
	created_at: SystemTime,
}

#[derive(SimpleObject)]
struct BlockResult {
	/// Breadcrumbs to render as a search result
	crumbs: Vec<BreadCrumb>,
	/// Icon to show to represent the block
	icon: Option<String>,
	/// Color of the block
	color: Option<String>,
	/// The ID of the block that was searched
	id: i64,
}

#[derive(InputObject, Default, Clone)]
/// Filters to help find a specific block
struct BlockSearchFilters {
	/// If true, will only include results that the user has starred
	starred: Option<bool>,
	/// Will only include blocks of the type provided. Note that data blocks
	/// will only be included with the `withData` parameter set to true.
	block_type: Option<String>,
	/// Will only include blocks owned by this user
	owner_id: Option<i32>,
}

#[derive(Enum, Copy, Clone, PartialEq, Eq)]
/// Custom ways to sort the results
enum BlockSortType {
	Default,
	StarCount,
	Updated,
	Created,
}

impl Default for BlockSortType {
	fn default() -> Self {
		Self::Default
	}
}
