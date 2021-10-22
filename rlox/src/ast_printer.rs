use crate::ast::{Acceptor, Binary, BinaryOperator, Expr, Literal, Unary, UnaryOperator, Visitor};

impl ToString for UnaryOperator {
	fn to_string(&self) -> String {
		use UnaryOperator::*;
		match self {
			Negation => "-".to_string(),
			Bang => "!".to_string(),
		}
	}
}

impl ToString for BinaryOperator {
	fn to_string(&self) -> String {
		use BinaryOperator::*;
		match self {
			EqualEqual => "-".to_string(),
			BangEqual => "!=".to_string(),
			Less => "<".to_string(),
			LessEqual => "<=".to_string(),
			Greater => ">".to_string(),
			GreaterEqual => ">=".to_string(),
			Add => "+".to_string(),
			Subtract => "-".to_string(),
			Multiply => "*".to_string(),
			Divide => "/".to_string(),
		}
	}
}

pub struct AstPrinter;

impl AstPrinter {
	pub fn unparse(&mut self, expr: &Expr) -> String {
		expr.accept(self)
	}
}

impl Visitor<String> for AstPrinter {
	fn visit_literal(&mut self, value: &Literal) -> String {
		use Literal::*;
		match value {
			Identifier(id) => id.to_string(),
			String(s) => s.to_string(),
			Number(n) => n.to_string(),
			Boolean(b) => b.to_string(),
			Nil => "nil".to_string(),
		}
	}

	fn visit_unary(&mut self, expr: &Unary) -> String {
		format!(
			"({} {})",
			expr.operator.to_string(),
			expr.operand.accept(self)
		)
	}

	fn visit_binary(&mut self, expr: &Binary) -> String {
		format!(
			"({} {} {})",
			expr.operator.to_string(),
			expr.left.accept(self),
			expr.right.accept(self)
		)
	}

	fn visit_grouping(&mut self, expr: &Expr) -> String {
		format!("(group {})", expr.accept(self))
	}
}
