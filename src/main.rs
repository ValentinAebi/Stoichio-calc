use Stoichio_calc::chemistry::{Atom, Molecule};
use Stoichio_calc::data_loading::load_periodic_table_as_map;
use Stoichio_calc::parsing::{parse_raw_equation, tokenize};

fn main() {
    let eq_str = "H + O2 -> H2O".to_string();
    let parsed_eq = parse_raw_equation(&load_periodic_table_as_map(), &tokenize(&eq_str));
    match parsed_eq {
        Ok(eq) => println!("{}", eq),
        Err(msg) => println!("{}", msg)
    }
}
