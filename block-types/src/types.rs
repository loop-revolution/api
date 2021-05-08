use crate::blocks::*;

pub enum BlockTypes {
	Data,
	Document,
	Group,
	Habit,
	Invalid(String),
	Task,
	Text,
}

impl From<String> for BlockTypes {
	fn from(s: String) -> Self {
		match s.as_str() {
			data_block::BLOCK_NAME => BlockTypes::Data,
			document_block::BLOCK_NAME => BlockTypes::Document,
			group_block::BLOCK_NAME => BlockTypes::Group,
			habit_block::BLOCK_NAME => BlockTypes::Habit,
			task_block::BLOCK_NAME => BlockTypes::Task,
			text_block::BLOCK_NAME => BlockTypes::Text,
			_ => BlockTypes::Invalid(s),
		}
	}
}
