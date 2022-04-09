#![deny(rust_2018_idioms)]

use location::span::{Span, HasSpan};
use std::error::Error;
use std::{io::Write, path::Path};
use tokenizer::{error::TokenError, token::Token, Tokenizer};
mod location;
mod tokenizer;

fn main() {
    let arg = std::env::args().nth(1);

    match arg {
        Some(arg) => run_file(Path::new(&arg)),
        None => run_repl(),
    };
}

fn run_file(path: &Path) {
    let file_content = std::fs::read_to_string(path);
    let _src = match file_content {
        Err(e) => panic!("An error occured: {e}"),
        Ok(src) => src,
    };

    todo!();
}

fn run_repl() {
    let mut line_buf = String::new();

    loop {
        print!("rc >");
        std::io::stdout().flush().expect("Failed to flush Stdout.");

        std::io::stdin()
            .read_line(&mut line_buf)
            .expect("Failed to read Stdin.");

        if line_buf.trim() == "quit" {
            eprintln!("Exiting . . .");
            break;
        }

        let tokens = tokens_from_input(&line_buf);

        line_buf.clear();
    }
}

fn tokens_from_input(src: &str) -> Result<Vec<(Token<'_>, Span)>, Vec<TokenError>> {
    let tokenizer = Tokenizer::new(src);
    let mut has_error = false;
    let mut tokens = Vec::new();
    let mut errors = Vec::new();

    for token_result in tokenizer {
        match token_result {
            Ok(token_span) => {
                if !has_error {
                    tokens.push(token_span);
                }
            }

            Err(e) => {
                has_error = true;
                errors.push(e);
            }
        }
    }

    if has_error {
        Err(errors)
    } else {
        Ok(tokens)
    }
}

fn display_errors<E>(src: &str, errors: Vec<E>)
where
    E: Error + HasSpan,
{
}
