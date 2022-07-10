use std::collections::BTreeMap;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::hash::Hash;
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
}

impl fmt::Display for Molecule {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
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

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct RawEquation {
    pub lhs: Vec<Molecule>,
    pub rhs: Vec<Molecule>
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
