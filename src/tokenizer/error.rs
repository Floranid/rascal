use std::{fmt::Display};
use std::error::Error;
use crate::location::span::{HasSpan, Span};

#[derive(Debug)]
pub enum TokenErrorKind {
    IllegalIndentLvl,
    Expected(char),
    Illegal(char),
    UnexpectedEof,
}

#[derive(Debug)]
pub struct TokenError {
    pub kind: TokenErrorKind,
    pub span: Span,
}

impl HasSpan for TokenError {
    fn span(&self) -> Span {
        self.span
    }
}

impl TokenError {
    pub fn new(kind: TokenErrorKind, span: Span) -> Self {
        Self { kind, span }
    }
}

impl Display for TokenErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IllegalIndentLvl => f.write_str("Illegal level of indentation found."),
            Self::Expected(c) => write!(f, "Expected char `{c}`. (U+{})", *c as u32),
            Self::Illegal(c) => write!(f, "Illegal char `{c}`. (U+{})", *c as u32),
            Self::UnexpectedEof => f.write_str("Found EOF where a Token was expected.")
        }
    }
}

impl Display for TokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.kind)
    }
}

impl Error for TokenError {}