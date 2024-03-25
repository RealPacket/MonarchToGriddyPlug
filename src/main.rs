mod monarch;

use std::env::args;

use full_moon::{
	ast::{
		punctuated::{Pair, Punctuated},
		span::ContainedSpan,
		Block, Call, Expression, FunctionArgs, FunctionCall, Prefix, Stmt, Suffix,
	},
	parse, print,
	tokenizer::{StringLiteralQuoteType, Token, TokenReference, TokenType},
	ShortString,
};
use monarch::{conf::LangConf, lang::Language};
// use full_moon;
use serde_json::from_str;
use std::fs::{read_to_string, write};

fn call(name: impl Into<ShortString>, args: Vec<Expression>) -> FunctionCall {
	let empty_vec = vec![];
	let parentheses = ContainedSpan::new(
		TokenReference::symbol("(").unwrap(),
		TokenReference::symbol(")\n").unwrap(),
	);
	let mut arguments = Punctuated::new();
	let mut idx = 0;
	for x in args.iter() {
		if idx < args.len() - 1 {
			arguments.push(Pair::Punctuated(
				x.clone(),
				TokenReference::symbol(", ").unwrap(),
			));
			idx += 1;
			continue;
		}
		arguments.push(Pair::End(x.clone()));
		idx += 1;
	}
	FunctionCall::new(Prefix::Name(TokenReference::new(
		empty_vec.clone(),
		Token::new(TokenType::Identifier {
			identifier: name.into(),
		}),
		empty_vec.clone(),
	)))
	.with_suffixes(vec![Suffix::Call(Call::AnonymousCall(
		FunctionArgs::Parentheses {
			parentheses: parentheses.clone(),
			arguments,
		},
	))])
}

fn block_append(blk: &Block, stmt: Stmt) -> Block {
	let mut stmts = vec![(stmt, None)];
	stmts.extend(blk.stmts().map(|s| (s.clone(), None)));
	return blk.clone().with_stmts(stmts);
}

fn main() {
	let mut args = args();
	args.next(); // ignore program path
	let lang_path = args.next().expect("No lang.json path provided.");
	let conf_path = args.next().expect("No conf.json path provided.");
	let l_str = read_to_string(lang_path).expect("Failed to read lang file");
	let lang = from_str::<Language>(&l_str).expect("Failed to parse lang file");
	let c_str = read_to_string(conf_path).expect("Failed to read conf file");
	let conf = from_str::<LangConf>(&c_str).expect("Failed to parse conf file");
	let mut block = Block::new();
	let empty_vec = vec![];
	let line_comment = conf.comments.line_comment;
	let line_comment_call = call(
		"highlight_region",
		vec![
			// region start
			Expression::String(TokenReference::new(
				empty_vec.clone(),
				Token::new(TokenType::StringLiteral {
					literal: line_comment.into(),
					multi_line: None,
					quote_type: StringLiteralQuoteType::Double,
				}),
				empty_vec.clone(),
			)),
			// region end
			Expression::String(TokenReference::new(
				empty_vec.clone(),
				Token::new(TokenType::StringLiteral {
					literal: "".into(),
					multi_line: None,
					quote_type: StringLiteralQuoteType::Double,
				}),
				empty_vec.clone(),
			)),
			// region type / "color"
			Expression::String(TokenReference::new(
				empty_vec.clone(),
				Token::new(TokenType::StringLiteral {
					literal: "comments".into(),
					multi_line: None,
					quote_type: StringLiteralQuoteType::Double,
				}),
				empty_vec.clone(),
			)),
			// line only?
			Expression::Symbol(TokenReference::symbol("true").unwrap()),
		],
	);
	block = block_append(&block, Stmt::FunctionCall(line_comment_call));
	let block_comment_start = conf
		.comments
		.block_comment
		.first()
		.expect("No block comment start");
	let block_comment_end = conf
		.comments
		.block_comment
		.last()
		.expect("No block comment end");
	let comment_call = call(
		"highlight_region",
		vec![
			// region start
			Expression::String(TokenReference::new(
				empty_vec.clone(),
				Token::new(TokenType::StringLiteral {
					literal: block_comment_start.into(),
					multi_line: None,
					quote_type: StringLiteralQuoteType::Double,
				}),
				empty_vec.clone(),
			)),
			// region end
			Expression::String(TokenReference::new(
				empty_vec.clone(),
				Token::new(TokenType::StringLiteral {
					literal: block_comment_end.into(),
					multi_line: None,
					quote_type: StringLiteralQuoteType::Double,
				}),
				empty_vec.clone(),
			)),
			// token type / "color"
			Expression::String(TokenReference::new(
				empty_vec.clone(),
				Token::new(TokenType::StringLiteral {
					literal: "comments".into(),
					multi_line: None,
					quote_type: StringLiteralQuoteType::Double,
				}),
				empty_vec.clone(),
			)),
			// line only?
			Expression::Symbol(TokenReference::symbol("false").unwrap()),
		],
	);
	block = block_append(&block, Stmt::FunctionCall(comment_call));
	for keyword in lang.keywords.iter() {
		let call = call(
			"highlight",
			vec![
				Expression::String(TokenReference::new(
					empty_vec.clone(),
					Token::new(TokenType::StringLiteral {
						literal: keyword.into(),
						multi_line: None,
						quote_type: StringLiteralQuoteType::Double,
					}),
					empty_vec.clone(),
				)),
				Expression::String(TokenReference::new(
					empty_vec.clone(),
					Token::new(TokenType::StringLiteral {
						literal: "reserved".into(),
						multi_line: None,
						quote_type: StringLiteralQuoteType::Double,
					}),
					empty_vec.clone(),
				)),
			],
		);
		block = block_append(&block, Stmt::FunctionCall(call));
	}
	// can't get an AST directly any other way, so...
	let a = parse("-- This code has been @generated with MonarchToGriddyPlug")
		.expect("Failed to parse a comment (sounds like a skill issue)");
	write("out.lua", print(&a.with_nodes(block))).expect("Failed to write AST.");
}
