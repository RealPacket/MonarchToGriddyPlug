use serde::Deserialize;
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommentRule {
	pub line_comment: String,
	pub block_comment: [String; 2],
}
type CharacterPair = Vec<[String; 2]>;
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AutoClosingPairConditional {
	pub open: String,
	pub close: String,
	pub not_in: Option<Vec<String>>,
}

// #[derive(Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct FoldingMarker {
// 	pub start: String, // regex
// 	pub end: String,   // regex
// }

// #[derive(Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct FoldingRule {
// 	pub off_side: Option<bool>,
// 	pub markers: Option<FoldingMarker>,
// }

type AutoClosingPairs = Vec<AutoClosingPairConditional>;

#[derive(Deserialize)]
pub struct OnEnterRule {}
#[derive(Deserialize)]
pub struct IndentationRule {}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LangConf {
	pub comments: CommentRule,
	pub brackets: CharacterPair,
	pub indentation_rules: Option<IndentationRule>,
	pub on_enter_rules: Option<OnEnterRule>,
	pub auto_closing_pairs: Option<AutoClosingPairs>,
	// TODO
	// pub folding: Option<FoldingRule>,
}
