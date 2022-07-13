/*
 * Despite being called end-to-end, these tests do not test the CLI
 *
 * The equations are taken from http://www.sky-web.net/science/balancing_chemical_equations_examples.htm
 */

mod end_to_end_tests {

    use Stoichio_calc::chemistry::balance;
    use Stoichio_calc::data_loading::load_periodic_table;
    use Stoichio_calc::parsing::{parse_raw_equation, tokenize};

    macro_rules! equation_balancing_test {
        ($input: literal, $expected: literal, $name: ident) => {
            #[test]
            fn $name() {
                perform_equation_balancing_test($input, $expected);
            }
        };
    }

    equation_balancing_test!("C5H12 + O2 => CO2 + H2O", "C5H12 + 8 O2 => 5 CO2 + 6 H2O", c5h12_x_o2_test);

    equation_balancing_test!("Zn + HCl => ZnCl2 + H2", "Zn + 2 HCl => ZnCl2 + H2", zn_x_hcl_test);

    equation_balancing_test!("Ca(OH)2 + H3PO4 => Ca3(PO4)2 + H2O", "3 Ca(OH)2 + 2 H3PO4 => Ca3(PO4)2 + 6 H2O", ca_oh2_x_h3po4_test);

    equation_balancing_test!("FeCl3 + NH4OH => Fe(OH)3 + NH4Cl", "FeCl3 + 3 NH4OH => Fe(OH)3 + 3 NH4Cl", fecl3_x_nh4oh_test);

    equation_balancing_test!("S8 + F2 => SF6", "S8 + 24 F2 => 8 SF6", s8_x_f2_test);

    equation_balancing_test!("C2H6 + O2 => CO2 + H2O", "2 C2H6 + 7 O2 => 4 CO2 + 6 H2O", c2h6_x_o2_test);

    equation_balancing_test!("Al2(CO3)3 + H3PO4 => AlPO4 + CO2 + H2O", "Al2(CO3)3 + 2 H3PO4 => 2 AlPO4 + 3 CO2 + 3 H2O", al2_co3_3_x_h3po4_test);

    fn perform_equation_balancing_test(input_eq: &str, expected_output_eq: &str) {
        let parsed_raw_eq =
            parse_raw_equation(
                &load_periodic_table(),
                &tokenize(&input_eq.to_string()),
            );
        assert!(parsed_raw_eq.is_ok());
        let balanced_eq = balance(&parsed_raw_eq.unwrap());
        assert!(balanced_eq.is_ok());
        let actual_output = balanced_eq.unwrap().to_string();
        assert_eq!(expected_output_eq.to_string(), actual_output)
    }
}
