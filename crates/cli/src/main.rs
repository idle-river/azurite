use lexer::tokenize;
use std::fs;

fn main() {
    let mut args = std::env::args();

    args.next(); // eat executable

    if let Some(file_path) = args.next() {
        let contents = fs::read_to_string(&file_path).unwrap_or_else(|e| {
            eprintln!("Failed to read from file {}:\n{}", file_path, e);
            panic!()
        });

        match tokenize(&contents) {
            Ok(tokens) => println!("{tokens:#?}"),
            Err(err) => eprintln!("{err}"),
        }
    } else {
        repl();
    }
}

fn repl() {
    unimplemented!()
}
