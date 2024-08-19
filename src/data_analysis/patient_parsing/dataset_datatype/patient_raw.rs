//! Raw schema of patients to match the patients dataset schema  

use serde::{Deserialize, Serialize};

use crate::data_analysis::constants::DATA_PATH;

use super::patient::Patient;

use super::super::deserialize_dataset::OutputSchema;

// I campi devono essere gli stessi delle chiavi del dict SCHEMA che si trova in
// "/home/giulio/Scrivania/materiale-universita/TIROCINIO/analisi_dati/pycode/my_py_modules/local_utils.py"
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PatientRaw {
    pub sex: String,
    pub id: String,
    pub età: String,
    pub kg: String,
    pub bmi: String,
    pub frc: String, // fattori di rischio
    pub sintomi: String,
    //pub ecg: String,
    //pub eco_basale: String,
    //pub eco_stress: String,
    //pub stress_test: String,
    pub pre_test: String,  // rischio stimato pre
    pub post_test: String, // rischio stimato post
}

pub fn create_refined_csv(rusty_patients: Vec<Patient>) {
    let file_path = DATA_PATH.to_string() + "refined_data.csv";
    let mut writer = csv::Writer::from_path(file_path).expect("creating the writer");

    for paz in rusty_patients.into_iter() {
        let paz = OutputSchema::from_patient(paz);
        writer.serialize(paz).expect("wrinting patient");
    }
    writer.flush().expect("flushing");
}

pub fn is_wrong_value(value: &str) -> bool {
    let values = value.split(",").map(|x| x.trim().to_lowercase());

    for value in values {
        match value.as_str() {
            // Correct strings
            "disll" | "didsl" | "dis" => return true,
            "fuy" => return true,
            "ipb" => return true,

            // Multiple strings
            "dislob" => return true,
            "fam ip" | "fam.ip" | "famip" | "fip" => return true,
            "fuip" => return true,
            "ip.dmnid" => return true,
            "ip;disl" => return true,
            "potus" => return true,
            _ => (),
        }
    }

    false
}

pub fn _see_data(input: String) {
    let sample = input
        .lines()
        .skip(1) //.take(10)
        .map(|x| x.split("\"").nth(1).unwrap_or("None"))
        .collect::<Vec<_>>();

    let sample = sample
        .iter()
        .map(|x: &&str| {
            let k = x.split(",").filter_map(|x: &str| {
                let k = match_possible_frc(x);
                match k.len() {
                    0..=5 => Some(k),
                    _ => None,
                }
            });
            k.collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    for (n, i) in sample.iter().enumerate() {
        let n = n + 2;
        if i.len() > 0 {
            println!("{} -- {:?} ", n, i);
        }
    }
}

pub fn _print_wrong_frc(input: String) {
    let sample = input
        .lines()
        .skip(1) //.take(100)
        .map(|x| x.split("\""))
        .map(|mut x| {
            let id = x.nth(0).unwrap().split(",").nth(1).unwrap_or("None");
            let frc = x.nth(0).unwrap_or("None");
            (id, frc)
        })
        .filter(|(_, frc)| is_wrong_value(frc))
        .collect::<Vec<_>>();
    println!("{}", sample.len());
    for i in sample.iter() {
        println!("\nid: {}\nfrc: {}\n", i.0, i.1);
    }
}

pub fn match_possible_frc(frc: &str) -> Vec<&str> {
    // il problema di questa funzione è che non valuta l'intera casella
    // ma solo la porzione di stringa splittata con la virgola

    // Process input
    let frc = frc.trim().to_lowercase();

    // se aggiungi una variante qui devi anche aggiungerla nelle
    // varianti dell'enum FattoreRischio e nel suo metodo .from_string
    match frc.as_str() {
        // Single-value matches
        "disl" | "disll" | "didsl" | "dis" => vec!["dislipidemia"],
        "dmid" => vec!["diabete_id"],   // insulino dipendente
        "dmnid" => vec!["diabete_nid"], // non insulino dipendente
        "fam" => vec!["familiarità"],
        "fu" | "fuy" => vec!["fumo"],
        "ip" | "ipb" => vec!["ipertensione"],
        "ob" => vec!["obesità"],

        // Multi-value matches
        "dislob" => vec!["dislipidemia", "obesità"],
        "fam ip" | "fam.ip" | "famip" | "fip" => vec!["familiarità", "ipertensione"],
        "fuip" => vec!["fumo", "ipertensione"],
        "ip.dmnid" => vec!["ipertensione", "diabete_nid"],
        "ip;disl" => vec!["ipertensione", "dislipidemia"],

        // Special cases
        "iperinsulinemia" | "potus" => vec![], // nb ha causato un [nessuno+ipertensione]
        // in questo caso ho messo vuoto per fare veloce
        // ma prima era vec!["Nessuno"]

        // Default case
        _ => vec!["Nessuno"],
    }
}
