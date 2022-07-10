
#[path = "test_atoms.rs"]
mod test_atoms;

#[cfg(test)]
mod parsing_tests {
    use std::collections::BTreeMap;
    use Stoichio_calc::chemistry::{Atom, Molecule, RawEquation};

    use Stoichio_calc::parsing::{parse_molecule, parse_raw_equation, Token, tokenize, TokenType};
    use crate::test_atoms;

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
    fn parenthesis_bracket_mismatch_test(){
        expect_molecule_parsing_failure("Se(CH3]2O")
    }

    #[test]
    fn bracket_parenthesis_mismatch_test(){
        expect_molecule_parsing_failure("Se[CH3)2O")
    }

    #[test]
    fn unclosed_parenthesis_test(){
        expect_molecule_parsing_failure("Se(CH3O")
    }

    #[test]
    fn unclosed_bracket_test(){
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
        expect_molecule_parsing_success("Se(CH3)2O", se_c2_h6_o);
    }

    #[test]
    fn parse_ch3_ch2_4_cooh(){
        let c6_h12_o2 = BTreeMap::from([
            (test_atoms::carbon(), 6),
            (test_atoms::hydrogen(), 12),
            (test_atoms::oxygen(), 2)
        ]);
        expect_molecule_parsing_success("CH3(CH2)4COOH", c6_h12_o2);
    }

    #[test]
    fn parse_na_rb5_pu2_o12_h4(){
        let na_rb5_pu2_o12_h4 = BTreeMap::from([
            (test_atoms::sodium(), 1),
            (test_atoms::rubidium(), 5),
            (test_atoms::plutonium(), 2),
            (test_atoms::oxygen(), 12),
            (test_atoms::hydrogen(), 4)
        ]);
        expect_molecule_parsing_success("NaRb5[PuO4(OH)2]2", na_rb5_pu2_o12_h4);
    }

    #[test]
    fn parse_photosynthesis_equation(){
        let eq_str = "C6H12O6 + O2 => H2O + CO2".to_string();
        let c6h12o6 = Molecule {
            atoms: BTreeMap::from([
                (test_atoms::carbon(), 6),
                (test_atoms::hydrogen(), 12),
                (test_atoms::oxygen(), 6)
            ])
        };
        let o2 = Molecule {
            atoms: BTreeMap::from([(test_atoms::oxygen(), 2)])
        };
        let h2o = Molecule {
            atoms: BTreeMap::from([(test_atoms::hydrogen(), 2), (test_atoms::oxygen(), 1)])
        };
        let co2 = Molecule {
            atoms: BTreeMap::from([(test_atoms::carbon(), 1), (test_atoms::oxygen(), 2)])
        };
        let expected = RawEquation {
            lhs: Vec::from([c6h12o6, o2]),
            rhs: Vec::from([h2o, co2])
        };
        let actual_res =
            parse_raw_equation(&test_atoms::atoms_map(), &tokenize(&eq_str));
        assert!(actual_res.is_ok());
        assert_eq!(expected, actual_res.unwrap());
    }

    fn expect_molecule_parsing_success(input: &str, expected_result: BTreeMap<Atom, u32>){
        let parsed = parse_molecule(
            &test_atoms::atoms_map(),
            &tokenize(&input.to_string()),
        );
        match parsed {
            Result::Ok(molecule) => {
                assert_eq!(expected_result, molecule.atoms);
            }
            Result::Err(msg) => {
                println!("Failure: parsed was:");
                println!("{}", msg);
                assert!(false);
            }
        }
    }

    fn expect_molecule_parsing_failure(input: &str){
        let result = parse_molecule(
            &test_atoms::atoms_map(),
            &tokenize(&input.to_string()),
        );
        assert!(result.is_err());
    }
}


