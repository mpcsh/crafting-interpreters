use std::fmt::Debug;

#[derive(Debug)]
pub enum TokenType {
	// delimiters
	LeftParen,
	RightParen,
	LeftBrace,
	RightBrace,
	Comma,
	Dot,
	Minus,
	Plus,
	Semicolon,
	Slash,
	Star,

	// comparison operators
	Bang,
	BangEqual,
	Equal,
	EqualEqual,
	Greater,
	GreaterEqual,
	Less,
	LessEqual,

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

pub fn keyword(identifier: &str) -> Option<TokenType> {
	use TokenType::*;
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

#[derive(Debug)]
pub struct Token {
	pub token: TokenType,
	pub lexeme: String,
	pub line: usize,
}
