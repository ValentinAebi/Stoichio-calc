use std::collections::BTreeMap;
use std::fs;
use crate::chemistry::Atom;

const CSV_SPLITTER: &str = ",";

pub fn load_peridic_table_as_vec() -> Vec<Atom> {
    match fs::read_to_string("./res/periodic_table.csv") {
        Result::Ok(content) => {
            content.lines().skip(1).map(|line|{
                let cols_to_match: Vec<&str> = line.split(CSV_SPLITTER).take(4).collect();
                match cols_to_match[..] {
                    [ _atomic_num, name, code, atomic_mass_str ] => {
                        let atomic_mass: f64 = atomic_mass_str.parse().unwrap();
                        Atom {
                            name: name.to_string(),
                            code: code.to_string(),
                            atomic_mass_milli_uma: (1000.0 * atomic_mass) as u64
                        }
                    }
                    _ => panic!("could not read recource file line")
                }
            }).collect()
        }
        Result::Err(_) => {
            panic!("could not load resource file")
        }
    }

}

pub fn load_periodic_table_as_map() -> BTreeMap<String, Atom> {
    load_peridic_table_as_vec()
        .iter()
        .map(|atom| { (atom.code.clone(), atom.clone()) })
        .collect()
}

