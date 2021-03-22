use serde::Serialize;
#[derive(Serialize, Clone)]
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
