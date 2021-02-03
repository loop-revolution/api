use async_graphql::*;
use block_tools::blocks::{BlockType as ToolsBlockType, TypeInfo};

#[derive(SimpleObject)]
pub struct BlockType {
	pub name: String,
	pub icon: String,
	pub desc: String,
}

pub enum BlockTypes {
	Data,
	Text,
	Invalid(String),
}

impl From<String> for BlockTypes {
	fn from(s: String) -> Self {
		match s.as_str() {
			data_block::BLOCK_NAME => BlockTypes::Data,
			text_block::BLOCK_NAME => BlockTypes::Text,
			_ => BlockTypes::Invalid(s),
		}
	}
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
