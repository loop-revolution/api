use crate::{
	graphql::ContextData,
	user_logic::user::{user_by_id, QLUser},
};
use async_graphql::*;
use block_tools::{
	auth::{require_token, validate_token},
	models::Block,
};
use block_types::delegation::display::{delegate_embed_display, delegate_page_display};
use chrono::{DateTime, Utc};
use std::time::SystemTime;

use super::breadcrumb::{gen_breadcrumb, BreadCrumb};

pub struct BlockObject {
	pub id: i64,
	pub block_data: Option<String>,
	pub created_at: SystemTime,
	pub updated_at: SystemTime,
	pub block_type: String,
	pub owner_id: i32,
	pub public: bool,
	pub perm_full: Vec<i32>,
	pub perm_edit: Vec<i32>,
	pub perm_view: Vec<i32>,
	pub stars: Vec<i32>,
	pub notif_enabled: Vec<i32>,
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

	async fn public(&self) -> bool {
		self.public
	}

	async fn star_count(&self) -> usize {
		self.stars.len()
	}

	async fn starred(&self, context: &Context<'_>) -> Result<bool> {
		let context = &context.data::<ContextData>()?.other();
		let user_id = validate_token(&require_token(context)?)?;
		Ok(self.stars.contains(&user_id))
	}

	async fn notif_enabled(&self, context: &Context<'_>) -> Result<bool> {
		let context = &context.data::<ContextData>()?.other();
		let user_id = validate_token(&require_token(context)?)?;
		Ok(self.notif_enabled.contains(&user_id))
	}

	async fn owner(&self, context: &Context<'_>) -> Result<QLUser> {
		let context = &context.data::<ContextData>()?;
		let user = user_by_id(context, self.owner_id)?;

		Ok(user.unwrap())
	}

	async fn perm_full(&self, context: &Context<'_>) -> Result<Vec<QLUser>> {
		let context = &context.data::<ContextData>()?;
		let mut users: Vec<QLUser> = vec![];

		for id in self.perm_full.clone() {
			let user = user_by_id(context, id)?;
			if let Some(user) = user {
				users.push(user);
			}
		}

		Ok(users)
	}

	async fn perm_edit(&self, context: &Context<'_>) -> Result<Vec<QLUser>> {
		let context = &context.data::<ContextData>()?;
		let mut users: Vec<QLUser> = vec![];

		for id in self.perm_edit.clone() {
			let user = user_by_id(context, id)?;
			if let Some(user) = user {
				users.push(user);
			}
		}

		Ok(users)
	}

	async fn perm_view(&self, context: &Context<'_>) -> Result<Vec<QLUser>> {
		let context = &context.data::<ContextData>()?;
		let mut users: Vec<QLUser> = vec![];

		for id in self.perm_view.clone() {
			let user = user_by_id(context, id)?;
			if let Some(user) = user {
				users.push(user);
			}
		}

		Ok(users)
	}

	async fn page_display(&self, context: &Context<'_>) -> Result<String> {
		let context = &context.data::<ContextData>()?;
		let display = delegate_page_display(&to_blockd(self), &context.other())?;
		Ok(serde_json::to_string(&display)?)
	}

	async fn embed_display(&self, context: &Context<'_>) -> Result<String> {
		let context = &context.data::<ContextData>()?;
		let display = delegate_embed_display(&to_blockd(self), &context.other());
		Ok(serde_json::to_string(&display)?)
	}

	async fn breadcrumb(&self, context: &Context<'_>) -> Result<Vec<BreadCrumb>> {
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
			public: blockd.public,
			perm_full: blockd.perm_full,
			perm_edit: blockd.perm_edit,
			perm_view: blockd.perm_view,
			stars: blockd.stars,
			notif_enabled: blockd.notif_enabled,
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
			public: blockd.public,
			perm_full: blockd.perm_full.clone(),
			perm_edit: blockd.perm_edit.clone(),
			perm_view: blockd.perm_view.clone(),
			stars: blockd.stars.clone(),
			notif_enabled: blockd.notif_enabled.clone(),
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
		public: block.public,
		perm_full: block.perm_full.clone(),
		perm_edit: block.perm_edit.clone(),
		perm_view: block.perm_view.clone(),
		stars: block.stars.clone(),
		notif_enabled: block.notif_enabled.clone(),
	}
}
