//! Rust datatype for fattori di rischio

use std::{collections::HashMap, fmt};

use itertools::Itertools;
use serde::Serialize;

use super::patient_raw::match_possible_frc;

#[derive(Debug, Serialize)]
pub struct FattoriDiRischio(pub Vec<FattoreRischio>);

impl FattoriDiRischio {
    pub fn count_frc(&self) -> u8 {
        self.0.len().try_into().unwrap()
    }
    pub fn from_frc(frc: &str) -> Self {
        let mut frc = frc
            .split(",")
            .enumerate()
            .filter_map(|(n, x)| match x {
                "" if n > 0 => None,
                _ => Some(match_possible_frc(x)),
            })
            .map(|x| {
                x.into_iter().map(|x| {
                    //dbg!(x);
                    FattoreRischio::from_string(x.to_lowercase().as_str())
                        .expect("provando a convertire frc in FattoriDiRischio rust type")
                })
            })
            .map(|x| x)
            .flatten()
            .collect::<Vec<FattoreRischio>>();

        if frc.len() > 1 && frc.contains(&FattoreRischio::Nessuno) {
            frc.remove(
                frc.iter()
                    .find_position(|x| x == &&FattoreRischio::Nessuno)
                    .unwrap()
                    .0,
            );
        }
        FattoriDiRischio(frc)
    }
    
    pub(crate) fn to_string(&self) -> String {
        self.0.iter().map(|x| x.to_string()).join(",").into()
    }
}

#[derive(Debug, PartialEq, PartialOrd, Serialize, Hash, Eq)]
pub enum FattoreRischio {
    Nessuno,
    DiabeteInsulinoDipendente,
    DiabeteNonInsulinoDipendente,
    Dislipidemia,
    Familiarità,
    Fumo,
    Ipertensione,
    Obesità,
}

impl FattoreRischio {
    pub fn from_string(s: &str) -> Option<FattoreRischio> {
        match s {
            "nessuno" => Some(FattoreRischio::Nessuno),
            "diabete_id" => Some(FattoreRischio::DiabeteInsulinoDipendente),
            "diabete_nid" => Some(FattoreRischio::DiabeteNonInsulinoDipendente),
            "dislipidemia" => Some(FattoreRischio::Dislipidemia),
            "familiarità" => Some(FattoreRischio::Familiarità),
            "fumo" => Some(FattoreRischio::Fumo),
            "ipertensione" => Some(FattoreRischio::Ipertensione),
            "obesità" => Some(FattoreRischio::Obesità),
            _ => None,
        }
    }
    pub fn generate_hashmap() -> HashMap<String, usize> {
        let mut hs = HashMap::new();
        hs.insert(FattoreRischio::Nessuno.to_string(), 0_usize);
        hs.insert(FattoreRischio::DiabeteInsulinoDipendente.to_string(), 0);
        hs.insert(FattoreRischio::DiabeteNonInsulinoDipendente.to_string(), 0);
        hs.insert(FattoreRischio::Dislipidemia.to_string(), 0);
        hs.insert(FattoreRischio::Familiarità.to_string(), 0);
        hs.insert(FattoreRischio::Fumo.to_string(), 0);
        hs.insert(FattoreRischio::Ipertensione.to_string(), 0);
        hs.insert(FattoreRischio::Obesità.to_string(), 0);

        hs
    }
}

impl fmt::Display for FattoreRischio {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            FattoreRischio::Nessuno => "nessuno",
            FattoreRischio::DiabeteInsulinoDipendente => "diabete_id",
            FattoreRischio::DiabeteNonInsulinoDipendente => "diabete_nid",
            FattoreRischio::Dislipidemia => "dislipidemia",
            FattoreRischio::Familiarità => "familiarità",
            FattoreRischio::Fumo => "fumo",
            FattoreRischio::Ipertensione => "ipertensione",
            FattoreRischio::Obesità => "obesità",
        };
        write!(f, "{}", s)
    }
}
