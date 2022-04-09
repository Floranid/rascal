#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Token<'a> {
    LParen,
    RParen,
    Indent,
    Dedent,
    Semi,

    Plus,
    Minus,
    Star,
    Slash,

    Eq,
    Gt,
    Lt,
    BangEq,

    Amp,
    Pipe,
    Bang,

    Int(i64),
    Float(f64),
    Bool(bool),
    Char(char),
    Ident(&'a str),

    If,
    Then,
    Else,
    Case,
    Of,

    Arrow,
}

pub fn keyword_from_str(s: &str) -> Option<Token<'_>> {
    let token = match s {
        "if" => Token::If,
        "then" => Token::Then,
        "else" => Token::Else,
        "case" => Token::Case,
        "of" => Token::Of,
        _ => return None,
    };

    Some(token)
}
