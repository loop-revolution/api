use block_tools::{
	blocks::{BlockType, Context, TypeInfo},
	display_api::{
		component::{
			button::ButtonComponent,
			card::{CardComponent, CardHeader},
			icon::Icon,
			search::{SearchComponent, SearchType},
			stack::StackComponent,
			text::TextComponent,
			DisplayComponent,
		},
		ActionObject, CreationObject, DisplayObject, MethodObject,
	},
	models::Block,
	Error,
};

pub const BLOCK_NAME: &str = "foobar";

pub struct FooBarBlock {}
impl BlockType for FooBarBlock {
	fn name() -> String {
		BLOCK_NAME.to_string()
	}

	fn info() -> TypeInfo {
		TypeInfo {
			name: Self::name(),
			icon: Icon::Feed,
			desc: "A block to use when implementing buttons & action objects & search.".to_string(),
		}
	}

	fn block_name(_block: &Block, _context: &Context) -> Result<String, Error> {
		Ok("foo bar".into())
	}

	fn page_display(_block: &Block, _context: &Context) -> Result<DisplayObject, Error> {
		Ok(DisplayObject::new(box TextComponent::new(
			"Page display not implemented",
		)))
	}

	fn embed_display(_block: &Block, _context: &Context) -> Box<dyn DisplayComponent> {
		let template: String = r#"{
			"foo": $[FOO]$,
			"bar": $[BAR]$
		}"#
		.into();
		let method = MethodObject {
			block_id: "9000".into(),
			block_type: Self::name(),
			method_name: "rand_method".into(),
			arg_template: template,
		};
		let choose_block = ActionObject::search(
			SearchComponent::default()
				.name("FOO")
				.r#type(SearchType::Block)
				.action_text("Choose a really cool block")
				.then(ActionObject::method(method.clone())),
		);
		let choose_user = ActionObject::search(
			SearchComponent::default()
				.name("BAR")
				.r#type(SearchType::User)
				.action_text("Choose your favorite person")
				.then(ActionObject::method(method.clone()))
				.cancel(ActionObject::search(
					SearchComponent::default()
						.name("BAR")
						.r#type(SearchType::User)
						.action_text("Ok, then choose somebody you like the least.")
						.then(ActionObject::method(method)),
				)),
		);
		let block_button = box ButtonComponent::new("Choose block").interact(choose_block);
		let user_button = box ButtonComponent::new("Choose user").interact(choose_user);
		let content = box StackComponent::default()
			.append(block_button)
			.append(user_button);
		let card = CardComponent {
			color: Some("#00ff00".into()),
			header: CardHeader::new("Foo Bar"),
			content,
		};
		box card
	}

	fn create_display(_context: &Context, _user_id: i32) -> Result<CreationObject, Error> {
		Err(Error::GenericError)
	}

	fn create(_input: String, _context: &Context, _user_id: i32) -> Result<Block, Error> {
		Err(Error::GenericError)
	}

	fn method_delegate(
		_context: &Context,
		_name: String,
		_block_id: i64,
		_args: String,
	) -> Result<Block, Error> {
		Err(Error::GenericError)
	}
}
