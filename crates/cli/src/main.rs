use lexer::tokenize;

fn main() {
    let args: Vec<_> = std::env::args().skip(1).collect();

    if args.len() < 1 {
        eprintln!("Usage: azurite <command> [args]");
        std::process::exit(1);
    }

    match args[0].as_ref() {
        "run" => {
            if args.len() < 2 {
                eprintln!("Usage: azurite run <source_file>");
                std::process::exit(1);
            }

            let source_file = &args[1];
            let source_code =
                std::fs::read_to_string(source_file).expect("Failed to read source file");

            let tokens = tokenize(&source_code);
            for token in tokens {
                println!("{token}");
            }
        }
        _ => {
            eprintln!("Unknown command: {}", args[0]);
            std::process::exit(1);
        }
    }
}
