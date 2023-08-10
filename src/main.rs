use std::env;
use std::fs::read_to_string;
use std::io::{self, stdout, BufRead, Result, Write};
use std::path::Path;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let pt: &Path = Path::new(&args[0]);
    if args.len() > 2 {
        println!("Usage: prox-lang [script]");
        std::process::exit(64);
    } else if args.len() == 2 {
        return run_file(pt);
    } else {
        return run_prompt();
    }
}

fn run(source: &str) {
    let scanner: Scanner = Scanner::new(source);
    let tokens: Vec<Token> = scanner.scan_tokens();
    tokens.iter().for_each(|token: Token| println!("{token}"));
}

fn run_file(path: &Path) -> Result<()> {
    read_to_string(path)?
        .lines()
        .for_each(|line: &str| run(line));
    Ok(())
}

fn run_prompt() -> Result<()> {
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
        run(&input);
        input_history.push(input);
        input = String::new();
    }
    Ok(())
}
