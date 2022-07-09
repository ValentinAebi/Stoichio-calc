use std::collections::BTreeMap;
use std::fmt;
use std::fmt::Formatter;
use std::hash::Hash;

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
            let status_res;
            if *coef == 1 {
                status_res = write!(f, "{}", atom.code);
            } else {
                status_res = write!(f, "{}{}", atom.code, coef);
            };
            if status_res.is_err() { return status_res }
        }
        Ok(())
    }
}

// TODO implement Display for RawEquation

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct RawEquation {
    pub lhs: Vec<Molecule>,
    pub rhs: Vec<Molecule>
}
