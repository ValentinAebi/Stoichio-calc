extern crate core;

#[path = "test_atoms.rs"]
mod test_atoms;

#[cfg(test)]
mod chemistry_tests {
    use std::collections::btree_map::BTreeMap;
    use Stoichio_calc::chemistry::{balance, BalancedEquation, Molecule, RawEquation};
    use crate::test_atoms;

    #[test]
    fn molecular_mass_c6h12o6_test(){
        let molecule = Molecule {
            atoms: BTreeMap::from([
                (test_atoms::carbon(), 6),
                (test_atoms::hydrogen(), 12),
                (test_atoms::oxygen(), 6)
            ]),
            charge: 0,
            string_repr: None
        };
        assert_near(180.16, molecule.mass_uma(), 0.05)
    }

    #[test]
    fn balance_photosynthesis_eq_test(){
        let h2o = Molecule {
            atoms: BTreeMap::from([(test_atoms::hydrogen(), 2), (test_atoms::oxygen(), 1)]),
            charge: 0,
            string_repr: None
        };
        let co2 = Molecule {
            atoms: BTreeMap::from([(test_atoms::carbon(), 1), (test_atoms::oxygen(), 2)]),
            charge: 0,
            string_repr: None
        };
        let c6h12o6 = Molecule {
            atoms: BTreeMap::from([
                (test_atoms::carbon(), 6),
                (test_atoms::hydrogen(), 12),
                (test_atoms::oxygen(), 6)
            ]),
            charge: 0,
            string_repr: None
        };
        let o2 = Molecule {
            atoms: BTreeMap::from([(test_atoms::oxygen(), 2)]),
            charge: 0,
            string_repr: None
        };
        let raw_equation = RawEquation {
            lhs: Vec::from([ h2o.clone(), co2.clone() ]),
            rhs: Vec::from([ c6h12o6.clone(), o2.clone() ])
        };
        let expected_balanced_equation = BalancedEquation {
            lhs: Vec::from([ (h2o, 6), (co2, 6) ]),
            rhs: Vec::from([ (c6h12o6, 1), (o2, 6) ])
        };
        let balanced_equation_res = balance(&raw_equation);
        assert!(balanced_equation_res.is_ok());
        assert_eq!(expected_balanced_equation, balanced_equation_res.unwrap());
    }

    fn assert_near(expected: f64, actual: f64, margin: f64){
        let ok = (expected - actual).abs() <= margin;
        if !ok {
            panic!("expected {}, was {}", expected, actual);
        }
    }

}
