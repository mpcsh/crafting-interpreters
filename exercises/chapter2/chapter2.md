## 1. Pick an open source implementation of a language you like. Download the source code and poke around in it. Try to find the code that implements the scanner and parser. Are they handwritten, or generated using tools like Lex and Yacc? (.l or .y files usually imply the latter.)

The rust [scanner](https://github.com/rust-lang/rust/tree/master/compiler/rustc_lexer/src) and [parser](https://github.com/rust-lang/rust/tree/master/compiler/rustc_parse/src/parser) are both very much hand-written!

[Spidermonkey's parser](https://searchfox.org/mozilla-central/source/js/src/frontend/Parser.cpp) is also a hand-rolled recursive descent parser, but I can't find its scanner.

## 2. Just-in-time compilation tends to be the fastest way to implement dynamically typed languages, but not all of them use it. What reasons are there to not JIT?

Start-up time, right? JITs take longer to start executing the program. If you care a lot about start-up time, you might choose not to use a JIT.

## 3. Most Lisp implementations that compile to C also contain an interpreter that lets them execute Lisp code on the fly as well. Why?

Pfft, no clue.
