use std::{env, fs, io, process};

struct Lox {
    hadError: bool,
}

impl Lox {
    fn runFile(&self, path: String) -> io::Result<()> {
        let chars = fs::read_to_string(path)?;
        self.run(chars);
        if self.hadError {
            process::exit(65);
        }
        Ok(())
    }

    fn runPrompt(&self) -> io::Result<()> {
        let mut chars = String::new();
        loop {
            print!("> ");
            match io::stdin().read_line(&mut chars) {
                Err(e) => return Err(e),
                Ok(0) => return Ok(()),
                Ok(_) => {
                    self.run(chars);
                    if self.hadError {
                        process::exit(65);
                    }
                }
            }
        }
    }

    fn run(&self, chars: String) {
        let scanner = Scanner::new(chars);
        let tokens = scanner.scanTokens();
        println!("{:?}", tokens);
    }

    fn error(&self, line: usize, message: String) {
        self.report(line, "".to_string(), message);
    }

    fn report(&self, line: usize, location: String, message: String) {
        println!("[line {}] Error {}: {}", line, location, message);
        self.hadError = true;
    }
}

fn main() {
    let args = env::args().skip(1).collect::<Vec<_>>();
    if args.len() > 1 {
        println!("Usage: rlox [script]");
        process::exit(64);
    } else if args.len() == 1 {
        runFile(args[0]);
    } else {
        runPrompt();
    }
}
