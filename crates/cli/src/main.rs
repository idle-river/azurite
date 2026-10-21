use std::{env, fs, io::ErrorKind, process};

use interpreter::tokenize;

fn main() {
    let mut args = env::args();

    let _ = args.next(); // Consume program name

    match args.next() {
        Some(file_name) => {
            let content = match fs::read_to_string(file_name) {
                Ok(file) => file,
                Err(error) if error.kind() == ErrorKind::NotFound => {
                    println!("Source file not found");
                    process::exit(1);
                }
                Err(e) => {
                    println!("An error occurred: {:?}", e);
                    process::exit(1);
                }
            };

            println!("{:?}", tokenize(&content));
        }

        None => repl(),
    }
}

fn repl() {
    todo!("repl has not been implemented yet");
}
