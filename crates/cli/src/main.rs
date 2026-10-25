use interpreter::evaluate;
use lexer::tokenize;
use parser::Parser;
use std::{fs, process};

fn main() {
    let mut args = std::env::args();

    args.next(); // eat executable

    if let Some(file_path) = args.next() {
        let mut parser = Parser::new();
        let contents = fs::read_to_string(&file_path).unwrap_or_else(|e| {
            eprintln!("Failed to read from file {}:\n{}", file_path, e);
            panic!()
        });

        generate(&contents, &mut parser);
    } else {
        repl();
    }
}

fn repl() {
    use std::io::{Write, stdin, stdout};
    let mut parser = Parser::new();

    println!("Azurite REPL v1.0");

    loop {
        print!("> ");
        let _ = stdout().flush();

        let mut cmd = String::new();
        stdin().read_line(&mut cmd).unwrap();

        generate(&cmd, &mut parser);
    }
}

fn generate(contents: &str, parser: &mut Parser) {
    let tokens = match tokenize(contents) {
        Ok(tokens) => tokens,
        Err(err) => {
            eprintln!("{err}");
            process::exit(1);
        }
    };

    let ast = match parser.parse(tokens) {
        Ok(ast) => ast,
        Err(err) => {
            eprintln!("{err}");
            process::exit(1);
        }
    };

    match evaluate(ast) {
        Ok(v) => println!("{v}"),
        Err(err) => {
            eprintln!("{err}");
            process::exit(1);
        }
    };
}
