use super::{super::schema::blocks, NewProperty};
use crate::LoopError;
use colors_transform::Color;
use diesel::prelude::*;
use palette::{Shade, Srgb};
use rand::Rng;
use std::time::SystemTime;

#[derive(Queryable, Clone)]
pub struct Block {
	pub id: i64,
	pub block_type: String,
	pub created_at: SystemTime,
	pub updated_at: SystemTime,
	pub block_data: Option<String>,
	pub owner_id: i32,
	pub public: bool,
	pub perm_full: Vec<i32>,
	pub perm_edit: Vec<i32>,
	pub perm_view: Vec<i32>,
	pub stars: Vec<i32>,
	pub notif_enabled: Vec<i32>,
	pub color: Option<String>,
}

impl Block {
	pub fn make_property(&self, name: &str, block_id: i64) -> NewProperty {
		NewProperty {
			property_name: name.to_string(),
			parent_id: self.id,
			value_id: block_id,
			annotation: None,
		}
	}

	pub fn by_id(block_id: i64, conn: &PgConnection) -> Result<Option<Self>, LoopError> {
		Ok(blocks::dsl::blocks
			.filter(blocks::id.eq(block_id))
			.limit(1)
			.get_result(conn)
			.optional()?)
	}

	pub fn update_data(&self, new_data: &str, conn: &PgConnection) -> Result<Block, LoopError> {
		Ok(
			diesel::update(blocks::dsl::blocks.filter(blocks::id.eq(self.id)))
				.set((
					blocks::block_data.eq(Some(new_data)),
					blocks::updated_at.eq(std::time::SystemTime::now()),
				))
				.get_result(conn)?,
		)
	}

	pub fn update_color(
		&self,
		new_color: Option<String>,
		conn: &PgConnection,
	) -> Result<Block, LoopError> {
		Ok(
			diesel::update(blocks::dsl::blocks.filter(blocks::id.eq(self.id)))
				.set((
					blocks::color.eq(new_color),
					blocks::updated_at.eq(std::time::SystemTime::now()),
				))
				.get_result(conn)?,
		)
	}

	pub fn update_public(&self, public: bool, conn: &PgConnection) -> Result<Block, LoopError> {
		Ok(
			diesel::update(blocks::dsl::blocks.filter(blocks::id.eq(self.id)))
				.set((
					blocks::public.eq(public),
					blocks::updated_at.eq(std::time::SystemTime::now()),
				))
				.get_result(conn)?,
		)
	}

	pub fn update_starred(
		&self,
		starred: bool,
		user_id: i32,
		conn: &PgConnection,
	) -> Result<Block, LoopError> {
		let filter = blocks::dsl::blocks.filter(blocks::id.eq(self.id));
		let mut stars: Vec<i32> = filter.limit(1).select(blocks::stars).get_result(conn)?;
		if starred {
			stars.push(user_id);
		} else {
			stars = stars.into_iter().filter(|id| id != &user_id).collect();
		}
		Ok(diesel::update(filter)
			.set((
				blocks::stars.eq(stars),
				blocks::updated_at.eq(std::time::SystemTime::now()),
			))
			.get_result(conn)?)
	}

	pub fn update_notifs(
		&self,
		enable: bool,
		user_id: i32,
		conn: &PgConnection,
	) -> Result<Block, LoopError> {
		let filter = blocks::dsl::blocks.filter(blocks::id.eq(self.id));
		let mut notifs: Vec<i32> = filter
			.limit(1)
			.select(blocks::notif_enabled)
			.get_result(conn)?;
		if enable {
			notifs.push(user_id);
		} else {
			notifs = notifs.into_iter().filter(|id| id != &user_id).collect();
		}
		Ok(diesel::update(filter)
			.set((
				blocks::notif_enabled.eq(notifs),
				blocks::updated_at.eq(std::time::SystemTime::now()),
			))
			.get_result(conn)?)
	}

	pub fn update_perms(
		&self,
		perm_full: Vec<i32>,
		perm_edit: Vec<i32>,
		perm_view: Vec<i32>,
		conn: &PgConnection,
	) -> Result<Block, LoopError> {
		Ok(
			diesel::update(blocks::dsl::blocks.filter(blocks::id.eq(self.id)))
				.set((
					blocks::perm_full.eq(perm_full),
					blocks::perm_edit.eq(perm_edit),
					blocks::perm_view.eq(perm_view),
					blocks::updated_at.eq(std::time::SystemTime::now()),
				))
				.get_result(conn)?,
		)
	}
}

#[derive(Insertable)]
#[table_name = "blocks"]
pub struct NewBlock {
	pub block_type: String,
	pub created_at: SystemTime,
	pub updated_at: SystemTime,
	pub block_data: Option<String>,
	pub owner_id: i32,
	pub public: bool,
	pub perm_full: Vec<i32>,
	pub perm_edit: Vec<i32>,
	pub perm_view: Vec<i32>,
	pub stars: Vec<i32>,
	pub notif_enabled: Vec<i32>,
	pub color: Option<String>,
}

impl NewBlock {
	pub fn new(block_type: impl ToString, owner_id: i32) -> Self {
		NewBlock {
			block_type: block_type.to_string(),
			created_at: std::time::SystemTime::now(),
			updated_at: std::time::SystemTime::now(),
			block_data: None,
			owner_id,
			public: false,
			perm_full: vec![],
			perm_edit: vec![],
			perm_view: vec![],
			stars: vec![],
			notif_enabled: vec![],
			color: None,
		}
	}

	pub fn insert(self, conn: &PgConnection) -> Result<Block, LoopError> {
		Ok(diesel::insert_into(blocks::table)
			.values(self)
			.get_result(conn)?)
	}

	pub fn color_from(&mut self, rgb: String) -> Result<(), LoopError> {
		let mut rng = rand::thread_rng();
		let color = rgb.parse::<colors_transform::Rgb>().unwrap();
		let mut color =
			Srgb::new(color.get_red(), color.get_green(), color.get_blue()).into_linear();
		match rng.gen_range(0..4) {
			0 => color = color.lighten(0.1),
			1 => color = color.darken(0.1),
			2 => color = color.darken(0.2),
			_ => color = color.darken(0.2),
		}
		let color = colors_transform::Rgb::from(color.red, color.blue, color.green).to_css_string();
		self.color = Some(color);
		Ok(())
	}
}
