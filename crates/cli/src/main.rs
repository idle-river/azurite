use lexer::tokenize;

fn main() {
    println!("{:#?}", tokenize("let x = 45;").unwrap());
}
