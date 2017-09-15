use std::env;
use std::io;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    let args: Vec<String> = env::args().collect();
    let ref program_name = args[0];
    if args.len() > 2 {
        println!("Usage: {} [script]", program_name);
    } else if args.len() == 2 {
        run_file(&args[1]);
    } else {
        run_prompt();
    }
}

fn run_file(file_name: &str) {
    let mut f = File::open(file_name).unwrap();
    let mut buffer = String::new();
    f.read_to_string(&mut buffer).unwrap();
    run(buffer);
}

fn run_prompt() {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        run(input);
    }
}


fn run(buffer: String) {}
