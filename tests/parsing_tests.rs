mod parsing_tests {
    use Stoichio_calc::parsing::{parse_atom, Token, tokenize, TokenType};
    use Stoichio_calc::parsing::TokenType::{Alphabetic, Numeric};

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
    fn parse_atom_test() {
        let molec_str = "H2O".to_string();
        let mut tokens = tokenize(&molec_str);
        let act = parse_atom(test_atoms::atoms_map(), &mut tokens);
        assert_eq!(Vec::from([
            Token("2".to_string(), Numeric),
            Token("O".to_string(), Alphabetic)
        ]), tokens);
        assert_eq!(Some(test_atoms::hydrogen()), act)
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

        pub fn all_atoms() -> Vec<Atom> {
            Vec::from([
                hydrogen(), carbon(), oxygen(), nitrogen(), sodium()
            ])
        }

        pub fn atoms_map() -> BTreeMap<String, Atom> {
            all_atoms().iter().map(
                |atom| { (atom.code.clone(), atom.clone()) }
            ).collect()
        }

    }

}


