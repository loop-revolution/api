pub mod db;
pub use db::*;
pub mod errors;
pub use errors::*;
pub mod blocks;
pub mod display_api;

#[macro_use]
extern crate diesel;

pub fn useless_fn() -> i32 {
	100
}
