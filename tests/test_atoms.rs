
use std::collections::BTreeMap;

use Stoichio_calc::chemistry::Atom;

pub fn hydrogen() -> Atom {
    Atom {
        name: "hydrogen".to_string(),
        code: "H".to_string(),
        atomic_mass_milli_uma: 1_007,
    }
}

pub fn carbon() -> Atom {
    Atom {
        name: "carbon".to_string(),
        code: "C".to_string(),
        atomic_mass_milli_uma: 12_011,
    }
}

pub fn oxygen() -> Atom {
    Atom {
        name: "oxygen".to_string(),
        code: "O".to_string(),
        atomic_mass_milli_uma: 15_999,
    }
}

pub fn nitrogen() -> Atom {
    Atom {
        name: "nitrogen".to_string(),
        code: "N".to_string(),
        atomic_mass_milli_uma: 14_007,
    }
}

pub fn sodium() -> Atom {
    Atom {
        name: "sodium".to_string(),
        code: "Na".to_string(),
        atomic_mass_milli_uma: 22_990,
    }
}

pub fn selenium() -> Atom {
    Atom {
        name: "selenium".to_string(),
        code: "Se".to_string(),
        atomic_mass_milli_uma: 78_960,
    }
}

pub fn rubidium() -> Atom {
    Atom {
        name: "rubidium".to_string(),
        code: "Rb".to_string(),
        atomic_mass_milli_uma: 85_468
    }
}

pub fn plutonium() -> Atom {
    Atom {
        name: "plutonium".to_string(),
        code: "Pu".to_string(),
        atomic_mass_milli_uma: 244_000
    }
}

pub fn all_atoms() -> Vec<Atom> {
    Vec::from([
        hydrogen(), carbon(), oxygen(), nitrogen(), sodium(), selenium(), rubidium(), plutonium()
    ])
}

pub fn atoms_map() -> BTreeMap<String, Atom> {
    all_atoms().iter().map(
        |atom| { (atom.code.clone(), atom.clone()) }
    ).collect()
}