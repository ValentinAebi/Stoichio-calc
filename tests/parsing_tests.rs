use Stoichio_calc::parsing::{Token, tokenize, TokenType};

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
