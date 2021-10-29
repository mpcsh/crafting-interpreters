use std::fmt::Debug;

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
	// delimiters
	LeftParen,
	RightParen,
	LeftBrace,
	RightBrace,
	Comma,
	Dot,
	Semicolon,
	Slash,
	Star,

	// operators
	Bang,
	BangEqual,
	Equal,
	EqualEqual,
	Greater,
	GreaterEqual,
	Less,
	LessEqual,
	Minus,
	Plus,

	// keywords
	And,
	Class,
	Else,
	False,
	Fun,
	For,
	If,
	Nil,
	Or,
	Print,
	Return,
	Super,
	This,
	True,
	Var,
	While,

	// identifiers
	Identifier(String),

	// literals
	StringLiteral(String),
	NumberLiteral(f64),

	End,
}

pub fn keyword(identifier: &str) -> Option<Token> {
	use Token::*;
	match identifier {
		"and" => Some(And),
		"class" => Some(Class),
		"else" => Some(Else),
		"false" => Some(False),
		"for" => Some(For),
		"fun" => Some(Fun),
		"if" => Some(If),
		"nil" => Some(Nil),
		"or" => Some(Or),
		"print" => Some(Print),
		"return" => Some(Return),
		"super" => Some(Super),
		"this" => Some(This),
		"true" => Some(True),
		"var" => Some(Var),
		"while" => Some(While),
		_ => None,
	}
}

#[derive(Clone, Debug)]
pub struct SpannedToken {
	pub token: Token,
	pub lexeme: String,
	pub line: usize,
}
