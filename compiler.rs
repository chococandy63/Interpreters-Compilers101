use std::env;
use std::fs;
use std::io::{self, Write};
use std::process;
use crate::ukiyoCompiler::Ukiyo;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => run_prompt(),
        2 => run_file(&args[1]),
        _ => {
            println!("Usage: ukiyoCompiler [script]");
            process::exit(64);
        }
    }
}

fn run_file(path: &String) {
    let contents = fs::read_to_string(path)
        .expect("Something went wrong reading the file");
    run(&contents);
    if had_error() {
        process::exit(65);
    }
}

fn run_prompt() {
    let stdin = io::stdin();
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut line = String::new();
        stdin.read_line(&mut line).unwrap();
        if line.is_empty() { break; }
        run(&line);
    }
}

fn run(source: &String) {
    let tokens = scan_tokens(source);
    for token in tokens {
        println!("{:?}", token);
    }
}

fn scan_tokens(source: &String) -> Vec<Token> {
    // TODO: Implement the Scanner logic here
    vec![]
}

fn had_error() -> bool {
    // TODO: Add your error handling logic here
    false
}

#[derive(Debug)]
struct Token {
    // TODO: Define the Token structure here
}

static mut HAD_ERROR: bool = false;

fn error(line: i32, message: &str) {
    report(line, "", message);
}

fn report(line: i32, location: &str, message: &str) {
    eprintln!("[line {}] Error{}: {}", line, location, message);
    unsafe {
        HAD_ERROR = true;
    }
}

