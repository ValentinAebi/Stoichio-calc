use std::collections::btree_map::BTreeMap;
use std::collections::HashSet;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::hash::Hash;
use crate::arith::lcm_vec;
use crate::chemistry::ChemUnit::{Gram, Milligram, Mol};
use crate::lin_alg::Matrix;
use crate::parsing::PositionedError;
use crate::return_on_error;

#[derive(PartialEq, Eq, Hash, Ord, PartialOrd, Debug, Clone)]
pub struct Atom {
    pub code: String,
    pub name: String,
    pub atomic_mass_milli_amu: u64,
}

impl Atom {
    pub fn atomic_mass_amu(&self) -> f64 {
        (self.atomic_mass_milli_amu as f64) / 1000.0
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct Molecule {
    pub atoms: BTreeMap<Atom, u32>,
    pub charge: i32,
    pub string_repr: Option<String>,
}

impl Molecule {
    pub fn mass_milli_amu(&self) -> u64 {
        let mut sum: u64 = 0;
        for (atom, &coef) in &self.atoms {
            sum += (coef as u64) * atom.atomic_mass_milli_amu;
        }
        sum
    }

    pub fn mass_amu(&self) -> f64 {
        (self.mass_milli_amu() as f64) / 1000.0
    }

    fn default_fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for (atom, coef) in &self.atoms {
            if *coef == 1 {
                return_on_error!(write!(f, "{}", atom.code));
            } else {
                return_on_error!(write!(f, "{}{}", atom.code, coef));
            };
        }
        Ok(())
    }
}

impl fmt::Display for Molecule {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if let Some(string_repr) = &self.string_repr {
            write!(f, "{}", string_repr)
        } else {
            self.default_fmt(f)
        }
    }
}

pub type PeriodicTable = BTreeMap<String, Atom>;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RawEquation {
    pub lhs: Vec<Molecule>,
    pub rhs: Vec<Molecule>,
    pub arrow: String,
}

impl RawEquation {
    pub fn all_atoms_ordered(&self) -> Vec<Atom> {
        self.lhs.iter().flat_map(|molec| { molec.atoms.keys().cloned() }).collect()
    }

    pub fn all_atoms_set(&self) -> HashSet<Atom> {
        HashSet::from_iter(self.all_atoms_ordered().iter().cloned())
    }
}

impl Display for RawEquation {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let lhs_strs: Vec<String> = self.lhs.iter().map(|molec| { molec.to_string() }).collect();
        let lhs_str = lhs_strs.join(" + ");
        let rhs_strs: Vec<String> = self.rhs.iter().map(|molec| { molec.to_string() }).collect();
        let rhs_str = rhs_strs.join(" + ");
        write!(f, "{} {} {}", lhs_str, self.arrow, rhs_str)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct BalancedEquation {
    pub lhs: Vec<(Molecule, i32)>,
    pub rhs: Vec<(Molecule, i32)>,
    pub arrow: String,
}

fn format_balanced_equation_member(member: &Vec<(Molecule, i32)>) -> String {
    let strs: Vec<String> = member.iter()
        .map(|(molec, coef)| {
            format!("{}{}", if *coef == 1 { "".to_string() } else { format!("{} ", coef) }, molec)
        })
        .collect();
    strs.join(" + ")
}

impl Display for BalancedEquation {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", format_balanced_equation_member(&self.lhs), self.arrow, format_balanced_equation_member(&self.rhs))
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum ChemUnit {
    Gram,
    Milligram,
    Mol,
}

impl Display for ChemUnit {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let repres = match self {
            Gram => "g",
            Milligram => "mg",
            Mol => "mol"
        };
        write!(f, "{}", repres)
    }
}

pub fn chem_unit_for(txt: &String) -> Result<ChemUnit, ()> {
    match txt.as_str() {
        "g" => Ok(Gram),
        "mg" => Ok(Milligram),
        "mol" => Ok(Mol),
        _ => Err(())
    }
}

#[derive(Debug, Clone)]
pub struct ChemQuantity(pub f64, pub ChemUnit);

impl Display for ChemQuantity {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:.3} {}", self.0, self.1)
    }
}

#[derive(Debug, Clone)]
pub struct QuantifiedEquation {
    pub lhs: Vec<(Molecule, Option<ChemQuantity>)>,
    pub rhs: Vec<(Molecule, Option<ChemQuantity>)>,
    pub arrow: String,
}

