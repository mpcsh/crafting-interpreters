## 1. Produce a grammar that matches the same language [elided, see book] but does not use any of that notational sugar.

Starting point:

```
expr -> expr ( "(" ( expr ("," expr)* )? ")" | "." IDENTIFIER )+
			| IDENTIFIER
			| NUMBER
```

Expanding the plus:

```
expr -> expr grouped
			| IDENTIFIER
			| NUMBER

grouped -> "(" ( expr ("," expr)* )? ")"
grouped -> "(" ( expr ("," expr)* )? ")" grouped
grouped -> "." IDENTIFIER
grouped -> "." IDENTIFIER grouped
```

Expanding the question mark and star:

```
expr -> expr grouped
			| IDENTIFIER
			| NUMBER

grouped -> "(" nested ")"
grouped -> "(" nested ")" grouped
grouped -> "(" expr ")"
grouped -> "(" expr ")" grouped
grouped -> "(" ")"
grouped -> "(" ")" grouped
grouped -> "." IDENTIFIER
grouped -> "." IDENTIFIER grouped

nested -> expr
nested -> expr, nested
```

### Bonus: What kind of expression does this bit of grammar encode?

Honestly? Not a fucking clue. I don't even know if I did the expansion right but I'm too tired to try harder.

## 2. The Visitor pattern lets you emulate the functional style in an object-oriented language. Devise a complementary pattern for a functional language. It should let you bundle all of the operations on one type together and let you define new types easily.

Owing again to tiredness here, I'm just going to count my [rlox implementation](/rlox/src/ast.rs) as good enough.

## 3. Define a visitor class for our syntax tree classes that takes an expression, converts it to RPN, and returns the resulting string.

```rust
impl Visitor<String> for AstPrinter {
	fn visit_literal(&mut self, value: &Number) -> String {
		n.to_string(),
	}

	fn visit_binary(&mut self, expr: &Binary) -> String {
		format!(
			"{} {} {}",
			expr.left.accept(self),
			expr.right.accept(self),
			expr.operator.to_string(),
		)
	}

	fn visit_grouping(&mut self, expr: &Expr) -> String {
		format!("{}", expr.accept(self))
	}
}
```
