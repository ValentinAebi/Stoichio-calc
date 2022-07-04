use std::collections::hash_set::Union;
use std::collections::HashSet;
use std::ops::Add;
use std::ptr::null;
use crate::Atom;
use TokenType::{Alphabetic, ClosingParenthesis, Delimiter, NoType, Numeric, OpeningParenthesis, Whitespace};

const DELIMITER_CHARS: [char; 3] = ['=', '<', '>'];

#[derive(Eq, PartialEq)]
enum TokenType {
    Alphabetic,
    Numeric,
    OpeningParenthesis,
    ClosingParenthesis,
    Delimiter,
    Whitespace,
    NoType
}

pub struct Token(String, TokenType);

fn token_type_for(c: &char) -> TokenType {
    match *c {
        c if c.is_alphabetic() => Alphabetic,
        c if c.is_numeric() => Numeric,
        c if c == '(' => OpeningParenthesis,
        c if c == ')' => ClosingParenthesis,
        _ if DELIMITER_CHARS.contains(c) => Delimiter,
        c if c.is_ascii_whitespace() => Whitespace,
        _ => NoType
    }
}

pub fn tokenize(txt: String) -> Vec<Token> {

    let formatted = txt.to_lowercase();

    let mut acc_token_str = String::new();
    let mut acc_tok_type = NoType;
    let mut tokens: Vec<Token> = Vec::new();

    for c in formatted.chars() {
        let c_tok_type = token_type_for(&c);
        if c_tok_type == acc_tok_type {
            acc_token_str.push(c);
        }
        else {
            tokens.push(Token(acc_token_str.clone(), acc_tok_type));
            acc_tok_type = c_tok_type;
            acc_token_str.clear();
            acc_token_str.push(c);
        }
    }
    tokens.push(Token(acc_token_str.clone(), acc_tok_type));
    tokens
}


