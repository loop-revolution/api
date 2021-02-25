use crate::blocks::*;

pub enum BlockTypes {
	Data,
	Text,
	Group,
	FooBar,
	Invalid(String),
}

impl From<String> for BlockTypes {
	fn from(s: String) -> Self {
		match s.as_str() {
			data_block::BLOCK_NAME => BlockTypes::Data,
			text_block::BLOCK_NAME => BlockTypes::Text,
			group_block::BLOCK_NAME => BlockTypes::Group,
			foobar_block::BLOCK_NAME => BlockTypes::FooBar,
			_ => BlockTypes::Invalid(s),
		}
	}
}
