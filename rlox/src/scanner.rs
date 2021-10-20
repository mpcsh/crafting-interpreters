use std::iter::Peekable;
use std::vec::IntoIter;

use crate::token::{
	keyword, Token,
	TokenType::{self, *},
};

#[derive(Debug)]
pub struct ScanError {
	line: usize,
	message: String,
}

pub struct Scanner {
	source_text: String,
	source: Peekable<IntoIter<char>>,
	errors: Vec<ScanError>,
	start: usize,
	current: usize,
	line: usize,
}

impl Scanner {
	pub fn new(source_text: String) -> Self {
		Self {
			source_text: source_text.clone(),
			source: source_text
				.chars()
				.collect::<Vec<_>>()
				.into_iter()
				.peekable(),
			errors: vec![],
			start: 0,
			current: 0,
			line: 1,
		}
	}

	pub fn scan_tokens(&mut self) -> Vec<Token> {
		let mut tokens = vec![];
		loop {
			self.start = self.current;
			let token = self.scan_token();
			match token {
				Some(End) => return tokens,
				Some(token) => tokens.push(Token {
					token,
					lexeme: self.source_text[self.start..self.current].to_string(),
					line: self.line,
				}),
				None => (),
			}
		}
	}

	fn scan_token(&mut self) -> Option<TokenType> {
		match self.next() {
			// single-character tokens
			Some('(') => Some(LeftParen),
			Some(')') => Some(RightParen),
			Some('{') => Some(LeftBrace),
			Some('}') => Some(RightBrace),
			Some(',') => Some(Comma),
			Some('.') => Some(Dot),
			Some('-') => Some(Minus),
			Some('+') => Some(Plus),
			Some(';') => Some(Semicolon),
			Some('*') => Some(Star),

			// tokens that depend on one character of lookahead
			Some('!') => self.one_or_two('=', Bang, BangEqual),
			Some('=') => self.one_or_two('=', Equal, EqualEqual),
			Some('<') => self.one_or_two('=', Less, LessEqual),
			Some('>') => self.one_or_two('=', Greater, GreaterEqual),

			// string literals
			Some('"') => {
				let mut literal = vec![];
				literal.append(&mut self.take_while(|&c| c != '"'));
				match self.next() {
					Some('"') => (),
					Some(c) => self.emit_error(format!("expected close quote, found {}", c)),
					None => self.emit_error(format!(
						"unterminated string {}",
						literal.iter().collect::<String>()
					)),
				}
				Some(StringLiteral(literal.iter().collect()))
			}

			// numeric literals
			Some(first_digit @ '0'..='9') => {
				let mut literal = vec![first_digit];
				literal.append(&mut self.take_while(|&c| ('0'..='9').contains(&c) || c == '.'));
				match literal.iter().collect::<String>().parse() {
					Ok(value) => Some(NumberLiteral(value)),
					Err(reason) => {
						self.emit_error(format!("failed to parse float: {}", reason));
						None
					}
				}
			}

			// identifiers, keywords
			Some(c) if c.is_alphabetic() || c == '_' => {
				let mut chars = vec![c];
				chars.append(&mut self.take_while(|&c| c.is_alphanumeric() || c == '_'));
				let ident: String = chars.iter().collect();
				keyword(&ident).or(Some(Identifier(ident)))
			}

			// comments
			Some('/') => match self.peek() {
				Some('/') => {
					while self.peek() != Some('\n') {
						self.skip();
					}
					None
				}
				_ => Some(Slash),
			},

			// whitespace
			Some(' ' | '\r' | '\t') => None,
			Some('\n') => {
				self.line += 1;
				None
			}

			// unexpected characters
			Some(c) => {
				self.emit_error(format!("Unexpected character {}", c));
				None
			}

			// end of file
			None => Some(End),
		}
	}

	fn next(&mut self) -> Option<char> {
		self.current += 1;
		self.source.next()
	}

	fn peek(&mut self) -> Option<char> {
		self.source.peek().cloned()
	}

	fn skip(&mut self) {
		self.current += 1;
		self.source.next();
	}

	fn take_while<F>(&mut self, mut pred: F) -> Vec<char>
	where
		F: FnMut(&char) -> bool,
	{
		let mut lexeme = vec![];
		loop {
			match self.peek() {
				None => return lexeme,
				Some(c) => {
					if pred(&c) {
						self.skip();
						lexeme.push(c);
					} else {
						return lexeme;
					}
				}
			}
		}
	}

	fn expect(&mut self, expected: char, to_emit: Option<TokenType>) -> Option<TokenType> {
		match self.peek() {
			None => {
				self.emit_error(format!("Expected {}, found EOF", expected));
				None
			}
			Some(c) if c == expected => {
				self.skip();
				to_emit
			}
			Some(c) => {
				self.skip();
				self.emit_error(format!("Expected {}, found {}", expected, c));
				None
			}
		}
	}

	fn one_or_two(
		&mut self,
		second: char,
		absent: TokenType,
		present: TokenType,
	) -> Option<TokenType> {
		self.expect(second, Some(present)).or(Some(absent))
	}

	fn emit_error(&mut self, message: String) {
		self.errors.push(ScanError {
			line: self.line,
			message,
		})
	}
}
