use super::{
	block::BlockObject,
	block_types::{type_list, BlockType},
	breadcrumb::{gen_breadcrumb, BreadCrumb},
};
use crate::graphql::ContextData;
use async_graphql::{Context, Error, Object};
use block_tools::{
	auth::{
		optional_token, optional_validate_token,
		permissions::{can_view, maybe_use_view},
		require_token, validate_token,
	},
	dsl::prelude::*,
	models::Block,
	schema::blocks,
	NoAccessSubject, UserError,
};
use block_types::delegation::{
	display::delegate_creation_display,
	methods::{delegate_create, delegate_method},
};
use strsim::normalized_levenshtein;

#[derive(Default)]
pub struct BlockMutations;

#[Object]
impl BlockMutations {
	/// This mutation attempts to create a block of the block type specified.
	/// An authorization header is required to authenticate the creation, and
	/// a JSON input string should be passed based on the block type.
	pub async fn create_block(
		&self,
		context: &Context<'_>,
		#[graphql(desc = "Name of the block type to create.")] r#type: String,
		#[graphql(desc = "JSON string to specify what to create.")] input: String,
	) -> Result<BlockObject, Error> {
		let context = &context.data::<ContextData>()?;
		create_block(context, r#type, input).await
	}

	/// Executes a specific method created by a block type. Takes
	/// method name, block type, block id, and arguments for the
	/// method. Method name and args determine on the block type
	pub async fn block_method(
		&self,
		context: &Context<'_>,
		#[graphql(desc = "The block type that determines the method")] r#type: String,
		#[graphql(desc = "Arguments to help the method")] args: String,
		#[graphql(desc = "Name of the method on the block type")] method_name: String,
		#[graphql(desc = "ID of the block to act on")] block_id: i64,
	) -> Result<BlockObject, Error> {
		let context = &context.data::<ContextData>()?;
		Ok(delegate_method(&context.other(), r#type, args, method_name, block_id)?.into())
	}

	/// Deletes a block from the database. Currently does not delete everything,
	/// still has artifacts that may exist in the form of data blocks.
	pub async fn delete_block(
		&self,
		context: &Context<'_>,
		#[graphql(desc = "ID of the block to delete.")] block_id: i64,
	) -> Result<i64, Error> {
		let context = &context.data::<ContextData>()?;
		let user_id = validate_token(require_token(&context.other())?)?;
		let conn = &context.pool.get()?;
		match block_tools::dsl::delete(
			blocks::dsl::blocks
				.filter(blocks::id.eq(block_id))
				.filter(blocks::owner_id.eq(user_id)),
		)
		.execute(conn)?
		{
			0 => Err(Error::from(UserError::NoAccess(
				NoAccessSubject::DeleteBlock(block_id),
			))),
			_ => Ok(block_id),
		}
	}

	pub async fn update_visibility(
		&self,
		context: &Context<'_>,
		public: bool,
		#[graphql(desc = "ID of the block to update.")] block_id: i64,
	) -> Result<BlockObject, Error> {
		let context = &context.data::<ContextData>()?.other();
		let conn = &context.pool.get()?;
		let user_id = validate_token(require_token(context)?)?;
		let access_err: Error =
			UserError::NoAccess(NoAccessSubject::UpdatePermissions(block_id)).into();
		let block = match Block::by_id(block_id, conn)? {
			Some(block) => block,
			None => return Err(access_err),
		};
		if block.owner_id != user_id {
			return Err(access_err);
		}
		Ok(block.update_public(public, conn)?.into())
	}
}

pub async fn create_block(
	context: &ContextData,
	r#type: String,
	input: String,
) -> Result<BlockObject, Error> {
	let user_id = validate_token(require_token(&context.other())?)?;

	Ok(BlockObject::from(delegate_create(
		r#type.as_str(),
		input,
		&context.other(),
		user_id,
	)?))
}

#[derive(Default)]
pub struct BlockQueries;
#[Object]
impl BlockQueries {
	/// Tries to find a block with a matching ID. Will be null if a block is not found.
	async fn block_by_id(
		&self,
		context: &Context<'_>,
		#[graphql(desc = "ID of the block to try to find.")] id: i64,
	) -> Result<Option<BlockObject>, Error> {
		let context = &context.data::<ContextData>()?.other();
		let conn = &context.pool.get()?;
		let block = maybe_use_view(context, Block::by_id(id, conn)?)?;
		let block = block.and_then(|block| Some(BlockObject::from(block)));
		Ok(block)
	}

	/// Returns a creation object based on the block type
	/// that is queried. Will have a template string to send
	/// back with the createBlock mutation
	async fn block_creation_display(
		&self,
		context: &Context<'_>,
		#[graphql(desc = "Name of the block type to create.")] r#type: String,
	) -> Result<String, Error> {
		let context = &context.data::<ContextData>()?;
		creation_display(context, r#type)
	}

	async fn block_types(&self) -> Vec<BlockType> {
		type_list()
	}

	async fn search_blocks(
		&self,
		context: &Context<'_>,
		query: String,
	) -> Result<Vec<Vec<BreadCrumb>>, Error> {
		let context = &context.data::<ContextData>()?.other();
		let conn = &context.pool.get()?;
		let user_id = optional_validate_token(optional_token(context))?;

		let mut helpers = blocks::dsl::blocks
			.load::<Block>(conn)?
			.into_iter()
			.filter(|block| can_view(user_id, block))
			.map(|block| {
				let crumbs = gen_breadcrumb(context, &block).unwrap_or(vec![]);
				let crumb_string = crumbs
					.iter()
					.map(|crumb| crumb.name.as_str())
					.collect::<Vec<&str>>()
					.join("/");
				let mut sim = normalized_levenshtein(&crumb_string, &query);
				if block.block_type == "data" {
					sim = sim / 2.;
				}
				BlockSortHelper {
					breadcrumb: crumbs,
					strsim: sim,
				}
			})
			.filter(|helper| helper.strsim != 0.)
			.collect::<Vec<BlockSortHelper>>();
		helpers.sort_by(|a, b| b.strsim.partial_cmp(&a.strsim).unwrap());
		let crumb_list: Vec<Vec<BreadCrumb>> = helpers
			.into_iter()
			.map(|helper| helper.breadcrumb)
			.collect();
		Ok(crumb_list)
	}
}

struct BlockSortHelper {
	breadcrumb: Vec<BreadCrumb>,
	strsim: f64,
}

pub fn creation_display(context: &ContextData, r#type: String) -> Result<String, Error> {
	let user_id = validate_token(require_token(&context.other())?)?;

	let display = delegate_creation_display(&context.other(), &r#type, user_id)?;
	Ok(serde_json::to_string(&display)?)
}
