use lexer::tokenize;

fn main() {
    let tokens = tokenize("let x = 45;");

    for token in tokens {
        println!("{token}")
    }
}
