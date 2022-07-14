use std::collections::BTreeMap;
use Stoichio_calc::chemistry::Molecule;

#[path = "test_atoms.rs"]
mod test_atoms;

pub fn h2o() -> Molecule {
    Molecule {
        atoms: BTreeMap::from([(test_atoms::hydrogen(), 2), (test_atoms::oxygen(), 1)]),
        charge: 0,
        string_repr: Some("H2O".to_string())
    }
}

pub fn co2() -> Molecule {
    Molecule {
        atoms: BTreeMap::from([(test_atoms::carbon(), 1), (test_atoms::oxygen(), 2)]),
        charge: 0,
        string_repr: Some("CO2".to_string())
    }
}

pub fn o2() -> Molecule {
    Molecule {
        atoms: BTreeMap::from([(test_atoms::oxygen(), 2)]),
        charge: 0,
        string_repr: Some("O2".to_string())
    }
}

pub fn c6h12o6() -> Molecule {
    Molecule {
        atoms: BTreeMap::from([
            (test_atoms::carbon(), 6),
            (test_atoms::hydrogen(), 12),
            (test_atoms::oxygen(), 6)
        ]),
        charge: 0,
        string_repr: Some("C6H12O6".to_string())
    }
}

