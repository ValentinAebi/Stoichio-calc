use std::collections::btree_map::BTreeMap;
use std::fmt::{Debug, Display, Formatter};

use TokenType::{Alphabetic, ClosingParenthesis, ClosingBracket, Arrow, NoType, Numeric, OpeningParenthesis, OpeningBracket, Whitespace};

use crate::chemistry::{Atom, chem_unit_for, ChemQuantity, Molecule, PeriodicTable, QuantifiedEquation, RawEquation};
use crate::parsing::TokenType::{Dot, Exponent, Minus, Plus};
use crate::return_on_error;

const ARROW_PARTS: [char; 3] = ['=', '<', '>'];

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum TokenType {
    Alphabetic,
    Numeric,
    Dot,
    OpeningParenthesis,
    ClosingParenthesis,
    OpeningBracket,
    ClosingBracket,
    Plus,
    Minus,
    Exponent,
    Arrow,
    Whitespace,
    NoType,
}

#[derive(Eq, PartialEq, Debug, Clone)]
/// Token(string, type, position)
pub struct Token(pub String, pub TokenType, pub u64);

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
/// PositionedError(error message, optional position)
pub struct PositionedError(pub String, pub Option<u64>);

impl Display for PositionedError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(pos) = self.1 {
            write!(f, "{}: {}", pos, self.0)
        } else {
            write!(f, "?: {}", self.0)
        }
    }
}

