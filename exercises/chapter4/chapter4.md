## 1. The lexical grammars of Python and Haskell are not _regular_. What does that mean, and why aren't they?

Indentation-sensitive syntax requires keeping state - specifically, the expected indentation level of the current line (based on the previous line(s)).

## 2. Aside from separating tokens — distinguishing `print foo` from `printfoo` — spaces aren't used for much in most languages. However, in a couple of dark corners, a space does affect how code is parsed in CoffeeScript, Ruby, and the C preprocessor. Where and what effect does it have in each of those languages?

- CoffeeScript: isn't CoffeeScript just, proudly whitespace sensitive? As in, not in a "dark corner"?
- Ruby: https://stackoverflow.com/questions/8924628/ruby-expression-evaluation-whitespace-matters
- C preprocessor: https://blog.robertelder.org/7-weird-old-things-about-the-c-preprocessor/

## 3. Our scanner here, like most, discards comments and whitespace since those aren’t needed by the parser. Why might you want to write a scanner that does not discard those? What would it be useful for?

Literate programming! JSDoc-style types!

## 4. Add support to Lox’s scanner for C-style /_ ... _/ block comments. Make sure to handle newlines in them. Consider allowing them to nest. Is adding support for nesting more work than you expected? Why?

Ugh, no thank you.
