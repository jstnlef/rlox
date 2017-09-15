use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::process;

use scanner::{Scanner, Token};

pub struct Lox {
    had_error: bool,
}

impl Lox {
    pub fn new() -> Self {
        Lox { had_error: false }
    }

    pub fn run_file(&mut self, file_name: &str) {
        let mut f = File::open(file_name).unwrap();
        let mut buffer = String::new();
        f.read_to_string(&mut buffer).unwrap();
        self.run(buffer);
        if self.had_error {
            process::exit(65);
        }
    }

    pub fn run_prompt(&mut self) {
        loop {
            print!("> ");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            self.run(input);
            self.had_error = false;
        }
    }

    fn run(&mut self, source: String) {
        let scanner = Scanner::new(source);
        let tokens: Vec<Token> = scanner.scan_tokens();
        for token in tokens {
            println!("{:?}", token);
        }
    }

    fn error(&mut self, line: i32, message: &str) {
        self.report(line, "", message);
    }

    fn report(&mut self, line: i32, error_location: &str, message: &str) {
        println!("[line {}] Error {}: {}", line, error_location, message);
        self.had_error = true;
    }
}
