#[derive(Debug)]
pub enum Expr {
	Identifier(String),
	StringLiteral(String),
	NumberLiteral(f64),
	BooleanLiteral(bool),
	NilLiteral,
	Unary(UnaryOp, Box<Expr>),
	Binary(Box<Expr>, BinaryOp, Box<Expr>),
	Grouping(Box<Expr>),
}

#[derive(Debug)]
pub enum UnaryOp {
	Negation,
	Not,
}

#[derive(Debug)]
pub enum BinaryOp {
	Equal,
	NotEqual,
	Less,
	LessEqual,
	Greater,
	GreaterEqual,
	Add,
	Subtract,
	Multiply,
	Divide,
}

pub trait Acceptor<T> {
	fn accept<V>(&self, visitor: &mut V) -> T
	where
		V: Visitor<T>;
}

impl<T> Acceptor<T> for Expr {
	fn accept<V>(&self, visitor: &mut V) -> T
	where
		V: Visitor<T>,
	{
		use Expr::*;
		match self {
			Identifier(id) => visitor.visit_identifier(id),
			StringLiteral(s) => visitor.visit_string(s),
			NumberLiteral(n) => visitor.visit_number(n),
			BooleanLiteral(b) => visitor.visit_boolean(b),
			NilLiteral => visitor.visit_nil(),
			Unary(op, expr) => visitor.visit_unary(op, expr),
			Binary(left, op, right) => visitor.visit_binary(left, op, right),
			Grouping(expr) => visitor.visit_grouping(expr),
		}
	}
}

pub trait Visitor<T> {
	fn visit_identifier(&mut self, id: &String) -> T;
	fn visit_string(&mut self, s: &String) -> T;
	fn visit_number(&mut self, n: &f64) -> T;
	fn visit_boolean(&mut self, b: &bool) -> T;
	fn visit_nil(&mut self) -> T;
	fn visit_unary(&mut self, op: &UnaryOp, expr: &Expr) -> T;
	fn visit_binary(&mut self, left: &Expr, op: &BinaryOp, right: &Expr) -> T;
	fn visit_grouping(&mut self, expr: &Expr) -> T;
}
