use crate::blocks::*;

pub enum BlockTypes {
	Data,
	Document,
	Text,
	Group,
	Invalid(String),
}

impl From<String> for BlockTypes {
	fn from(s: String) -> Self {
		match s.as_str() {
			data_block::BLOCK_NAME => BlockTypes::Data,
			text_block::BLOCK_NAME => BlockTypes::Text,
			group_block::BLOCK_NAME => BlockTypes::Group,
			document_block::BLOCK_NAME => BlockTypes::Document,
			_ => BlockTypes::Invalid(s),
		}
	}
}
