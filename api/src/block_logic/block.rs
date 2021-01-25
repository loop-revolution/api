use super::delegation::{delegate_embed_display, delegate_page_display};
use crate::{
	graphql::{query::user_by_id, Context},
	user_logic::user::User,
	Error,
};
use block_tools::models::BlockD;
use chrono::{DateTime, Utc};
use juniper::graphql_object;
use std::time::SystemTime;

pub struct Block {
	pub id: i32,
	pub block_data: Option<String>,
	pub created_at: SystemTime,
	pub updated_at: SystemTime,
	pub block_type: String,
	pub owner_id: i32,
}

#[graphql_object(context = Context)]
impl Block {
	fn id(&self) -> i32 {
		self.id
	}

	fn data(&self) -> Option<String> {
		self.block_data.clone()
	}

	fn r#type(&self) -> String {
		self.block_type.clone()
	}

	fn created_at(&self) -> DateTime<Utc> {
		self.created_at.into()
	}

	fn updated_at(&self) -> DateTime<Utc> {
		self.updated_at.into()
	}

	async fn owner(&self, context: &Context) -> Result<User, Error> {
		let user = user_by_id(context, self.owner_id).await?;

		Ok(user.unwrap())
	}

	async fn page_display(&self, context: &Context) -> Result<String, Error> {
		let display = delegate_page_display(self, context).await?;
		Ok(serde_json::to_string(&display)?)
	}

	async fn embed_display(&self, context: &Context) -> Result<String, Error> {
		let display = delegate_embed_display(self, context).await?;
		Ok(serde_json::to_string(&display)?)
	}
}

impl From<BlockD> for Block {
	fn from(blockd: BlockD) -> Self {
		Block {
			id: blockd.id,
			created_at: blockd.created_at,
			updated_at: blockd.updated_at,
			block_data: blockd.block_data,
			block_type: blockd.block_type,
			owner_id: blockd.owner_id,
		}
	}
}

impl From<&BlockD> for Block {
	fn from(blockd: &BlockD) -> Self {
		Block {
			id: blockd.id,
			created_at: blockd.created_at,
			updated_at: blockd.updated_at,
			block_data: blockd.block_data.clone(),
			block_type: blockd.block_type.clone(),
			owner_id: blockd.owner_id,
		}
	}
}

pub fn to_blockd(block: &Block) -> BlockD {
	BlockD {
		id: block.id,
		created_at: block.created_at,
		updated_at: block.updated_at,
		block_data: block.block_data.clone(),
		block_type: block.block_type.clone(),
		owner_id: block.owner_id,
	}
}