/// Returns (token_type, force_start)
fn token_type_for(c: &char) -> (TokenType, bool) {
    match *c {
        c if c.is_alphabetic() => (Alphabetic, c.is_uppercase()),
        c if c.is_numeric() => (Numeric, false),
        '.' => (Dot, true),
        '(' => (OpeningParenthesis, true),
        ')' => (ClosingParenthesis, true),
        '[' => (OpeningBracket, true),
        ']' => (ClosingBracket, true),
        '+' => (Plus, true),
        '-' => (Minus, true),
        '^' => (Exponent, true),
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

/// Check parentheses and makes sure that no `NoType` token is in the `tokens` vector
///
/// Returns `Err(message)` if a violation is found, `Ok(())` o.w.
fn check_token_seq(tokens: &Vec<Token>) -> Result<(), PositionedError> {
    let mut parentheses: Vec<Token> = Vec::new();
    for tok in tokens {
        match tok {
            opening @ Token(_, OpeningParenthesis | OpeningBracket, _) =>
                parentheses.push(opening.clone()),
            Token(_, ClosingParenthesis, closing_pos) => {
                match parentheses.pop() {
                    Some(Token(_, OpeningParenthesis, _)) => {}
                    Some(Token(_, OpeningBracket, opening_pos)) =>
                        return Err(PositionedError(
                            format!("'[' at position {} closed by ')' at position {}", opening_pos, closing_pos),
                            Some(*closing_pos),
                        )),
                    None => return Err(PositionedError(
                        "')' closed but never opened".to_string(),
                        Some(*closing_pos),
                    )),
                    _ => panic!("should not happen")
                }
            }
            Token(_, ClosingBracket, closing_pos) => {
                match parentheses.pop() {
                    Some(Token(_, OpeningBracket, _)) => {}
                    Some(Token(_, OpeningParenthesis, opening_pos)) =>
                        return Err(PositionedError(
                            format!("'(' at position {} closed by ']' at position {}", opening_pos, closing_pos),
                            Some(*closing_pos),
                        )),
                    None => return Err(PositionedError(
                        "']' closed but never opened".to_string(),
                        Some(*closing_pos),
                    )),
                    _ => panic!("should not happen")
                }
            }
            Token(txt, NoType, pos) => return Err(PositionedError(
                format!("unrecognized token: {} at position {}", txt, pos),
                Some(*pos),
            )),
            _ => {}
        }
    }
    match parentheses.pop() {
        Some(Token(_, OpeningBracket, pos)) =>
            Err(PositionedError(
                format!("'[' at position {} never closed", pos),
                Some(pos),
            )),
        Some(Token(_, OpeningParenthesis, pos)) =>
            Err(PositionedError(
                format!("'(' at position {} never closed", pos),
                Some(pos),
            )),
        Some(_) => panic!("should not happen"),
        None => Ok(())
    }
}

/// If `atoms_seq` already contains a coefficient for `atom`, then increment it by `coef`
///
/// Else, add an entry `atom` -> `coef`
fn merge_atom_into_seq(atoms_seq: &mut BTreeMap<Atom, u32>, atom: Atom, coef: u32) {
    let prev_coef = if let Some(&c) = atoms_seq.get(&atom) { c } else { 0 };
    atoms_seq.insert(atom.clone(), prev_coef + coef);
}

/// Parses an atom sequence expected to correspond to a molecule (or a part of it)
///
/// Returns a map atom -> coef and a charge (in a pair), or a `PositionedError` if an error occurs
fn parse_atoms_seq(atoms: &PeriodicTable, tokens: &Vec<Token>) -> Result<(BTreeMap<Atom, u32>, i32), PositionedError> {
    let mut rem_tokens = tokens.clone();
    let mut atoms_seq: BTreeMap<Atom, u32> = BTreeMap::new();
    let mut charge = 0;

    while !rem_tokens.is_empty() {
        let next_tok = rem_tokens.remove(0);
        match next_tok {

            // alphabetic token -> atom, possibly followed by a coefficient
            Token(alpha, Alphabetic, pos) => {
                if let Some(atom) = atoms.get(&alpha) {
                    let curr_coef: u32 = if let Some(Token(num, Numeric, _)) = rem_tokens.get(0) {
                        let parsed_coef = num.parse().unwrap();
                        rem_tokens.remove(0);
                        parsed_coef
                    } else { 1 };
                    merge_atom_into_seq(&mut atoms_seq, atom.clone(), curr_coef);
                } else {
                    return Err(PositionedError(
                        format!("unknown element: {} at position {}", alpha, pos),
                        Some(pos),
                    ));
                }
            }

            // parenthesis token, need to parse the sequence between the parentheses recursively
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
                        Ok((sub_molec_atoms, sub_charge)) => {
                            for (at, coef) in sub_molec_atoms {
                                merge_atom_into_seq(&mut atoms_seq, at, mul_factor * coef);
                            }
                            charge += sub_charge * (mul_factor as i32);
                        }
                        err @ Err(_) => return err
                    }
                } else {
                    return Err(PositionedError(
                        format!("unbalanced parentheses"),
                        None,
                    ));
                }
            }

            // exponent token (for charge); expect '+' or '-' and possibly a number
            Token(_, Exponent, pos_exp) => {
                match (rem_tokens.get(0), rem_tokens.get(1)) {
                    (
                        Some(Token(_, sign @ (Plus | Minus), _)),
                        Some(Token(num_str, Numeric, _))
                    ) | (
                        Some(Token(num_str, Numeric, _)),
                        Some(Token(_, sign @ (Plus | Minus), _))
                    ) => {
                        let num: i32 = num_str.parse().unwrap();
                        charge += if *sign == Plus { num } else { -num };
                        rem_tokens.remove(0);
                        rem_tokens.remove(0);
                    }
                    (
                        Some(Token(_, sign @ (Plus | Minus), _)),
                        _
                    ) => {
                        if *sign == Plus { charge += 1 } else { charge -= 1 };
                        rem_tokens.remove(0);
                    }
                    _ => return Err(PositionedError(
                        format!("charge format error, expected '^<charge><+/->', e.g. '^3+', or '^<+/-><charge>', e.g. '^+3'"),
                        Some(pos_exp),
                    ))
                }
            }

            // unexpected token: format error
            Token(_, _, pos) => return Err(PositionedError(
                format!("unexpected: '{}' at position {}", next_tok, pos),
                Some(pos),
            ))
        }
    }
    Ok((atoms_seq, charge))
}

/// `periodic_table` - all possible atoms in the molecule
pub fn parse_molecule(atoms: &PeriodicTable, tokens: &Vec<Token>) -> Result<Molecule, PositionedError> {
    match check_token_seq(&tokens) {
        Ok(()) => {
            parse_atoms_seq(atoms, tokens).map(|(atoms_seq, charge)| {
                let str_repr: String = tokens.iter().map(|tok| { format!("{}", tok.0) }).collect();
                Molecule {
                    atoms: atoms_seq,
                    charge,
                    string_repr: Some(str_repr),
                }
            })
        }
        Err(err) => Err(err)
    }
}

