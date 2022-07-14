#[path = "test_atoms.rs"]
mod test_atoms;

#[path = "test_molecules.rs"]
mod test_molecules;

#[cfg(test)]
mod parsing_tests {
    use std::collections::btree_map::BTreeMap;
    use Stoichio_calc::chemistry::{Atom, ChemQuantity, Molecule, QuantifiedEquation, RawEquation};
    use Stoichio_calc::chemistry::ChemUnit::{Gram, Mol};

    use Stoichio_calc::parsing::{parse_molecule, parse_quantified_equation, parse_raw_equation, Token, tokenize, TokenType};
    use crate::{assert_near, test_atoms};
    use crate::test_molecules::{c6h12o6, co2, h2o, o2};

    #[test]
    fn tokenize_valid_string_test() {
        let equation_str = "C6H12O6 + O2 => NaCl";
        let actual = tokenize(&equation_str.to_string());
        let exp: Vec<Token> = Vec::from([
            Token("C".to_string(), TokenType::Alphabetic, 0),
            Token("6".to_string(), TokenType::Numeric, 1),
            Token("H".to_string(), TokenType::Alphabetic, 2),
            Token("12".to_string(), TokenType::Numeric, 3),
            Token("O".to_string(), TokenType::Alphabetic, 5),
            Token("6".to_string(), TokenType::Numeric, 6),
            Token(" ".to_string(), TokenType::Whitespace, 7),
            Token("+".to_string(), TokenType::Plus, 8),
            Token(" ".to_string(), TokenType::Whitespace, 9),
            Token("O".to_string(), TokenType::Alphabetic, 10),
            Token("2".to_string(), TokenType::Numeric, 11),
            Token(" ".to_string(), TokenType::Whitespace, 12),
            Token("=>".to_string(), TokenType::Arrow, 13),
            Token(" ".to_string(), TokenType::Whitespace, 15),
            Token("Na".to_string(), TokenType::Alphabetic, 16),
            Token("Cl".to_string(), TokenType::Alphabetic, 18)
        ]);
        assert_eq!(exp, actual)
    }

    #[test]
    fn parenthesis_bracket_mismatch_test() {
        expect_molecule_parsing_failure("Se(CH3]2O")
    }

    #[test]
    fn bracket_parenthesis_mismatch_test() {
        expect_molecule_parsing_failure("Se[CH3)2O")
    }

    #[test]
    fn unclosed_parenthesis_test() {
        expect_molecule_parsing_failure("Se(CH3O")
    }

    #[test]
    fn unclosed_bracket_test() {
        expect_molecule_parsing_failure("Se[Ch3O")
    }

    #[test]
    fn parse_se_ch3_2_o_test() {
        let se_c2_h6_o = BTreeMap::from([
            (test_atoms::selenium(), 1),
            (test_atoms::carbon(), 2),
            (test_atoms::hydrogen(), 6),
            (test_atoms::oxygen(), 1)
        ]);
        expect_molecule_parsing_success("Se(CH3)2O", se_c2_h6_o, 0);
    }

    #[test]
    fn parse_ch3_ch2_4_cooh_test() {
        let c6_h12_o2 = BTreeMap::from([
            (test_atoms::carbon(), 6),
            (test_atoms::hydrogen(), 12),
            (test_atoms::oxygen(), 2)
        ]);
        expect_molecule_parsing_success("CH3(CH2)4COOH", c6_h12_o2, 0);
    }

    #[test]
    fn parse_na_rb5_pu2_o12_h4_test() {
        let na_rb5_pu2_o12_h4 = BTreeMap::from([
            (test_atoms::sodium(), 1),
            (test_atoms::rubidium(), 5),
            (test_atoms::plutonium(), 2),
            (test_atoms::oxygen(), 12),
            (test_atoms::hydrogen(), 4)
        ]);
        expect_molecule_parsing_success("NaRb5[PuO4(OH)2]2", na_rb5_pu2_o12_h4, 0);
    }

    #[test]
    fn parse_ch3coo_minus_test() {
        let ch3coo = BTreeMap::from([
            (test_atoms::carbon(), 2),
            (test_atoms::hydrogen(), 3),
            (test_atoms::oxygen(), 2)
        ]);
        expect_molecule_parsing_success("CH3COO^-", ch3coo, -1);
    }

    #[test]
    fn parse_c_ch3_4_test() {
        let c_ch3_4 = BTreeMap::from([
            (test_atoms::carbon(), 5),
            (test_atoms::hydrogen(), 12)
        ]);
        expect_molecule_parsing_success("C(CH3^-)4", c_ch3_4, -4)
    }