impl QuantifiedEquation {
    pub fn to_raw_eq(&self) -> RawEquation {
        RawEquation {
            lhs: self.lhs.iter().map(|p| { p.0.clone() }).collect(),
            rhs: self.rhs.iter().map(|p| { p.0.clone() }).collect(),
            arrow: self.arrow.clone(),
        }
    }

    fn false_if_one_matches(member: &Vec<(Molecule, Option<ChemQuantity>)>, pred: fn(&Option<ChemQuantity>) -> bool) -> bool {
        for (_, opt) in member {
            if pred(opt) {
                return false;
            }
        }
        true
    }

    fn is_member_raw(member: &Vec<(Molecule, Option<ChemQuantity>)>) -> bool {
        Self::false_if_one_matches(member, Option::is_some)
    }

    fn has_member_all_units(member: &Vec<(Molecule, Option<ChemQuantity>)>) -> bool {
        Self::false_if_one_matches(member, Option::is_none)
    }

    pub fn is_raw_eq(&self) -> bool {
        Self::is_member_raw(&self.lhs) && Self::is_member_raw(&self.rhs)
    }

    /// returns `true` iff the quantities are known for all recatives and unknown for all products
    pub fn only_reactants_quantities_known(&self) -> bool {
        Self::has_member_all_units(&self.lhs) && Self::is_member_raw(&self.rhs)
    }

    fn convert(conv_fn: fn(&Molecule, &ChemQuantity) -> ChemQuantity, member: &Vec<(Molecule, Option<ChemQuantity>)>) -> Vec<(Molecule, Option<ChemQuantity>)> {
        member.iter().map(|(molec, qty_opt)| {
            (molec.clone(), qty_opt.clone().map(|q| { conv_fn(&molec.clone(), &q) }))
        }).collect()
    }

    pub fn quantities_to_mol(&self) -> QuantifiedEquation {
        QuantifiedEquation {
            lhs: Self::convert(to_mol, &self.lhs),
            rhs: Self::convert(to_mol, &self.rhs),
            arrow: self.arrow.clone(),
        }
    }

    pub fn quantities_to_grams(&self) -> QuantifiedEquation {
        QuantifiedEquation {
            lhs: Self::convert(to_gram, &self.lhs),
            rhs: Self::convert(to_gram, &self.rhs),
            arrow: self.arrow.clone(),
        }
    }
}

fn format_quant_equation_member(member: &Vec<(Molecule, Option<ChemQuantity>)>) -> String {
    let strs: Vec<String> = member.iter().map(|(molec, quant_opt)| {
        if let Some(quant) = quant_opt {
            format!("{} {}", quant, molec)
        } else {
            format!("{}", molec)
        }
    }).collect();
    strs.join(" + ")
}

impl Display for QuantifiedEquation {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}",
               format_quant_equation_member(&self.lhs),
               self.arrow,
               format_quant_equation_member(&self.rhs)
        )
    }
}

fn to_mol(molec: &Molecule, qty: &ChemQuantity) -> ChemQuantity {
    let ChemQuantity(value, unit) = qty;
    let factor = match unit {
        Gram => molec.mass_amu(),
        Milligram => molec.mass_milli_amu() as f64,
        Mol => 1.0
    };
    ChemQuantity(value / factor, Mol)
}

fn to_gram(molec: &Molecule, qty: &ChemQuantity) -> ChemQuantity {
    let ChemQuantity(value, unit) = qty;
    let factor = match unit {
        Gram => 1.0,
        Milligram => 0.001,
        Mol => molec.mass_amu()
    };
    ChemQuantity(value * factor, Gram)
}

fn matrix_for(raw_eq: &RawEquation) -> Matrix {
    let mut coefs: Vec<Vec<i32>> = Vec::new();
    for elem in &raw_eq.all_atoms_set() {
        let mut curr_row: Vec<i32> = Vec::new();
        for molec in &raw_eq.lhs {
            curr_row.push(*molec.atoms.get(elem).unwrap_or(&0u32) as i32)
        }
        for molec in &raw_eq.rhs {
            curr_row.push(-(*molec.atoms.get(elem).unwrap_or(&0u32) as i32))
        }
        coefs.push(curr_row)
    }
    let mut charges_row: Vec<i32> = Vec::new();
    for molec in &raw_eq.lhs {
        charges_row.push(molec.charge);
    }
    for molec in &raw_eq.rhs {
        charges_row.push(-molec.charge);
    }
    coefs.push(charges_row);
    Matrix::of_row_major(&coefs)
}

