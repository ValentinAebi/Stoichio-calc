use crate::chemistry::{Atom, PeriodicTable};

const CSV_SPLITTER: &str = ",";

pub fn load_peridic_table_as_vec(file_content: &str) -> Vec<Atom> {
    file_content.lines().skip(1).map(|line|{
        let cols_to_match: Vec<&str> = line.split(CSV_SPLITTER).take(4).collect();
        match cols_to_match[..] {
            [ _atomic_num, name, code, atomic_mass_str ] => {
                let atomic_mass: f64 = atomic_mass_str.parse().unwrap();
                Atom {
                    name: name.to_string(),
                    code: code.to_string(),
                    atomic_mass_milli_amu: (1000.0 * atomic_mass) as u64
                }
            }
            _ => panic!("could not read recource file line")
        }
    }).collect()
}

pub fn load_periodic_table(file_content: &str) -> PeriodicTable {
    load_peridic_table_as_vec(file_content)
        .iter()
        .map(|atom| { (atom.code.clone(), atom.clone()) })
        .collect()
}

