use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ColorScheme {
	Blue,
	Cyan,
	Gray,
	Green,
	Orange,
	Pink,
	Purple,
	Red,
	Teal,
	Yellow,
}
