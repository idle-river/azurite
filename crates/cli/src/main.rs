use environment::Environment;
use interpreter::evaluate;
use parser::Parser;
use std::{env, fs, process};

fn main() {
    let args: Vec<_> = env::args().collect();
    match args.len() {
        1 => repl(),
        2 => {
            let file = &args[1];
            let contents = fs::read_to_string(file).expect("Failed to read from file");

            let mut parser = Parser::new();

            println!("{:#?}", parser.produce_ast(&contents));
        }
        _ => {
            println!("Usage: {} <filename>", args[0]);
            process::exit(0);
        }
    }
}

macro_rules! handle_repl_error {
    ($x: expr) => {
        if $x.is_err() {
            continue;
        }
    };
}

fn repl() {
    use environment::declare_var;
    use environment::values::RuntimeValue;
    use std::{io, io::Write};
    let mut parser = Parser::new();
    let mut env = Environment::new(None);

    // Hardcoded Variables
    declare_var!(env, "x", RuntimeValue::Number(100.0));
    declare_var!(env, "y", RuntimeValue::Number(150.0));
    declare_var!(env, "z", RuntimeValue::Number(200.0));

    println!("Azurite REPL v1.0");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut cmd = String::new();
        handle_repl_error!(io::stdin().read_line(&mut cmd));

        let src = cmd.trim();

        if src == "exit" {
            break;
        }

        let ast = parser.produce_ast(src);
        let result = evaluate(ast.clone(), &mut env);

        println!("Generated AST: {:#?}", ast);
        println!("Result: {:#?}", result);
    }
}
