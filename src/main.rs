#[macro_use]
extern crate lazy_static;

mod lox;
mod scanner;
mod parser;
mod interpreter;

use std::env;
use lox::Lox;

fn main() {
    let args: Vec<String> = env::args().collect();
    let ref program_name = args[0];
    if args.len() > 2 {
        println!("Usage: {} [script]", program_name);
        return;
    }
    let mut lox = Lox::new();

    if args.len() == 2 {
        lox.run_file(&args[1]);
    } else {
        lox.run_prompt();
    }
}
