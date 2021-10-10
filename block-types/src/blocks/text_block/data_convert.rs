use super::TextBlock;
use block_tools::display_api::component::{atomic::text::TextComponent, DisplayComponent};
use serde::{Deserialize, Serialize};
use serde_json::Value;

impl TextBlock {
	/// From the data string of a text block, get the rich text to render
	pub fn data_to_display(data: &str) -> Vec<DisplayComponent> {
		let doc: Value = serde_json::from_str(data).unwrap_or_default();
		if let Value::Object(doc) = doc {
			if let Some(Value::Array(vals)) = doc.get("content") {
				let mut components: Vec<DisplayComponent> = vec![];
				for val in vals {
					if let Value::Object(obj) = &val {
						if let Some(args) = obj.get("args") {
							let res = serde_json::from_value::<TextComponent>(args.clone());
							if let Ok(content) = res {
								components.push(content.into())
							}
						}
					}
				}
				components
			} else {
				vec![]
			}
		} else {
			vec![]
		}
	}
}

impl TextBlock {
	/// Turns some input rich text to render into string to set as data
	pub fn display_to_data(components: Vec<DisplayComponent>) -> String {
		let mut content: Vec<ComponentStruct> = vec![];
		for component in components {
			if let DisplayComponent::Text(c) = component {
				content.push(ComponentStruct {
					cid: "text".into(),
					args: c,
				})
			}
		}
		let data = DataFormat { content };
		if let Ok(data) = serde_json::to_string(&data) {
			data
		} else {
			r#"{"content":[]}"#.to_string()
		}
	}
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DataFormat {
	pub content: Vec<ComponentStruct>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ComponentStruct {
	pub cid: String,
	pub args: TextComponent,
}
