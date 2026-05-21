use std::fs;

fn main() {
    let file = fs::read_to_string("main.mpl").unwrap();
    println!("{}", file);
}

mod lexer;
