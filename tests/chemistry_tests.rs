extern crate core;

#[path = "test_atoms.rs"]
mod test_atoms;

#[path = "test_molecules.rs"]
mod test_molecules;

#[cfg(test)]
mod chemistry_tests {
    use std::collections::btree_map::BTreeMap;
    use Stoichio_calc::chemistry::{balance, BalancedEquation, ChemQuantity, compute_lhs_coefs, Molecule, QuantifiedEquation, RawEquation};
    use Stoichio_calc::chemistry::ChemUnit::{Gram, Milligram, Mol};
    use crate::test_atoms;
    use crate::test_molecules::{c6h12o6, co2, h2o, o2};

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
        let raw_equation = RawEquation {
            lhs: Vec::from([ h2o(), co2() ]),
            rhs: Vec::from([ c6h12o6(), o2() ]),
            arrow: "=>".to_string()
        };
        let expected_balanced_equation = BalancedEquation {
            lhs: Vec::from([ (h2o(), 6), (co2(), 6) ]),
            rhs: Vec::from([ (c6h12o6(), 1), (o2(), 6) ]),
            arrow: "=>".to_string()
        };
        let balanced_equation_res = balance(&raw_equation);
        assert!(balanced_equation_res.is_ok());
        assert_eq!(expected_balanced_equation, balanced_equation_res.unwrap());
    }

    #[test]
    fn compute_rhs_test(){

        /*
        C6H12O6: 1*9 mol -- 180_160 mg/mol * 9 mol = 180_160 * 9 mg
        6 O2: 6*7 mol -- 32 g/mol * 6*7 mol = 32*6*7 g
        =>
        6 H2O: 6*7 mol -- 18.015 g/mol * 6*7 mol = 18.015*6*7 g
        6 CO2: 6*7 mol -- 44.01 g/mol * 6*7 mol = 44.01*6*7 g
         */

        let eq = QuantifiedEquation {
            lhs: Vec::from([
                (c6h12o6(), Some(ChemQuantity(180_160.0 * 9.0, Milligram))),
                (o2(), Some(ChemQuantity(32.0*6.0*7.0, Gram)))
            ]),
            rhs: Vec::from([
                (h2o(), None),
                (co2(), None)
            ]),
            arrow: "=>".to_string()
        };
        let expected_eq = QuantifiedEquation {
            lhs: Vec::from([
                (c6h12o6(), Some(ChemQuantity(9.0, Mol))),
                (o2(), Some(ChemQuantity(6.0*7.0, Mol)))
            ]),
            rhs: Vec::from([
                (h2o(), Some(ChemQuantity(6.0*7.0, Mol))),
                (co2(), Some(ChemQuantity(6.0*7.0, Mol)))
            ]),
            arrow: "=>".to_string()
        };
        let act_res = compute_lhs_coefs(&eq);
        assert!(act_res.is_ok());
        let (act_eq, act_limiting) = act_res.unwrap();
        assert_eq!(o2(), act_limiting);
        let expected_member = expected_eq.lhs;
        let actual_member = act_eq.lhs;
        let zipped: Vec<(&(Molecule, Option<ChemQuantity>), (Molecule, Option<ChemQuantity>))> =
            expected_member.iter().zip(actual_member).collect();
        for ((exp_molec, exp_q_opt), (act_molec, act_q_opt)) in zipped {
            assert_eq!(exp_molec.clone(), act_molec);
            assert!(act_q_opt.is_some());
            let act_quantity = act_q_opt.unwrap();
            let exp_quantity = exp_q_opt.clone().unwrap();
            assert_near(exp_quantity.0, act_quantity.0, 0.05);
            assert_eq!(exp_quantity.1, act_quantity.1);
        }
    }

    fn assert_near(expected: f64, actual: f64, margin: f64){
        let ok = (expected - actual).abs() <= margin;
        if !ok {
            panic!("expected {}, was {}", expected, actual);
        }
    }

}
