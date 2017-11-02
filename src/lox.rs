use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::process;

use scanner::{Scanner, Token, TokenType};
use parser::parser::Parser;
use interpreter::{Interpreter, RuntimeError};

pub struct Lox {
    had_error: bool,
    had_runtime_error: bool,
}

impl Lox {
    pub fn new() -> Self {
        Lox {
            had_error: false,
            had_runtime_error: false,
        }
    }

    pub fn run_file(&mut self, file_name: &str) {
        let mut f = File::open(file_name).unwrap();
        let mut buffer = String::new();
        f.read_to_string(&mut buffer).unwrap();
        self.run(buffer);
        if self.had_error {
            process::exit(65);
        }
        if self.had_runtime_error {
            process::exit(70);
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
            self.had_runtime_error = false;
        }
    }

    fn run(&mut self, source: String) {
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse();
        let mut interpreter = Interpreter::new();

        match ast {
            Ok(tree) => {
                if let Err(err) = interpreter.interpret(&tree) {
                    self.runtime_error(err)
                }
            }
            Err(e) => self.token_error(e.token, &e.message),
        }
    }

    fn token_error(&mut self, token: Token, message: &str) {
        if token.token_type == TokenType::EOF {
            self.report(token.line, " at end", message);
        } else {
            self.report(token.line, &format!(" at '{}'", token.lexeme), message);
        }
    }

    fn runtime_error(&mut self, err: RuntimeError) {
        println!("{}\n[line {}]", err.message, err.token.line);
        self.had_runtime_error = true;
    }

    fn line_error(&mut self, line: i32, message: &str) {
        self.report(line, "", message);
    }

    fn report(&mut self, line: i32, error_location: &str, message: &str) {
        println!("[line {}] Error {}: {}", line, error_location, message);
        self.had_error = true;
    }
}
