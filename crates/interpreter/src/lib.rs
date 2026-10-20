pub struct Token;

pub fn tokenize(src: String) -> Vec<Token> {
    let chars = src.chars();
    for c in chars {
        println!("{c}");
    }

    unimplemented!()
}
