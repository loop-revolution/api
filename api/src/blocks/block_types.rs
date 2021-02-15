use async_graphql::*;
use block_tools::blocks::{BlockType as ToolsBlockType, TypeInfo};
use block_types::blocks::*;

#[derive(SimpleObject)]
/// A single block type. These dictate the logic on how to interpret the
/// block's properties and data.
pub struct BlockType {
	/// The block type's name, which is a unique identifier.
	pub name: String,
	/// The name of an icon that corresponds to the block type. These use the
	/// same name as the display API icon name.
	pub icon: String,
	/// A description of the block type, can be shown to users.
	pub desc: String,
}

impl From<TypeInfo> for BlockType {
	fn from(t: TypeInfo) -> Self {
		BlockType {
			name: t.name,
			desc: t.desc,
			icon: t.icon.to_string(),
		}
	}
}

pub fn type_list() -> Vec<BlockType> {
	vec![
		data_block::DataBlock::info().into(),
		text_block::TextBlock::info().into(),
		group_block::GroupBlock::info().into(),
	]
}
