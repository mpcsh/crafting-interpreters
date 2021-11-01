use std::fmt::{self, Debug, Display};

#[derive(Clone, Debug)]
pub struct Spanned<T>
where
	T: Clone,
	T: Debug + Display,
{
	pub item: T,
	pub line: usize,
	pub column: usize,
}

impl<T> Display for Spanned<T>
where
	T: Clone,
	T: Debug + Display,
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "[{}:{}] {}", self.line, self.column, self.item)
	}
}