fn deduce_sols(matrix: &Matrix) -> Vec<i32> {
    assert_eq!(matrix.n_rows() + 1, matrix.n_cols());
    let diag = matrix.diagonal();
    let last_col = matrix.column(matrix.n_cols() - 1);
    let lcm = lcm_vec(&diag);
    let mut solution: Vec<i32> = diag.iter().zip(last_col)
        .map(|(pivot, last_col_coef)| { -last_col_coef * lcm / pivot })
        .collect();
    solution.push(lcm);
    solution
}

fn solution_vec_for_balancing_of(raw_eq: &RawEquation) -> Result<Vec<i32>, PositionedError> {
    match matrix_for(raw_eq).diagonalized().map(|matrix| { matrix.without_full_zero_rows() }) {
        Ok(matrix) => {
            let n_rows = matrix.n_rows();
            let n_cols = matrix.n_cols();
            if n_rows + 1 < n_cols {
                Err(PositionedError(format!("solving failed, underconstrained equation"), None))
            } else if n_rows + 1 == n_cols {
                Ok(deduce_sols(&matrix))
            } else {
                Err(PositionedError(format!("cannot balance equation"), None))
            }
        }
        Err(_) => Err(PositionedError(format!("cannot balance equation"), None))
    }
}

pub fn balance(raw_eq: &RawEquation) -> Result<BalancedEquation, PositionedError> {
    let solution_vec = return_on_error!(solution_vec_for_balancing_of(raw_eq));
    let (lhs_sols, rhs_sols) = solution_vec.split_at(raw_eq.lhs.len());
    Ok(BalancedEquation {
        lhs: raw_eq.lhs.iter().zip(lhs_sols)
            .map(|(molec, coef)| { (molec.clone(), coef.clone()) })
            .collect(),
        rhs: raw_eq.rhs.iter().zip(rhs_sols)
            .map(|(molec, coef)| { (molec.clone(), coef.clone()) })
            .collect(),
        arrow: raw_eq.arrow.clone(),
    })
}

/// returns the equation with all coefficients and the limiting reactant
pub fn compute_lhs_coefs(quant_eq: &QuantifiedEquation) -> Result<(QuantifiedEquation, Molecule), PositionedError> {
    if quant_eq.only_reactants_quantities_known() {
        let lhs_n_mol: Vec<f64> = quant_eq.lhs.iter().map(|(molec, quant_opt)| {
            to_mol(molec, &quant_opt.clone().unwrap()).0
        }).collect();
        let pos_vect_res = solution_vec_for_balancing_of(&quant_eq.to_raw_eq());
        let (lhs_stoic_coefs, rhs_stoic_coefs) =
            match &pos_vect_res {
                Ok(sol_vec) => sol_vec.split_at(lhs_n_mol.len()),
                Err(err) => return Err(err.clone())
            };
        let times_reaction: Vec<f64> = lhs_stoic_coefs.iter().zip(lhs_n_mol).map(|(st_coef, n_mol)| {
            (n_mol as f64) / (*st_coef as f64)
        }).collect();
        let times_reaction_per_reactant: Vec<(Molecule, f64)> = quant_eq.lhs.iter()
            .map(|(molec, _)| { molec.clone() }).zip(times_reaction)
            .collect();
        let mut min: (Molecule, f64) = (Molecule { atoms: BTreeMap::new(), charge: 0, string_repr: None }, f64::MAX);
        for (molec, times_reac) in times_reaction_per_reactant {
            if times_reac < min.1 {
                min = (molec, times_reac)
            }
        }
        let rhs_quantities: Vec<ChemQuantity> = rhs_stoic_coefs.iter()
            .map(|&stoic_coef| { ChemQuantity((stoic_coef as f64) * min.1, Mol) })
            .collect();
        let rhs: Vec<(Molecule, Option<ChemQuantity>)> = quant_eq.rhs.iter()
            .zip(rhs_quantities).map(|((m, _), quant)| { (m.clone(), Some(quant)) })
            .collect();
        Ok((QuantifiedEquation {
            rhs,
            ..quant_eq.quantities_to_mol()
        }, min.0))
    } else {
        Err(PositionedError(format!("quantities should be given for all reactants and for no product"), None))
    }
}
