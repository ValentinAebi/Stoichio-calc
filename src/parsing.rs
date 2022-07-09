use std::collections::BTreeMap;
use std::fmt::{Debug, Display, format, Formatter};
use std::panic::panic_any;

use TokenType::{Alphabetic, ClosingParenthesis, Delimiter, NoType, Numeric, OpeningParenthesis, Whitespace};

use crate::chemistry::{Atom, Molecule};
use crate::parsing::TokenType::Plus;

const ARROW_PARTS: [char; 4] = ['=', '-', '<', '>'];

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
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

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Token(pub String, pub TokenType);

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

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

fn check_token_seq(tokens: &Vec<Token>) -> Result<(), String> {
    let mut depth = 0;
    for tok in tokens {
        match tok {
            Token(_, OpeningParenthesis) => depth += 1,
            Token(_, ClosingParenthesis) => depth -= 1,
            Token(txt, NoType) => return Result::Err(format!("unrecognized token: {txt}")),
            _ => {}
        }
        if depth < 0 {
            return Result::Err(format!("negative parentheses depth at {}", tok.0));
        }
    }
    if depth == 0 {
        Result::Ok(())
    } else {
        Result::Err(format!("unbalanced parentheses"))
    }
}

fn merge_atom_into_seq(atoms_seq: &mut BTreeMap<Atom, u32>, atom: Atom, coef: u32){
    let prev_coef = if let Some(&c) = atoms_seq.get(&atom) { c } else { 0 };
    atoms_seq.insert(atom.clone(), prev_coef + coef);
}

fn parse_atoms_seq(atoms: &BTreeMap<String, Atom>, tokens: &Vec<Token>) -> Result<BTreeMap<Atom, u32>, String> {

    let mut rem_tokens = tokens.clone();
    let mut atoms_seq: BTreeMap<Atom, u32> = BTreeMap::new();

    while !rem_tokens.is_empty() {
        let next_tok = rem_tokens.remove(0);
        match next_tok {
            Token(alpha, Alphabetic) => {
                if let Some(atom) = atoms.get(&alpha) {
                    let curr_coef: u32 = if let Some(Token(num, Numeric)) = rem_tokens.get(0) {
                        let parsed_coef = num.parse().unwrap();
                        rem_tokens.remove(0);
                        parsed_coef
                    } else { 1 };
                    merge_atom_into_seq(&mut atoms_seq, atom.clone(), curr_coef);
                } else {
                    return Result::Err(format!("unknown element: {alpha}"));
                }
            }
            Token(_, OpeningParenthesis) => {
                let mut sub_seq: Vec<Token> = Vec::new();
                let mut depth = 1;
                while depth > 0 && !rem_tokens.is_empty() {
                    let next_tok = rem_tokens.remove(0);
                    match next_tok {
                        Token(_, OpeningParenthesis) => depth += 1,
                        Token(_, ClosingParenthesis) => depth -= 1,
                        _ => {}
                    }
                    if depth > 0 {
                        sub_seq.push(next_tok.clone())
                    }
                }
                if depth == 0 {
                    let mul_factor = if let Some(Token(num, Numeric)) = rem_tokens.get(0) {
                        let parsed_factor: u32 = num.parse().unwrap();
                        rem_tokens.remove(0);
                        parsed_factor
                    } else { 1 };
                    match parse_atoms_seq(atoms, &sub_seq) {
                        Result::Ok(sub_molec_atoms) => {
                            for (at, coef) in sub_molec_atoms {
                                merge_atom_into_seq(&mut atoms_seq, at, mul_factor*coef);
                            }
                        }
                        err @ Result::Err(_) => return err
                    }
                } else {
                    return Result::Err(format!("unbalanced parentheses"));
                }
            }
            _ => return Result::Err(format!("unexpected: {next_tok}"))
        }
    }
    Result::Ok(atoms_seq)
}

pub fn parse_molecule(atoms: &BTreeMap<String, Atom>, tokens: Vec<Token>) -> Result<Molecule, String> {
    if let Result::Err(msg) = check_token_seq(&tokens) { panic_any(msg) };
    parse_atoms_seq(atoms, &tokens).map(|atoms_seq| {
        Molecule {
            atoms: atoms_seq
        }
    })
}

