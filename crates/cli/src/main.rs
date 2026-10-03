use std::{env, fs, process};

fn main() {
    let args: Vec<_> = env::args().collect();
    match args.len() {
        1 => repl(),
        2 => {
            let file = &args[1];
            let contents = fs::read_to_string(file).expect("Failed to read from file");

            println!("{:#?}", lexer::tokenize(&contents));
        }
        _ => {
            println!("Usage: {} <filename>", args[0]);
            process::exit(0);
        }
    }
}

fn repl() {
    unimplemented!("REPL has not been implemented yet.");
}
