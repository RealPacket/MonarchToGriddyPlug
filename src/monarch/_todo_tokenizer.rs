// TODO(tokenizer): more
use serde_json::Map;

type Tokenizer = Map<String, Vec<MonarchLanguageRule>>;

type BasicMonarchLanguageRule = [String; 2];
type BasicMonarchLanguageRule2 = [String; 3];

enum MonarchLanguageRule {
	Basic(BasicMonarchLanguageRule),
	Basic2(BasicMonarchLanguageRule2),
	Expanded(BasicMonarchLanguageRule),
}

// #[derive(Deserialize)]
// #[serde(rename_all = "camelCase")]

// pub struct MonarchLanguageRule {

// }
