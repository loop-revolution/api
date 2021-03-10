use serde::Serialize;
#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ColorScheme {
	Blue,
	Cyan,
	Gray,
	Orange,
	Pink,
	Purple,
	Red,
	Teal,
	Yellow,
}
