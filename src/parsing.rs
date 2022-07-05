use std::collections::BTreeMap;
use std::iter::Map;
use TokenType::{Alphabetic, ClosingParenthesis, Delimiter, NoType, Numeric, OpeningParenthesis, Whitespace};
use crate::chemistry::{Atom, Molecule, RawEquation};
use crate::parsing::TokenType::Plus;

const ARROW_PARTS: [char; 4] = ['=', '-', '<', '>'];

#[derive(Eq, PartialEq, Debug)]
pub enum TokenType {
    Alphabetic,
    Numeric,
    OpeningParenthesis,
    ClosingParenthesis,
    Plus,
    Delimiter,
    Whitespace,
    NoType,
}

#[derive(Eq, PartialEq, Debug)]
pub struct Token(pub String, pub TokenType);

/// returns (token_type, force_start)
fn token_type_for(c: &char) -> (TokenType, bool) {
    match *c {
        c if c.is_alphabetic() => (Alphabetic, c.is_uppercase()),
        c if c.is_numeric() => (Numeric, false),
        c if c == '(' => (OpeningParenthesis, true),
        c if c == ')' => (ClosingParenthesis, true),
        c if c == '+' => (Plus, true),
        _ if ARROW_PARTS.contains(c) => (Delimiter, false),
        c if c.is_ascii_whitespace() => (Whitespace, false),
        _ => (NoType, false)
    }
}

pub fn tokenize(txt: &String) -> Vec<Token> {
    let mut acc_token_str = String::new();
    let mut acc_tok_type = NoType;
    let mut tokens: Vec<Token> = Vec::new();

    for c in txt.chars() {
        let (c_tok_type, force_start) = token_type_for(&c);
        if c_tok_type == acc_tok_type && !force_start {
            acc_token_str.push(c);
        } else {
            if acc_token_str.len() != 0 {
                tokens.push(Token(acc_token_str.clone(), acc_tok_type));
            }
            acc_token_str.clear();
            acc_token_str.push(c);
            acc_tok_type = c_tok_type;
        }
    }
    tokens.push(Token(acc_token_str.clone(), acc_tok_type));
    tokens
}

pub fn parse_atom(atoms: BTreeMap<String, Atom>, tokens: &mut Vec<Token>) -> Option<Atom> {
    if let Some(Token(txt, TokenType::Alphabetic)) = tokens.get(0) {
        if let Some(atom) = atoms.get(txt).cloned() {
            tokens.remove(0);
            Some(atom)
        } else {
            None
        }
    } else {
        None
    }
}

fn parse_raw_equation(atoms: BTreeMap<String, Atom>, tokens: Vec<Token>) -> Result<RawEquation, String> {
    todo!()
}
