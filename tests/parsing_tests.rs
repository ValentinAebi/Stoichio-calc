mod parsing_tests {
    use std::collections::BTreeMap;
    use Stoichio_calc::chemistry::Atom;

    use Stoichio_calc::parsing::{parse_molecule, Token, tokenize, TokenType};

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
            Token("=>".to_string(), TokenType::Delimiter, 13),
            Token(" ".to_string(), TokenType::Whitespace, 15),
            Token("Na".to_string(), TokenType::Alphabetic, 16),
            Token("Cl".to_string(), TokenType::Alphabetic, 18)
        ]);
        assert_eq!(exp, actual)
    }

    #[test]
    fn parenthesis_bracket_mismatch_test(){
        expect_parsing_failure("Se(CH3]2O")
    }

    #[test]
    fn bracket_parenthesis_mismatch_test(){
        expect_parsing_failure("Se[CH3)2O")
    }

    #[test]
    fn unclosed_parenthesis_test(){
        expect_parsing_failure("Se(CH3O")
    }

    #[test]
    fn unclosed_bracket_test(){
        expect_parsing_failure("Se[Ch3O")
    }

    #[test]
    fn parse_se_ch3_2_o_test() {
        let se_c2_h6_o = BTreeMap::from([
            (test_atoms::selenium(), 1),
            (test_atoms::carbon(), 2),
            (test_atoms::hydrogen(), 6),
            (test_atoms::oxygen(), 1)
        ]);
        expect_parsing_success("Se(CH3)2O", se_c2_h6_o);
    }

    #[test]
    fn parse_ch3_ch2_4_cooh(){
        let c6_h12_o2 = BTreeMap::from([
            (test_atoms::carbon(), 6),
            (test_atoms::hydrogen(), 12),
            (test_atoms::oxygen(), 2)
        ]);
        expect_parsing_success("CH3(CH2)4COOH", c6_h12_o2);
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
        expect_parsing_success("NaRb5[PuO4(OH)2]2", na_rb5_pu2_o12_h4);
    }

    fn expect_parsing_success(input: &str, expected_result: BTreeMap<Atom, u32>){
        let parsed = parse_molecule(
            &test_atoms::atoms_map(),
            tokenize(&input.to_string()),
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

    fn expect_parsing_failure(input: &str){
        let result = parse_molecule(
            &test_atoms::atoms_map(),
            tokenize(&input.to_string()),
        );
        assert!(result.is_err());
    }

    mod test_atoms {
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
    }
}


