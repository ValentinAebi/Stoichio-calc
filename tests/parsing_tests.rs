mod parsing_tests {
    use std::collections::BTreeMap;

    use Stoichio_calc::parsing::{parse_molecule, Token, tokenize, TokenType};

    #[test]
    fn tokenize_test() {
        let equation_str = "C6H12O6 + O2 => NaCl";
        let actual = tokenize(&equation_str.to_string());
        let exp: Vec<Token> = Vec::from([
            Token("C".to_string(), TokenType::Alphabetic),
            Token("6".to_string(), TokenType::Numeric),
            Token("H".to_string(), TokenType::Alphabetic),
            Token("12".to_string(), TokenType::Numeric),
            Token("O".to_string(), TokenType::Alphabetic),
            Token("6".to_string(), TokenType::Numeric),
            Token(" ".to_string(), TokenType::Whitespace),
            Token("+".to_string(), TokenType::Plus),
            Token(" ".to_string(), TokenType::Whitespace),
            Token("O".to_string(), TokenType::Alphabetic),
            Token("2".to_string(), TokenType::Numeric),
            Token(" ".to_string(), TokenType::Whitespace),
            Token("=>".to_string(), TokenType::Delimiter),
            Token(" ".to_string(), TokenType::Whitespace),
            Token("Na".to_string(), TokenType::Alphabetic),
            Token("Cl".to_string(), TokenType::Alphabetic)
        ]);
        assert_eq!(exp, actual)
    }

    #[test]
    fn parse_molecule_test() {
        let molec_str = "Se(CH3)2O".to_string();
        let parsed = parse_molecule(
            &test_atoms::atoms_map(),
            tokenize(&molec_str),
        );
        assert!(parsed.is_ok());
        let actual_res = parsed.unwrap().atoms;
        let expected_res = BTreeMap::from([
            (test_atoms::selenium(), 1),
            (test_atoms::carbon(), 2),
            (test_atoms::hydrogen(), 6),
            (test_atoms::oxygen(), 1)
        ]);
        assert_eq!(expected_res, actual_res);
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

        pub fn all_atoms() -> Vec<Atom> {
            Vec::from([
                hydrogen(), carbon(), oxygen(), nitrogen(), sodium(), selenium()
            ])
        }

        pub fn atoms_map() -> BTreeMap<String, Atom> {
            all_atoms().iter().map(
                |atom| { (atom.code.clone(), atom.clone()) }
            ).collect()
        }
    }
}


