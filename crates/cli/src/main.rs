use std::{env, fs, io::ErrorKind, process};

use parser::Parser;

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

            let mut parser = Parser::new();

            println!("{:?}", parser.produce_ast(&content));
        }

        None => repl(),
    }
}

fn repl() {
    use std::io::{self, Write};

    let mut parser = Parser::new();
    println!("Azurite REPL v1.0");

    loop {
        print!(">>> ");
        io::stdout().flush().unwrap();

        let mut code = String::new();

        match io::stdin().read_line(&mut code) {
            Ok(0) => break, // EOF (e.g. Ctrl+D on macOS)
            Ok(_) => {}
            Err(e) => {
                eprintln!("Failed to read input: {e}");
                break;
            }
        }

        let code = code.trim();

        if code == "exit" {
            break;
        }

        if code.is_empty() {
            continue;
        }

        match parser.produce_ast(code) {
            Ok(ast) => println!("AST: {:?}", ast),
            Err(e) => eprintln!("Parse error: {e}"),
        }
    }
}
