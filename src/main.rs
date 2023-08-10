mod error_handler;
mod scanner;
mod token;
mod token_kind;
mod utils;

use scanner::Scanner;
use std::env;
use std::fs::read_to_string;
use std::io::{self, stdout, BufRead, Result, Write};
use std::path::Path;
use token::Token;

struct Prox {
    had_error: bool,
}

impl Prox {
    fn error(&mut self, line: u64, message: &str) {
        self.report(line, message.to_string(), String::new());
    }

    fn new() -> Prox {
        Prox { had_error: false }
    }

    fn report(&mut self, line: u64, message: String, whre: String) {
        eprintln!("[line {line}] Error {whre}: {message}");
        self.had_error = true;
    }

    fn run(&self, source: &str) {
        let mut scanner: Scanner = Scanner::new(source.to_string());
        let tokens: Vec<Token> = scanner.scan_tokens();
        tokens
            .iter()
            .for_each(|token: &Token| println!("{}", token.to_string()));
    }

    fn run_file(&self, path: &Path) -> Result<()> {
        read_to_string(path)?
            .lines()
            .for_each(|line: &str| self.run(line));
        if self.had_error {
            std::process::exit(65)
        }
        Ok(())
    }

    fn run_prompt(&mut self) -> Result<()> {
        let mut stdin = io::stdin().lock();
        let mut input = String::new();
        let mut input_history: Vec<String> = Vec::new();

        loop {
            print!("> ");
            stdout().flush()?;
            stdin.read_line(&mut input)?;
            if input.is_empty() {
                break;
            }
            self.run(&input);
            self.had_error = false;
            input_history.push(input);
            input = String::new();
        }
        Ok(())
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let pt: &Path = Path::new(&args[0]);
    let mut app = Prox::new();
    if args.len() > 2 {
        println!("Usage: prox-lang [script]");
        std::process::exit(64);
    } else if args.len() == 2 {
        return app.run_file(pt);
    } else {
        return app.run_prompt();
    }
}
