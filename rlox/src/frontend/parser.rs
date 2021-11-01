use std::iter::Peekable;
use std::vec::IntoIter;

use super::ast::{BinaryOp, Expr, UnaryOp};
use super::parse_error::ParseError::{self, *};
use super::spanned::Spanned;
use super::token::{SpannedToken, Token};

type ParseResult<T> = Result<T, Spanned<ParseError>>;

pub struct Parser {
	tokens: Peekable<IntoIter<SpannedToken>>,
	errors: Vec<Spanned<ParseError>>,
	line: usize,
	column: usize,
}

impl Parser {
	pub fn new(tokens: Vec<SpannedToken>) -> Self {
		Self {
			tokens: tokens.into_iter().peekable(),
			errors: vec![],
			line: 1,
			column: 1,
		}
	}

	// ===========================================================================
	// entry point
	// ===========================================================================

	pub fn parse(&mut self) -> Option<Expr> {
		match self.expression() {
			Ok(expr) => {
				dbg!(&self.tokens);
				dbg!(expr);
			}
			Err(err) => {
				dbg!(&self.tokens);
				dbg!(err);
			}
		}
		None
		// self.expression().ok()
	}

	// ===========================================================================
	// error handling
	// ===========================================================================

	fn spanned_error<T>(&mut self, error: ParseError) -> ParseResult<T> {
		let err = Spanned::<ParseError> {
			item: error,
			line: self.line,
			column: self.column,
		};
		self.errors.push(err.clone());
		Err(err)
	}

	fn synchronize(&mut self) {
		use Token::*;
		while let Some(head) = self.peek() {
			match head {
				Semicolon => {
					self.next();
					return;
				}
				Class | For | Fun | If | Print | Return | Var | While => return,
				_ => {
					self.next();
				}
			}
		}
	}

	// ===========================================================================
	// iterator helpers
	// ===========================================================================

	fn next(&mut self) -> Option<Token> {
		self.tokens.next().map(|span| {
			self.line = span.line;
			span.token
		})
	}

	fn peek(&mut self) -> Option<Token> {
		self.tokens.peek().map(|span| span.token.clone())
	}

	fn eat(&mut self, expected: Token) -> ParseResult<()> {
		match self.next() {
			Some(received) if received == expected => Ok(()),
			Some(received) => self.spanned_error(ExpectedToken(expected, Some(received))),
			None => self.spanned_error(UnexpectedEOF(Some(expected))),
		}
	}

	fn try_eat_one_of(&mut self, tokens: &[Token]) -> ParseResult<Option<Token>> {
		let head = self.peek();
		for expected in tokens {
			match head {
				Some(received) if received == *expected => {
					self.next();
					return Ok(Some(received));
				}
				_ => {
					continue;
				}
			}
		}

		// self.spanned_error(ExpectedOneOf(tokens.to_vec(), head))
		Ok(None)
	}

	// ===========================================================================
	// parser helpers
	// ===========================================================================

	fn parse_binary_left<P>(&mut self, mut parse_operand: P, operators: &[Token]) -> ParseResult<Expr>
	where
		P: FnMut(&mut Self) -> ParseResult<Expr>,
	{
		fn parse_operator(token: Token) -> BinaryOp {
			match token {
				Token::EqualEqual => BinaryOp::Equal,
				Token::BangEqual => BinaryOp::NotEqual,
				Token::Greater => BinaryOp::Greater,
				Token::GreaterEqual => BinaryOp::GreaterEqual,
				Token::Less => BinaryOp::Less,
				Token::LessEqual => BinaryOp::LessEqual,
				Token::Minus => BinaryOp::Subtract,
				Token::Plus => BinaryOp::Add,
				Token::Slash => BinaryOp::Divide,
				Token::Star => BinaryOp::Multiply,
				_ => unreachable!(),
			}
		}
		let mut left = parse_operand(self)?;
		while let Some(operator) = self.try_eat_one_of(operators)?.map(&parse_operator) {
			let right = parse_operand(self)?;
			left = Expr::Binary(Box::new(left), operator, Box::new(right));
		}
		Ok(left)
	}

	// ===========================================================================
	// expression parsers
	// ===========================================================================

	fn expression(&mut self) -> ParseResult<Expr> {
		self.equality()
	}

	fn equality(&mut self) -> ParseResult<Expr> {
		self.parse_binary_left(Self::comparison, &[Token::EqualEqual, Token::BangEqual])
	}

	fn comparison(&mut self) -> ParseResult<Expr> {
		self.parse_binary_left(
			Self::term,
			&[
				Token::Greater,
				Token::GreaterEqual,
				Token::Less,
				Token::LessEqual,
			],
		)
	}

	fn term(&mut self) -> ParseResult<Expr> {
		self.parse_binary_left(Self::factor, &[Token::Minus, Token::Plus])
	}

	fn factor(&mut self) -> ParseResult<Expr> {
		self.parse_binary_left(Self::unary, &[Token::Slash, Token::Star])
	}

	fn unary(&mut self) -> ParseResult<Expr> {
		let parse_operator = &|token| match token {
			Token::Bang => UnaryOp::Not,
			Token::Minus => UnaryOp::Negation,
			_ => unreachable!(),
		};
		if let Some(operator) = self
			.try_eat_one_of(&[Token::Bang, Token::Minus])?
			.map(parse_operator)
		{
			let right = self.unary()?;
			return Ok(Expr::Unary(operator, Box::new(right)));
		}

		self.primary()
	}

	fn primary(&mut self) -> ParseResult<Expr> {
		match self.next() {
			Some(Token::False) => Ok(Expr::BooleanLiteral(false)),
			Some(Token::True) => Ok(Expr::BooleanLiteral(true)),
			Some(Token::Nil) => Ok(Expr::NilLiteral),
			Some(Token::NumberLiteral(n)) => Ok(Expr::NumberLiteral(n)),
			Some(Token::StringLiteral(s)) => Ok(Expr::StringLiteral(s)),
			Some(Token::LeftParen) => {
				let expr = self.expression()?;
				self.eat(Token::RightParen)?;
				Ok(Expr::Grouping(Box::new(expr)))
			}
			Some(head) => self.spanned_error(InvalidExpressionStart(head)),
			None => self.spanned_error(UnexpectedEOF(None)),
		}
	}
}
