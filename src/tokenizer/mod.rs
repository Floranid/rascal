use std::str::Chars;

use crate::location::span::Span;

use self::{
    error::{TokenError, TokenErrorKind},
    token::{keyword_from_str, Token},
};

pub mod error;
pub mod token;

#[derive(Debug)]
pub struct Tokenizer<'a> {
    src: &'a str,
    chars: Chars<'a>,
    span: Span,
    current: Option<char>,
    next: Option<char>,
    insert_semicolon: bool,
    indent_lvls: Vec<usize>,
    pending_dedents: usize,
}

impl<'a> Tokenizer<'a> {
    pub fn new(src: &'a str) -> Self {
        let mut chars = src.chars();
        let current = chars.next();
        let next = chars.next();

        Self {
            src,
            chars,
            current,
            next,
            span: Span::new(0, 0),
            insert_semicolon: false,
            indent_lvls: Vec::new(),
            pending_dedents: 0,
        }
    }

    fn is_at_end(&self) -> bool {
        self.current.is_none()
    }

    fn slice(&self) -> &'a str {
        &self.src[self.span.as_range()]
    }

    fn current_indent_lvl(&self) -> usize {
        self.indent_lvls.last().copied().unwrap_or(0)
    }

    fn eat(&mut self) -> Option<char> {
        if let Some(c) = self.current {
            self.span.end += c.len_utf8();
            self.current = self.next;
            self.next = self.chars.next();
            Some(c)
        } else {
            None
        }
    }

    fn eat_while<F>(&mut self, f: F) -> usize
    where
        F: Fn(char) -> bool,
    {
        let mut count = 0;
        while !self.is_at_end() && f(self.current.unwrap()) {
            self.eat();
            count += 1;
        }

        count
    }

    fn tokenize_number(&mut self) -> Token<'a> {
        self.eat_while(|c| c.is_ascii_digit());
        if self.current == Some('.') && matches!(self.next, Some('0'..='9')) {
            self.eat();
            self.eat_while(|c| c.is_ascii_digit());
            Token::Float(self.slice().parse().unwrap())
        } else {
            Token::Int(self.slice().parse().unwrap())
        }
    }

    fn tokenize_char(&mut self) -> Result<Token<'a>, TokenErrorKind> {
        self.eat();
        let c = self.eat().ok_or(TokenErrorKind::UnexpectedEof)?;
        match self.eat() {
            Some('\'') => Ok(Token::Char(c)),
            _ => Err(TokenErrorKind::Expected('\'')),
        }
    }

    fn tokenize_alphanum(&mut self) -> Token<'a> {
        self.eat_while(|c| c.is_ascii_alphanumeric() || c == '_');
        let slice = self.slice();
        keyword_from_str(slice).unwrap_or(match slice {
            "true" => Token::Bool(true),
            "false" => Token::Bool(false),
            s => Token::Ident(s),
        })
    }

    fn next_token(&mut self) -> Option<Result<Token<'a>, TokenErrorKind>> {
        if self.insert_semicolon {
            self.insert_semicolon = false;
            return Some(Ok(Token::Semi));
        }

        if self.current == Some('\n') {
            self.eat();
            let new_indent_lvl = self.eat_while(|c| c == ' ');

            if new_indent_lvl > self.current_indent_lvl() {
                self.indent_lvls.push(new_indent_lvl);
                return Some(Ok(Token::Indent));
            } else if new_indent_lvl < self.current_indent_lvl() {
                let mut counter = 0;
                while new_indent_lvl < self.current_indent_lvl() {
                    self.indent_lvls.pop();
                    counter += 1;
                }

                self.pending_dedents += counter - 1;

                return if self.current_indent_lvl() == new_indent_lvl {
                    Some(Ok(Token::Dedent))
                } else {
                    Some(Err(TokenErrorKind::IllegalIndentLvl))
                };
            }
        }

        self.eat_while(|c| c.is_ascii_whitespace());
        self.span.start = self.span.end;

        let mut infer_semicolon = false;

        let token = match (self.current?, self.next) {
            ('(', _) => {
                self.eat();
                Ok(Token::LParen)
            }

            (')', _) => {
                self.eat();
                infer_semicolon = true;
                Ok(Token::RParen)
            }

            (';', _) => {
                self.eat();
                Ok(Token::Semi)
            }

            ('+', _) => {
                self.eat();
                Ok(Token::Plus)
            }

            ('-', Some('>')) => {
                self.eat();
                self.eat();
                Ok(Token::Arrow)
            }

            ('-', _) => {
                self.eat();
                Ok(Token::Minus)
            }

            ('*', _) => {
                self.eat();
                Ok(Token::Star)
            }

            ('/', _) => {
                self.eat();
                Ok(Token::Slash)
            }

            ('=', _) => {
                self.eat();
                Ok(Token::Eq)
            }

            ('>', _) => {
                self.eat();
                Ok(Token::Gt)
            }

            ('<', _) => {
                self.eat();
                Ok(Token::Lt)
            }

            ('!', Some('=')) => {
                self.eat();
                self.eat();
                Ok(Token::BangEq)
            }

            ('!', _) => {
                self.eat();
                Ok(Token::Bang)
            }

            ('&', _) => {
                self.eat();
                Ok(Token::Amp)
            }

            ('|', _) => {
                self.eat();
                Ok(Token::Pipe)
            }

            ('\'', _) => {
                infer_semicolon = true;
                self.tokenize_char()
            }

            ('0'..='9', _) => {
                infer_semicolon = true;
                Ok(self.tokenize_number())
            }

            ('_' | 'a'..='z' | 'A'..='Z', _) => {
                let token = self.tokenize_alphanum();
                infer_semicolon = matches!(token, Token::Ident(_) | Token::Bool(_));
                Ok(token)
            }

            (c, _) => {
                self.eat();
                Err(TokenErrorKind::Illegal(c))
            }
        };

        self.insert_semicolon = infer_semicolon
            && (self.current == Some('\n')
                || (self.current, self.next) == (Some('\r'), Some('\n')));

        Some(token)
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Result<(Token<'a>, Span), TokenError>;
    fn next(&mut self) -> Option<Self::Item> {
        let token = if self.pending_dedents > 0 {
            self.pending_dedents -= 1;
            Ok(Token::Dedent)
        } else {
            self.next_token()?
        };

        let with_span = match token {
            Ok(token) => Ok((token, self.span)),
            Err(e) => Err(TokenError::new(e, self.span)),
        };

        Some(with_span)
    }
}

