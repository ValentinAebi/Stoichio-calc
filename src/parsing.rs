use std::collections::BTreeMap;
use std::fmt::{Debug, Display, Formatter};

use TokenType::{Alphabetic, ClosingParenthesis, ClosingBracket, Arrow, NoType, Numeric, OpeningParenthesis, OpeningBracket, Whitespace};

use crate::chemistry::{Atom, Molecule, RawEquation};
use crate::parsing::TokenType::Plus;

const ARROW_PARTS: [char; 4] = ['=', '-', '<', '>'];

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum TokenType {
    Alphabetic,
    Numeric,
    OpeningParenthesis,
    ClosingParenthesis,
    OpeningBracket,
    ClosingBracket,
    Plus,
    Arrow,
    Whitespace,
    NoType,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Token(pub String, pub TokenType, pub u64);

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
        '(' => (OpeningParenthesis, true),
        ')' => (ClosingParenthesis, true),
        '[' => (OpeningBracket, true),
        ']' => (ClosingBracket, true),
        '+' => (Plus, true),
        _ if ARROW_PARTS.contains(c) => (Arrow, false),
        c if c.is_ascii_whitespace() => (Whitespace, false),
        _ => (NoType, false)
    }
}

pub fn tokenize(txt: &String) -> Vec<Token> {
    let mut acc_token_str = String::new();
    let mut acc_tok_type = NoType;
    let mut acc_tok_start: u64 = 0;
    let mut tokens: Vec<Token> = Vec::new();

    let mut pos: u64 = 0;
    for c in txt.chars() {
        let (c_tok_type, force_start) = token_type_for(&c);
        if c_tok_type == acc_tok_type && !force_start {
            acc_token_str.push(c);
        } else {
            if acc_token_str.len() != 0 {
                tokens.push(Token(acc_token_str.clone(), acc_tok_type, acc_tok_start));
            }
            acc_tok_start = pos;
            acc_token_str.clear();
            acc_token_str.push(c);
            acc_tok_type = c_tok_type;
        }
        pos += 1;
    }
    tokens.push(Token(acc_token_str.clone(), acc_tok_type, acc_tok_start));
    tokens
}

fn check_token_seq(tokens: &Vec<Token>) -> Result<(), String> {
    let mut parentheses: Vec<Token> = Vec::new();
    for tok in tokens {
        match tok {
            opening @ Token(_, OpeningParenthesis | OpeningBracket, _) =>
                parentheses.push(opening.clone()),
            Token(_, ClosingParenthesis, closing_pos) => {
                match parentheses.pop() {
                    Some(Token(_, OpeningParenthesis, _)) => {}
                    Some(Token(_, OpeningBracket, opening_pos)) =>
                        return Result::Err(format!("'[' at position {} closed by ')' at position {}", opening_pos, closing_pos)),
                    None => return Result::Err("')' closed but never opened".to_string()),
                    _ => panic!("should not happen")
                }
            }
            Token(_, ClosingBracket, closing_pos) => {
                match parentheses.pop() {
                    Some(Token(_, OpeningBracket, _)) => {}
                    Some(Token(_, OpeningParenthesis, opening_pos)) =>
                        return Result::Err(format!("'(' at position {} closed by ']' at position {}", opening_pos, closing_pos)),
                    None => return Result::Err("']' closed but never opened".to_string()),
                    _ => panic!("should not happen")
                }
            }
            Token(txt, NoType, pos) => return Result::Err(format!("unrecognized token: {} at position {}", txt, pos)),
            _ => {}
        }
    }
    match parentheses.pop() {
        Some(Token(_, OpeningBracket, pos)) =>
            Result::Err(format!("'[' at position {} never closed", pos)),
        Some(Token(_, OpeningParenthesis, pos)) =>
            Result::Err(format!("'(' at position {} never closed", pos)),
        Some(_) => panic!("should not happen"),
        None => Result::Ok(())
    }
}

fn merge_atom_into_seq(atoms_seq: &mut BTreeMap<Atom, u32>, atom: Atom, coef: u32) {
    let prev_coef = if let Some(&c) = atoms_seq.get(&atom) { c } else { 0 };
    atoms_seq.insert(atom.clone(), prev_coef + coef);
}

