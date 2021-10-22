pub enum Literal {
	Identifier(String),
	String(String),
	Number(f64),
	Boolean(bool),
	Nil,
}

pub enum UnaryOperator {
	Negation,
	Bang,
}

pub struct Unary {
	pub operator: UnaryOperator,
	pub operand: Box<Expr>,
}

pub enum BinaryOperator {
	EqualEqual,
	BangEqual,
	Less,
	LessEqual,
	Greater,
	GreaterEqual,
	Add,
	Subtract,
	Multiply,
	Divide,
}

pub struct Binary {
	pub operator: BinaryOperator,
	pub left: Box<Expr>,
	pub right: Box<Expr>,
}

pub struct Grouping {
	pub expr: Box<Expr>,
}

pub enum Expr {
	Literal(Literal),
	Unary(Unary),
	Binary(Binary),
	Grouping(Box<Expr>),
}

pub trait Acceptor<T> {
	fn accept(&self, visitor: &mut dyn Visitor<T>) -> T;
}

impl<T> Acceptor<T> for Expr {
	fn accept(&self, visitor: &mut dyn Visitor<T>) -> T {
		use Expr::*;
		match self {
			Literal(lit) => visitor.visit_literal(lit),
			Unary(expr) => visitor.visit_unary(expr),
			Binary(expr) => visitor.visit_binary(expr),
			Grouping(expr) => visitor.visit_grouping(expr),
		}
	}
}

pub trait Visitor<T> {
	fn visit_literal(&mut self, value: &Literal) -> T;
	fn visit_unary(&mut self, expr: &Unary) -> T;
	fn visit_binary(&mut self, expr: &Binary) -> T;
	fn visit_grouping(&mut self, expr: &Expr) -> T;
}
