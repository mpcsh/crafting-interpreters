## 1. Write some sample Lox programs and run them (you can use the implementations of Lox in my repository). Try to come up with edge case behavior I didn't specify here. Does it do what you expect? Why or why not?

- Blocks can't be used in expression position!
- "Expression statements" seem to just throw away the expression value
- Classes are first class! You can return a class from a function and it'll work just fine!
- Inheritance cycles are impossible - the second class isn't yet in scope by the time the first one is read

## 2. This informal introduction leaves a _lot_ unspecified. List several open questions you have about the language’s syntax and semantics. What do you think the answers should be?

1. Do any of the conditional constructs introduce hoisting?
	- _Please god no!_
1. Can you import things from other files?
	- _Probably not - that's pretty complex._

## 3. Lox is a pretty tiny language. What features do you think it is missing that would make it annoying to use for real programs? (Aside from the standard library, of course.)

1. No ability to break code into multiple files
1. Lack of async / await
1. Minimalist compiler - not focused on error reporting / UX