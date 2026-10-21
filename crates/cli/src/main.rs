use std::{env, fs, io::ErrorKind, process};

use interpreter::evaluate;
use parser::Parser;
use scopes::Scope;

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
            let mut global_env = Scope::global();

            let ast = parser.produce_ast(&content).unwrap();
            let res = evaluate(ast, &mut global_env);

            println!("{:?}", res);
        }

        None => repl(),
    }
}

fn repl() {
    use std::io::{self, Write};
    let mut global_env = Scope::global();
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

        let ast = match parser.produce_ast(code) {
            Ok(ast) => ast,
            Err(e) => {
                use std::process;
                eprintln!("Parse error: {e}");
                process::exit(1);
            }
        };

        let res = evaluate(ast, &mut global_env);

        println!("Result: {:?}", res);
    }
}
