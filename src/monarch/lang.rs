use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BracketPair {
	pub open: String,
	pub close: String,
	pub token: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Language {
	pub default_token: Option<String>,
	pub token_postfix: Option<String>,
	pub unicode: Option<bool>,
	pub ignore_case: Option<bool>,
	pub include_lf: Option<bool>,
	pub brackets: Vec<BracketPair>,
	//#region tokenizer properties
	// TODO: where's the types for these properties?
	pub keywords: Vec<String>,
	pub operators: Vec<String>,
	// symbols: Obj,
	// escapes: Obj,
	//#endregion
	// TODO(lang): implement tokenizer struct
	// tokenizer: Tokenizer,
}
