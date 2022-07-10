use std::collections::BTreeMap;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::hash::Hash;
use crate::return_on_error;

#[derive(PartialEq, Eq, Hash, Ord, PartialOrd, Debug, Clone)]
pub struct Atom {
    pub code: String,
    pub name: String,
    pub atomic_mass_milli_uma: i64,
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct Molecule {
    pub atoms: BTreeMap<Atom, u32>,
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
