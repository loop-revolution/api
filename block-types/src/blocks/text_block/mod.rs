use block_tools::{
	blocks::{BlockType, Context, TypeInfo},
	display_api::{
		component::{atomic::icon::Icon, layout::card::CardComponent, DisplayComponent},
		CreationObject, DisplayObject,
	},
	models::Block,
	LoopError,
};

pub mod data_convert;
mod display;
mod methods;

pub const BLOCK_NAME: &str = "text";

pub struct TextBlock {}

impl BlockType for TextBlock {
	fn name() -> String {
		BLOCK_NAME.to_string()
	}

	fn info() -> TypeInfo {
		TypeInfo {
			name: Self::name(),
			icon: Icon::Type,
			desc: "Text blocks can have special formatting".to_string(),
		}
	}

	fn page_display(block: &Block, context: &Context) -> Result<DisplayObject, LoopError> {
		Self::handle_page_display(block, context)
	}

	fn embed_display(block: &Block, context: &Context) -> DisplayComponent {
		Self::handle_embed_display(block, context)
			.unwrap_or_else(|e| CardComponent::error_card(e).into())
	}

	fn create_display(_context: &Context, _user_id: i32) -> Result<CreationObject, LoopError> {
		Self::handle_create_display()
	}

	fn create(input: String, context: &Context, user_id: i32) -> Result<Block, LoopError> {
		Self::handle_create(input, context, user_id)
	}

	fn method_delegate(
		context: &Context,
		name: String,
		block_id: i64,
		args: String,
	) -> Result<Block, LoopError> {
		Self::handle_method_delegate(context, name, block_id, args)
	}

	fn block_name(_block: &Block, _context: &Context) -> Result<String, LoopError> {
		Ok("Text Block".into())
	}
}
