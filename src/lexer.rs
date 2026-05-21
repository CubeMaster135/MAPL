pub enum Token {
    Ident(&str),
    Int(&str),
    Float(&str),
    Str(&str),
    Func,
    Class,
    Struct,
    While,
    For,
    If,
    Else,
    Var,
    Const
}

pub struct Lexer {
    input: String,
    position: usize,
    tokens: Vec<Token>,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Self {
            input,
            position: 0,
            tokens: Vec::new(),
        }
    }

}