fn parse_equation_member(periodic_table: &PeriodicTable, tokens: &Vec<Token>) -> Result<Vec<(Molecule, Option<ChemQuantity>)>, PositionedError> {
    let mut molecules: Vec<(Molecule, Option<ChemQuantity>)> = Vec::new();
    let mut acc_tokens: Vec<Token> = Vec::new();

    // to distinguish between '+' between molecules and '+' for the charge
    let mut expect_charge_plus_or_minus = false;

    for tok in tokens {
        match tok.1 {
            Whitespace => {}
            Exponent => {
                expect_charge_plus_or_minus = true;
                acc_tokens.push(tok.clone());
            }
            Plus => {
                if expect_charge_plus_or_minus {
                    expect_charge_plus_or_minus = false;
                    acc_tokens.push(tok.clone());
                } else {
                    let status_res = terminate_molecule(periodic_table, &mut molecules, &mut acc_tokens);
                    if let Err(err) = status_res { return Err(err); }
                }
            }
            Minus => {
                if expect_charge_plus_or_minus { expect_charge_plus_or_minus = false }
                acc_tokens.push(tok.clone());
            }
            Arrow | NoType => return Err(PositionedError(format!("unexpected: {}", tok.0), Some(tok.2))),
            _ => acc_tokens.push(tok.clone())
        }
    }
    let status_res = terminate_molecule(periodic_table, &mut molecules, &mut acc_tokens);
    if let Err(err) = status_res { Err(err) } else { Ok(molecules) }
}

/// Tries to parse the molecule described by the tokens in `acc_molec_tokens`, adds it to `molecules` and clears `acc_molec_tokens`
fn terminate_molecule(periodic_table: &PeriodicTable, molecules: &mut Vec<(Molecule, Option<ChemQuantity>)>, acc_tokens: &mut Vec<Token>) -> Result<(), PositionedError> {
    let (chem_quant_opt, num_remove) =
        match &acc_tokens[..] {
            [
            Token(digits_str, Numeric, _),
            Token(_, Dot, _),
            Token(decimal_str, Numeric, _),
            Token(unit_str, Alphabetic, unit_tok_pos),
            ..
            ] => (Some(return_on_error!(chem_quantity_for(unit_str, unit_tok_pos, &format!("{}.{}", digits_str, decimal_str)))), 4),
            [
            Token(digits_str, Numeric, _),
            Token(unit_str, Alphabetic, unit_tok_pos),
            ..
            ] => (Some(return_on_error!(chem_quantity_for(unit_str, unit_tok_pos, &digits_str))), 2),
            _ => (None, 0)
        };
    for _ in 0..num_remove {
        acc_tokens.remove(0);
    }
    let parsed = parse_molecule(periodic_table, &acc_tokens);
    acc_tokens.clear();
    match parsed {
        Ok(molecule) => {
            molecules.push((molecule, chem_quant_opt));
            Ok(())
        }
        Err(err) => Err(err)
    }
}

fn chem_quantity_for(unit_str: &String, unit_tok_pos: &u64, value_str: &String) -> Result<ChemQuantity, PositionedError> {
    let value: f64 = value_str.parse().unwrap();
    let unit = match chem_unit_for(unit_str) {
        Ok(u) => u,
        Err(_) => return Err(PositionedError(format!("unknown unit: {}", unit_str), Some(*unit_tok_pos)))
    };
    Ok(ChemQuantity(value, unit))
}

/// `periodic_table` - all possible atoms in the molecule
pub fn parse_quantified_equation(periodic_table: &PeriodicTable, tokens: &Vec<Token>) -> Result<QuantifiedEquation, PositionedError> {
    let mut lhs_tokens: Vec<Token> = Vec::new();
    let mut rhs_tokens: Vec<Token> = Vec::new();
    let mut arrow: String = String::new();
    let mut member_idx = 0;
    for tok in tokens {
        match tok.1 {
            Arrow => {
                arrow = tok.0.clone();
                member_idx += 1;
            }
            _ => {
                match member_idx {
                    0 => lhs_tokens.push(tok.clone()),
                    1 => rhs_tokens.push(tok.clone()),
                    _ => return Err(PositionedError(
                        format!("more than 2 members in equation"),
                        Some(tok.2),
                    ))
                }
            }
        }
    }
    if member_idx == 1 {
        let (lhs, rhs) = match (
            parse_equation_member(periodic_table, &lhs_tokens),
            parse_equation_member(periodic_table, &rhs_tokens)
        ) {
            (Ok(l), Ok(r)) => (l, r),
            (Err(err), _) => return Err(err),
            (_, Err(err)) => return Err(err)
        };
        Ok(QuantifiedEquation { lhs, rhs, arrow })
    } else {
        Err(PositionedError(
            format!("an equation must have exactly 2 members"),
            None,
        ))
    }
}

pub fn parse_raw_equation(periodic_table: &PeriodicTable, tokens: &Vec<Token>) -> Result<RawEquation, PositionedError> {
    let quant_eq = return_on_error!(parse_quantified_equation(periodic_table, tokens));
    if quant_eq.is_raw_eq() {
        Ok(quant_eq.to_raw_eq())
    }
    else {
        Err(PositionedError(format!("expected chemical equation without coefficients or quantities"), None))
    }
}
