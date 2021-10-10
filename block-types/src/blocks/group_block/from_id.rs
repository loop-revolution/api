use block_tools::{
	auth::permissions::can_view,
	dsl::prelude::*,
	models::{Block, Property},
	schema::properties,
	LoopError,
};

use super::GroupBlock;

impl Default for GroupBlock {
	fn default() -> Self {
		Self {
			name: None,
			description: None,
			items: vec![],
		}
	}
}

impl GroupBlock {
	pub fn from_id(
		block_id: i64,
		user_id: Option<i32>,
		conn: &PgConnection,
	) -> Result<Self, LoopError> {
		let property_list: Vec<Property> = properties::dsl::properties
			.filter(properties::dsl::parent_id.eq(block_id))
			.load::<Property>(conn)?;

		let mut props = GroupBlock::default();

		for property in property_list {
			match property.property_name.as_str() {
				"name" => {
					props.name = Block::by_id(property.value_id, conn)?
						.filter(|block| can_view(user_id, block));
				}
				"desc" => {
					props.description = Block::by_id(property.value_id, conn)?
						.filter(|block| can_view(user_id, block));
				}
				"item" => {
					let block = Block::by_id(property.value_id, conn)?
						.filter(|block| can_view(user_id, block));
					if let Some(block) = block {
						props.items.push((block, property.id));
					}
				}
				_ => {}
			}
		}

		Ok(props)
	}

	pub fn from_id_admin(block_id: i64, conn: &PgConnection) -> Result<Self, LoopError> {
		let property_list: Vec<Property> = properties::dsl::properties
			.filter(properties::dsl::parent_id.eq(block_id))
			.load::<Property>(conn)?;

		let mut props = Self::default();

		for property in property_list {
			match property.property_name.as_str() {
				"name" => {
					props.name = Block::by_id(property.value_id, conn)?;
				}
				"desc" => {
					props.description = Block::by_id(property.value_id, conn)?;
				}
				"item" => {
					let block = Block::by_id(property.value_id, conn)?;
					if let Some(block) = block {
						props.items.push((block, property.id));
					}
				}
				_ => {}
			}
		}

		Ok(props)
	}
}