    #[test]
    fn parse_respiration_equation_test() {
        let eq_str = "C6H12O6 + O2 => H2O + CO2".to_string();
        let expected = RawEquation {
            lhs: Vec::from([c6h12o6(), o2()]),
            rhs: Vec::from([h2o(), co2()]),
            arrow: "=>".to_string(),
        };
        let actual_res =
            parse_raw_equation(&test_atoms::atoms_map(), &tokenize(&eq_str));
        assert!(actual_res.is_ok());
        assert_eq!(expected, actual_res.unwrap());
    }

    #[test]
    fn parse_redox_equation_test() {
        let eq_str = "Fe + Cu^2+ => Fe^2+ + Cu".to_string();
        let fe = Molecule {
            atoms: BTreeMap::from([
                (test_atoms::iron(), 1)
            ]),
            charge: 0,
            string_repr: Some("Fe".to_string()),
        };
        let cu_2plus = Molecule {
            atoms: BTreeMap::from([
                (test_atoms::copper(), 1)
            ]),
            charge: 2,
            string_repr: Some("Cu^2+".to_string()),
        };
        let fe_2plus = Molecule {
            atoms: BTreeMap::from([
                (test_atoms::iron(), 1)
            ]),
            charge: 2,
            string_repr: Some("Fe^2+".to_string()),
        };
        let cu = Molecule {
            atoms: BTreeMap::from([
                (test_atoms::copper(), 1)
            ]),
            charge: 0,
            string_repr: Some("Cu".to_string()),
        };
        let actual_res = parse_raw_equation(&test_atoms::atoms_map(), &tokenize(&eq_str));
        let expected = RawEquation {
            lhs: Vec::from([fe, cu_2plus]),
            rhs: Vec::from([fe_2plus, cu]),
            arrow: "=>".to_string(),
        };
        assert!(actual_res.is_ok());
        assert_eq!(expected, actual_res.unwrap())
    }

    #[test]
    fn parse_quantified_equation_test() {
        let eq_str = "2.3 mol C6H12O6 + O2 => H2O + 1 g CO2".to_string();
        let expected_output = QuantifiedEquation {
            lhs: Vec::from([
                (c6h12o6(), Some(ChemQuantity(2.3, Mol))),
                (o2(), None)
            ]),
            rhs: Vec::from([
                (h2o(), None),
                (co2(), Some(ChemQuantity(1.0, Gram)))
            ]),
            arrow: "=>".to_string(),
        };
        let act_output_res = parse_quantified_equation(&test_atoms::atoms_map(), &tokenize(&eq_str));
        if let Err(err) = &act_output_res {
            println!("{}", err);
        }
        assert!(act_output_res.is_ok());
        let act_output = act_output_res.unwrap();
        let expected_chain: Vec<(Molecule, Option<ChemQuantity>)> = expected_output.lhs.iter().chain(expected_output.rhs.iter())
            .cloned().collect();
        let act_chain: Vec<(Molecule, Option<ChemQuantity>)> = act_output.lhs.iter().chain(act_output.rhs.iter())
            .cloned().collect();
        assert_eq!(expected_chain.len(), act_chain.len());
        for ((exp_molec, exp_quant), (act_molec, act_quant))
        in expected_chain.iter().zip(act_chain) {
            assert_eq!(exp_molec.clone(), act_molec);
            assert_eq!(exp_quant.is_some(), act_quant.is_some());
            if exp_quant.is_some(){
                let ChemQuantity(exp_value, exp_unit) = exp_quant.clone().unwrap();
                let ChemQuantity(act_value, act_unit) = act_quant.clone().unwrap();
                assert_eq!(exp_unit, act_unit);
                assert_near(exp_value, act_value, 1e-2);
            }
        }
    }

    fn expect_molecule_parsing_success(input: &str, expected_atoms: BTreeMap<Atom, u32>, expected_charge: i32) {
        let parsed = parse_molecule(
            &test_atoms::atoms_map(),
            &tokenize(&input.to_string()),
        );
        match parsed {
            Ok(molecule) => {
                assert_eq!(expected_atoms, molecule.atoms);
                assert_eq!(expected_charge, molecule.charge);
            }
            Err(msg) => {
                println!("Failure: parsed was:");
                println!("{}", msg);
                assert!(false);
            }
        }
    }

    fn expect_molecule_parsing_failure(input: &str) {
        let result = parse_molecule(
            &test_atoms::atoms_map(),
            &tokenize(&input.to_string()),
        );
        assert!(result.is_err());
    }
}

fn assert_near(expected: f64, actual: f64, margin: f64){
    let ok = (expected - actual).abs() <= margin;
    if !ok {
        panic!("expected {}, was {}", expected, actual);
    }
}
