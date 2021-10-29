use std::iter::Peekable;
use std::vec::IntoIter;

use crate::ast::{BinaryOp, Expr, UnaryOp};
use crate::token::{SpannedToken, Token};

#[derive(Debug)]
pub struct ParseError {
	line: usize,
	message: String,
}

pub struct Parser {
	tokens: Peekable<IntoIter<SpannedToken>>,
	errors: Vec<ParseError>,
	line: usize,
}

impl Parser {
	pub fn new(tokens: Vec<SpannedToken>) -> Self {
		Self {
			tokens: tokens.into_iter().peekable(),
			errors: vec![],
			line: 1,
		}
	}

	fn next(&mut self) -> Option<Token> {
		self.tokens.next().map(|span| {
			self.line = span.line;
			span.token
		})
	}

	fn peek(&mut self) -> Option<Token> {
		self.tokens.peek().map(|span| span.token.clone())
	}

	fn expect(&mut self, tokens: &Vec<Token>) -> Option<Token> {
		match self.peek() {
			Some(token) if tokens.contains(&token) => {
				self.next();
				Some(token)
			}
			Some(_) => None,
			None => {
				self.emit_error(format!("Unexpected EOF while searching for {:?}", tokens));
				None
			}
		}
	}

	fn emit_error(&mut self, message: String) {
		self.errors.push(ParseError {
			line: self.line,
			message,
		})
	}

	fn expression(&mut self) -> Expr {
		self.equality()
	}

	fn parse_binary_left<P, T>(
		&mut self,
		mut parse_operand: P,
		operators: &Vec<Token>,
		parse_operator: &T,
	) -> Expr
	where
		P: FnMut(&mut Self) -> Expr,
		T: Fn(Token) -> BinaryOp,
	{
		let mut left = parse_operand(self);
		while let Some(operator) = self.expect(operators).map(parse_operator) {
			let right = parse_operand(self);
			left = Expr::Binary(Box::new(left), operator, Box::new(right));
		}
		left
	}

	fn equality(&mut self) -> Expr {
		self.parse_binary_left(
			Self::comparison,
			&vec![Token::EqualEqual, Token::BangEqual],
			&|token| match token {
				Token::EqualEqual => BinaryOp::Equal,
				Token::BangEqual => BinaryOp::NotEqual,
				_ => unreachable!(),
			},
		)
	}

	fn comparison(&mut self) -> Expr {
		self.parse_binary_left(
			Self::term,
			&vec![
				Token::Greater,
				Token::GreaterEqual,
				Token::Less,
				Token::LessEqual,
			],
			&|token| match token {
				Token::Greater => BinaryOp::Greater,
				Token::GreaterEqual => BinaryOp::GreaterEqual,
				Token::Less => BinaryOp::Less,
				Token::LessEqual => BinaryOp::LessEqual,
				_ => unreachable!(),
			},
		)
	}

	fn term(&mut self) -> Expr {
		self.parse_binary_left(
			Self::factor,
			&vec![Token::Minus, Token::Plus],
			&|token| match token {
				Token::Minus => BinaryOp::Subtract,
				Token::Plus => BinaryOp::Add,
				_ => unreachable!(),
			},
		)
	}

	fn factor(&mut self) -> Expr {
		self.parse_binary_left(
			Self::unary,
			&vec![Token::Slash, Token::Star],
			&|token| match token {
				Token::Slash => BinaryOp::Divide,
				Token::Star => BinaryOp::Multiply,
				_ => unreachable!(),
			},
		)
	}

	fn unary(&mut self) -> Expr {
		let parse_operator = &|token| match token {
			Token::Bang => UnaryOp::Not,
			Token::Minus => UnaryOp::Negation,
			_ => unreachable!(),
		};
		if let Some(operator) = self
			.expect(&vec![Token::Bang, Token::Minus])
			.map(parse_operator)
		{
			let right = self.unary();
			return Expr::Unary(operator, Box::new(right));
		}

		self.primary()
	}

	fn primary(&mut self) -> Expr {
		match self.peek() {
			Some(Token::False) => Expr::BooleanLiteral(false),
			Some(Token::True) => Expr::BooleanLiteral(true),
			Some(Token::Nil) => Expr::NilLiteral,
			Some(Token::NumberLiteral(n)) => Expr::NumberLiteral(n),
			Some(Token::StringLiteral(s)) => Expr::StringLiteral(s),
			Some(Token::LeftParen) => {
				let expr = self.expression();
				self.expect(&vec![Token::RightParen]);
				Expr::Grouping(Box::new(expr))
			}
			Some(_) => unreachable!(),
			None => unimplemented!(),
		}
	}
}