fn parse_atoms_seq(atoms: &BTreeMap<String, Atom>, tokens: &Vec<Token>) -> Result<BTreeMap<Atom, u32>, String> {
    let mut rem_tokens = tokens.clone();
    let mut atoms_seq: BTreeMap<Atom, u32> = BTreeMap::new();

    while !rem_tokens.is_empty() {
        let next_tok = rem_tokens.remove(0);
        match next_tok {
            Token(alpha, Alphabetic, pos) => {
                if let Some(atom) = atoms.get(&alpha) {
                    let curr_coef: u32 = if let Some(Token(num, Numeric, _)) = rem_tokens.get(0) {
                        let parsed_coef = num.parse().unwrap();
                        rem_tokens.remove(0);
                        parsed_coef
                    } else { 1 };
                    merge_atom_into_seq(&mut atoms_seq, atom.clone(), curr_coef);
                } else {
                    return Result::Err(format!("unknown element: {} at position {}", alpha, pos));
                }
            }
            Token(_, OpeningParenthesis | OpeningBracket, _) => {
                let mut sub_seq: Vec<Token> = Vec::new();
                let mut depth = 1;
                while depth > 0 && !rem_tokens.is_empty() {
                    let next_tok = rem_tokens.remove(0);
                    match next_tok {
                        Token(_, OpeningParenthesis | OpeningBracket, _) => depth += 1,
                        Token(_, ClosingParenthesis | ClosingBracket, _) => depth -= 1,
                        _ => {}
                    }
                    if depth > 0 {
                        sub_seq.push(next_tok.clone())
                    }
                }
                if depth == 0 {
                    let mul_factor = if let Some(Token(num, Numeric, _)) = rem_tokens.get(0) {
                        let parsed_factor: u32 = num.parse().unwrap();
                        rem_tokens.remove(0);
                        parsed_factor
                    } else { 1 };
                    match parse_atoms_seq(atoms, &sub_seq) {
                        Result::Ok(sub_molec_atoms) => {
                            for (at, coef) in sub_molec_atoms {
                                merge_atom_into_seq(&mut atoms_seq, at, mul_factor * coef);
                            }
                        }
                        err @ Result::Err(_) => return err
                    }
                } else {
                    return Result::Err(format!("unbalanced parentheses"));
                }
            }
            Token(_, _, pos) => return Result::Err(format!("unexpected: {} at position {}", next_tok, pos))
        }
    }
    Result::Ok(atoms_seq)
}

pub fn parse_molecule(atoms: &BTreeMap<String, Atom>, tokens: &Vec<Token>) -> Result<Molecule, String> {
    match check_token_seq(&tokens) {
        Result::Ok(()) => {
            parse_atoms_seq(atoms, tokens).map(|atoms_seq| {
                Molecule {
                    atoms: atoms_seq
                }
            })
        }
        Result::Err(msg) => Result::Err(msg)
    }
}

fn parse_raw_equation_member(atoms: &BTreeMap<String, Atom>, tokens: &Vec<Token>) -> Result<Vec<Molecule>, String> {
    let mut molecules: Vec<Molecule> = Vec::new();
    let mut acc_molec_tokens: Vec<Token> = Vec::new();
    for tok in tokens {
        match tok.1 {
            Whitespace => {}
            Plus => {
                let status_res = terminate_molecule(atoms, &mut molecules, &mut acc_molec_tokens);
                if let Err(msg) = status_res { return Err(msg); }
            }
            Arrow | NoType => panic!("should not happen"),
            _ => acc_molec_tokens.push(tok.clone())
        }
    }
    let status_res = terminate_molecule(atoms, &mut molecules, &mut acc_molec_tokens);
    if let Err(msg) = status_res { Err(msg) } else { Ok(molecules) }
}

fn terminate_molecule(atoms: &BTreeMap<String, Atom>, molecules: &mut Vec<Molecule>, acc_molec_tokens: &mut Vec<Token>) -> Result<(), String> {
    let parsed = parse_molecule(atoms, &acc_molec_tokens);
    acc_molec_tokens.clear();
    match parsed {
        Result::Ok(molecule) => {
            molecules.push(molecule);
            Result::Ok(())
        }
        Result::Err(msg) => Result::Err(msg)
    }
}

pub fn parse_raw_equation(atoms: &BTreeMap<String, Atom>, tokens: &Vec<Token>) -> Result<RawEquation, String> {
    let mut lhs_tokens: Vec<Token> = Vec::new();
    let mut rhs_tokens: Vec<Token> = Vec::new();
    let mut member_idx = 0;
    for tok in tokens {
        match tok.1 {
            Arrow => member_idx += 1,
            _ => {
                match member_idx {
                    0 => lhs_tokens.push(tok.clone()),
                    1 => rhs_tokens.push(tok.clone()),
                    _ => return Result::Err(format!("more than 2 members in equation"))
                }
            }
        }
    }
    if member_idx == 1 {
        let (lhs, rhs) = match (
            parse_raw_equation_member(atoms, &lhs_tokens),
            parse_raw_equation_member(atoms, &rhs_tokens)
        ) {
            (Ok(l), Result::Ok(r)) => (l, r),
            (Err(msg), _) => return Err(msg),
            (_, Err(msg)) => return Err(msg)
        };
        Result::Ok(RawEquation { lhs, rhs })
    } else {
        Result::Err(format!("an equation must have exactly 2 members"))
    }
}

