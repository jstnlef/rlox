use std::env;

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
    println!("I'm running the script: {}", file_name);
}

fn run_prompt() {
    println!("I'm running the prompt!");
}
