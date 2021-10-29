use super::ast::{Acceptor, BinaryOp, Expr, UnaryOp, Visitor};

impl ToString for UnaryOp {
	fn to_string(&self) -> String {
		use UnaryOp::*;
		match self {
			Negation => "-".to_string(),
			Not => "!".to_string(),
		}
	}
}

impl ToString for BinaryOp {
	fn to_string(&self) -> String {
		use BinaryOp::*;
		match self {
			Equal => "-".to_string(),
			NotEqual => "!=".to_string(),
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
	fn visit_identifier(&mut self, id: &String) -> String {
		id.clone()
	}
	fn visit_string(&mut self, s: &String) -> String {
		format!("\"{}\"", s)
	}
	fn visit_number(&mut self, n: &f64) -> String {
		n.to_string()
	}
	fn visit_boolean(&mut self, b: &bool) -> String {
		b.to_string()
	}
	fn visit_nil(&mut self) -> String {
		"nil".to_string()
	}
	fn visit_unary(&mut self, op: &UnaryOp, expr: &Expr) -> String {
		format!("({} {})", op.to_string(), expr.accept(self))
	}
	fn visit_binary(&mut self, left: &Expr, op: &BinaryOp, right: &Expr) -> String {
		format!(
			"({} {} {})",
			left.accept(self),
			op.to_string(),
			right.accept(self)
		)
	}
	fn visit_grouping(&mut self, expr: &Expr) -> String {
		format!("(group {})", expr.accept(self))
	}
}
