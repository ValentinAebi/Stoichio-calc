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

impl fmt::Display for Atom {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({}, {} uma)", self.code, self.name, self.atomic_mass_milli_uma)
    }
}

#[derive(Debug)]
pub struct Molecule {
    pub atoms: BTreeMap<Atom, u32>,
}

impl fmt::Display for Molecule {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for (atom, coef) in &self.atoms {
            if *coef == 1 {
                write!(f, "{}", atom.code);
            } else {
                write!(f, "{}{}", atom.code, coef);
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct RawEquation {
    pub lhs: Vec<(Atom, u32)>,
    pub rhs: Vec<(Atom, u32)>
}
