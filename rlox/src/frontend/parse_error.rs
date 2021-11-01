use std::fmt;

use super::token::Token;

#[derive(Clone, Debug)]
pub enum ParseError {
	ExpectedToken(Token, Option<Token>),
	ExpectedOneOf(Vec<Token>, Option<Token>),
	InvalidExpressionStart(Token),
	UnexpectedEOF(Option<Token>),
}

impl fmt::Display for ParseError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		use ParseError::*;
		match self {
			ExpectedToken(expected, Some(actual)) => {
				write!(f, "expected {:?}, got {:?}", expected, actual)
			}
			ExpectedToken(expected, None) => {
				write!(f, "expected {:?}, got none", expected)
			}
			ExpectedOneOf(expecteds, Some(actual)) => {
				write!(f, "expected one of {:?}, got {:?}", expecteds, actual)
			}
			ExpectedOneOf(expecteds, None) => {
				write!(f, "expected one of {:?}, got none", expecteds)
			}
			InvalidExpressionStart(token) => {
				write!(f, "token {:?} cannot start an expression", token)
			}
			UnexpectedEOF(Some(expected)) => {
				write!(f, "unexpected EOF while searching for {:?}", expected)
			}
			UnexpectedEOF(None) => {
				write!(f, "unexpected EOF")
			}
		}
	}
}
