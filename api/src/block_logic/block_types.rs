use async_graphql::*;
use block_tools::blocks::{BlockType as ToolsBlockType, TypeInfo};
use block_types::blocks::*;

#[derive(SimpleObject)]
pub struct BlockType {
	pub name: String,
	pub icon: String,
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
	]
}
