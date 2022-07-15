
#[cfg(test)]
mod data_loading_tests {
    use std::fs;
    use Stoichio_calc::data_loading::load_peridic_table_as_vec;

    #[test]
    fn load_periodic_table_tests(){
        let loaded = load_peridic_table_as_vec(
            fs::read_to_string("./res/periodic_table.csv").unwrap().as_str()
        );
        assert_eq!(118, loaded.len());
        assert_eq!("Neon", loaded.get(9).unwrap().name);
        assert_eq!("Se", loaded.get(33).unwrap().code);
        assert_eq!(174_967, loaded.get(70).unwrap().atomic_mass_milli_amu);
    }

}

