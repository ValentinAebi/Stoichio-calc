extern crate core;

#[path = "test_atoms.rs"]
mod test_atoms;

#[cfg(test)]
mod chemistry_tests {
    use std::collections::btree_map::BTreeMap;
    use Stoichio_calc::chemistry::Molecule;
    use crate::test_atoms;

    #[test]
    fn molecular_mass_c6h12o6_test(){
        let molecule = Molecule {
            atoms: BTreeMap::from([
                (test_atoms::carbon(), 6),
                (test_atoms::hydrogen(), 12),
                (test_atoms::oxygen(), 6)
            ]),
            charge: 0
        };
        assert_near(180.16, molecule.mass_uma(), 0.05)
    }

    fn assert_near(expected: f64, actual: f64, margin: f64){
        let ok = (expected - actual).abs() <= margin;
        if !ok {
            panic!("expected {}, was {}", expected, actual);
        }
    }

}
