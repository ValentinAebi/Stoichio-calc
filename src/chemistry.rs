use std::collections::btree_map::BTreeMap;
use std::collections::HashSet;
use std::fmt;
use std::fmt::{Display, format, Formatter};
use std::hash::Hash;
use crate::arith::lcm_vec;
use crate::lin_alg::Matrix;
use crate::parsing::PositionedError;
use crate::return_on_error;

#[derive(PartialEq, Eq, Hash, Ord, PartialOrd, Debug, Clone)]
pub struct Atom {
    pub code: String,
    pub name: String,
    pub atomic_mass_milli_uma: u64,
}

impl Atom {
    pub fn atomic_mass_uma(&self) -> f64 {
        (self.atomic_mass_milli_uma as f64) / 1000.0
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct Molecule {
    pub atoms: BTreeMap<Atom, u32>,
    pub charge: i32,
    pub string_repr: Option<String>
}

impl Molecule {

    pub fn mass_milli_uma(&self) -> u64 {
        let mut sum: u64 = 0;
        for (atom, &coef) in &self.atoms {
            sum += (coef as u64) * atom.atomic_mass_milli_uma;
        }
        sum
    }

    pub fn mass_uma(&self) -> f64 {
        (self.mass_milli_uma() as f64) / 1000.0
    }

    pub fn chemically_equals(&self, other: &Molecule) -> bool {
        self.atoms == other.atoms && self.charge == other.charge
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
        }
        else {
            self.default_fmt(f)
        }
    }
}

pub type PeriodicTable = BTreeMap<String, Atom>;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct RawEquation {
    pub lhs: Vec<Molecule>,
    pub rhs: Vec<Molecule>
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
        let lhs_strs: Vec<String> = self.lhs.iter().map(|molec|{ molec.to_string() }).collect();
        let lhs_str = lhs_strs.join(" + ");
        let rhs_strs: Vec<String> = self.rhs.iter().map(|molec|{ molec.to_string() }).collect();
        let rhs_str = rhs_strs.join(" + ");
        write!(f, "{} => {}", lhs_str, rhs_str)
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct BalancedEquation {
    pub lhs: Vec<(Molecule, i32)>,
    pub rhs: Vec<(Molecule, i32)>
}

fn format_equation_member(member: &Vec<(Molecule, i32)>) -> String {
    let strs: Vec<String> = member.iter()
        .map(|(molec, coef)|{
            format!("{}{}", if *coef == 1 { "".to_string() } else { format!("{} ", coef) }, molec)
        })
        .collect();
    strs.join(" + ")
}

impl Display for BalancedEquation {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} => {}", format_equation_member(&self.lhs), format_equation_member(&self.rhs))
    }
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
    assert_eq!(matrix.n_rows()+1, matrix.n_cols());
    let diag = matrix.diagonal();
    let last_col = matrix.column(matrix.n_cols()-1);
    let lcm = lcm_vec(&diag);
    let mut solution: Vec<i32> = diag.iter().zip(last_col)
        .map(|(pivot, last_col_coef)|{ -last_col_coef * lcm / pivot })
        .collect();
    solution.push(lcm);
    solution
}

pub fn balance(raw_eq: &RawEquation) -> Result<BalancedEquation, PositionedError> {
    match matrix_for(raw_eq).diagonalized().map(|matrix|{ matrix.without_full_zero_rows() }) {
        Ok(matrix) => {
            let n_rows = matrix.n_rows();
            let n_cols = matrix.n_cols();
            if n_rows + 1 < n_cols {
                Err(PositionedError(format!("solving failed, underconstrained equation"), None))
            }
            else if n_rows + 1 == n_cols {
                let solution_vec = deduce_sols(&matrix);
                let (lhs_sols, rhs_sols) = solution_vec.split_at(raw_eq.lhs.len());
                Ok(BalancedEquation {
                    lhs: raw_eq.lhs.iter().zip(lhs_sols)
                        .map(|(molec, coef)|{(molec.clone(), coef.clone())})
                        .collect(),
                    rhs: raw_eq.rhs.iter().zip(rhs_sols)
                        .map(|(molec, coef)|{(molec.clone(), coef.clone())})
                        .collect()
                })
            }
            else {
                Err(PositionedError(format!("cannot balance equation"), None))
            }
        }
        Err(_) => Err(PositionedError(format!("cannot balance equation"), None))
    }
}
