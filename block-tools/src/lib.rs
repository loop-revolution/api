pub mod db;
pub use db::*;
pub mod errors;
pub use errors::*;

#[macro_use]
extern crate diesel;

pub fn useless_fn() -> i32 {
	100
}