#[cfg(test)]
mod tests {

    use crate::tokenizer::token::Token;

    use super::Tokenizer;

    fn raw_token_stream(src: &str) -> impl Iterator<Item = Token<'_>> {
        Tokenizer::new(src).map(|t| t.unwrap().0)
    }

    fn compare_raw_token_stream<'a, T1, T2>(lhs: T1, rhs: T2)
    where
        T1: IntoIterator<Item = Token<'a>>,
        T2: IntoIterator<Item = Token<'a>>,
    {
        assert!(lhs.into_iter().eq(rhs))
    }

    #[test]
    fn semicolon_insertion() {
        let src = "5\n\r\n()\n'a'\n";

        let expected = [
            Token::Int(5),
            Token::Semi,
            Token::LParen,
            Token::RParen,
            Token::Semi,
            Token::Char('a'),
            Token::Semi,
        ];

        compare_raw_token_stream(expected, raw_token_stream(src));
    }

    #[test]
    fn indentation() {
        let src = r"
case a of
 b -> case c of
       d -> e
 f -> g
";

        let expected = [
            Token::Case,
            Token::Ident("a"),
            Token::Of,
            Token::Indent,
            Token::Ident("b"),
            Token::Arrow,
            Token::Case,
            Token::Ident("c"),
            Token::Of,
            Token::Indent,
            Token::Ident("d"),
            Token::Arrow,
            Token::Ident("e"),
            Token::Semi,
            Token::Dedent,
            Token::Ident("f"),
            Token::Arrow,
            Token::Ident("g"),
            Token::Semi,
            Token::Dedent,
        ];

        compare_raw_token_stream(expected, raw_token_stream(src));
    }

    #[test]
    #[should_panic]

    fn illegal_indentation() {
        let src = r"
        case a of
         b -> case c of
               d -> e
            x -> y
         f -> g
        ";

        raw_token_stream(src).for_each(drop);
    }
}
