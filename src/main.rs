use std::collections::BTreeMap;
use Stoichio_calc::chemistry::{Atom, Molecule};

fn main() {
    let hydrogen = Atom {
        name : "hydrogen".to_string(),
        code: "H".to_string(),
        atomic_mass_milli_uma: 1_007
    };
    let oxygen = Atom {
        name: "oxygen".to_string(),
        code: "O".to_string(),
        atomic_mass_milli_uma: 15_999
    };
    let natrium = Atom {
        name: "natrium".to_string(),
        code: "Na".to_string(),
        atomic_mass_milli_uma: 14_007
    };
    println!("{}\n{}", natrium, oxygen);
    let water = Molecule {
        atoms: BTreeMap::from([
            (hydrogen, 2),
            (oxygen, 1)
        ])
    };
    println!("{}", water);
}
