use crate::{
	graphql::ContextData,
	user_logic::user::{user_by_id, QLUser},
};
use async_graphql::*;
use block_tools::models::Block;
use block_types::delegation::display::{delegate_embed_display, delegate_page_display};
use chrono::{DateTime, Utc};
use std::time::SystemTime;

use super::breadcrumb::{BreadCrumb, gen_breadcrumb};

pub struct BlockObject {
	pub id: i64,
	pub block_data: Option<String>,
	pub created_at: SystemTime,
	pub updated_at: SystemTime,
	pub block_type: String,
	pub owner_id: i32,
}

#[Object]
impl BlockObject {
	async fn id(&self) -> i64 {
		self.id
	}

	async fn data(&self) -> Option<String> {
		self.block_data.clone()
	}

	async fn r#type(&self) -> String {
		self.block_type.clone()
	}

	async fn created_at(&self) -> DateTime<Utc> {
		self.created_at.into()
	}

	async fn updated_at(&self) -> DateTime<Utc> {
		self.updated_at.into()
	}

	async fn owner(&self, context: &Context<'_>) -> Result<QLUser, Error> {
		let context = &context.data::<ContextData>()?;
		let user = user_by_id(context, self.owner_id).await?;

		Ok(user.unwrap())
	}

	async fn page_display(&self, context: &Context<'_>) -> Result<String, Error> {
		let context = &context.data::<ContextData>()?;
		let display = delegate_page_display(&to_blockd(self), &context.other())?;
		Ok(serde_json::to_string(&display)?)
	}

	async fn embed_display(&self, context: &Context<'_>) -> Result<String, Error> {
		let context = &context.data::<ContextData>()?;
		let display = delegate_embed_display(&to_blockd(self), &context.other());
		Ok(serde_json::to_string(&display)?)
	}

	async fn breadcrumb(&self, context: &Context<'_>) -> Result<Vec<BreadCrumb>, Error> {
		let context = &context.data::<ContextData>()?;
		Ok(gen_breadcrumb(&context.other(), &to_blockd(self))?)
	}
}

impl From<Block> for BlockObject {
	fn from(blockd: Block) -> Self {
		BlockObject {
			id: blockd.id,
			created_at: blockd.created_at,
			updated_at: blockd.updated_at,
			block_data: blockd.block_data,
			block_type: blockd.block_type,
			owner_id: blockd.owner_id,
		}
	}
}

impl From<&Block> for BlockObject {
	fn from(blockd: &Block) -> Self {
		BlockObject {
			id: blockd.id,
			created_at: blockd.created_at,
			updated_at: blockd.updated_at,
			block_data: blockd.block_data.clone(),
			block_type: blockd.block_type.clone(),
			owner_id: blockd.owner_id,
		}
	}
}

pub fn to_blockd(block: &BlockObject) -> Block {
	Block {
		id: block.id,
		created_at: block.created_at,
		updated_at: block.updated_at,
		block_data: block.block_data.clone(),
		block_type: block.block_type.clone(),
		owner_id: block.owner_id,
	}
}
