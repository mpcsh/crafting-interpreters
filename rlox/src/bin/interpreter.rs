use std::{
	env, fs,
	io::{self, Write},
	process,
};

extern crate rlox;

use rlox::frontend::ast_printer::AstPrinter;
use rlox::frontend::parser::Parser;
use rlox::frontend::scanner::Scanner;

pub struct Lox {
	had_error: bool,
}

impl Default for Lox {
	fn default() -> Self {
		Self::new()
	}
}

impl Lox {
	pub fn new() -> Self {
		Self { had_error: false }
	}

	pub fn run_file(&mut self, path: String) -> io::Result<()> {
		let chars = fs::read_to_string(path)?;
		self.run(chars);
		if self.had_error {
			process::exit(65);
		}
		Ok(())
	}

	pub fn run_prompt(&mut self) -> io::Result<()> {
		loop {
			print!("> ");
			io::stdout().flush()?;
			let mut chars = String::new();
			match io::stdin().read_line(&mut chars) {
				Err(e) => return Err(e),
				Ok(0) => return Ok(()),
				Ok(_) => {
					self.run(chars);
					self.had_error = false;
				}
			}
		}
	}

	fn run(&mut self, chars: String) {
		// scan
		let mut scanner = Scanner::new(chars);
		let tokens = scanner.scan_tokens();

		// parse
		let mut parser = Parser::new(tokens);
		if let Some(ast) = parser.parse() {
			let mut printer = AstPrinter::default();
			println!("{:?}", printer.unparse(&ast));
		}
	}

	// 	fn error(&mut self, line: usize, message: String) {
	// 		self.report(line, "".to_string(), message);
	// 	}

	// 	fn report(&mut self, line: usize, location: String, message: String) {
	// 		println!("[line {}] Error {}: {}", line, location, message);
	// 		self.had_error = true;
	// 	}
}

fn main() -> io::Result<()> {
	let args = env::args().skip(1).collect::<Vec<_>>();
	if args.len() > 1 {
		println!("Usage: rlox [script]");
		process::exit(64);
	}

	let mut lox = Lox::new();

	if args.len() == 1 {
		lox.run_file(args[0].to_string())
	} else {
		lox.run_prompt()
	}
}
