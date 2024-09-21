use std::collections::HashMap;

use serde::Serialize;

use crate::data_analysis::{
    constants::DATA_PATH,
    patient_parsing::dataset_datatype::{
        fattori_di_rischio::FattoreRischio::{self, *},
        patient::Patient,
        risk_level::Rischio,
        sex::Sex,
        sintomi::Sintomo::{self, *},
    },
};

use super::boolean_cell::Bool01;

// Writing a new dataset with elaborated data

#[derive(Serialize)]
pub struct OutputSchema {
    sex: String,
    id: String,
    età: f64,
    kg: f64,
    bmi: f64,
    sintomo: String,
    fattori_di_rischio: String,

    // RISCHIO
    pre_test: String,  // rischio stimato pre
    post_test: String, // rischio stimato post

    // FATTORI DI RISCHIO
    diabete_insulino_dipendente: Bool01,
    diabete_non_insulino_dipendente: Bool01,
    dislipidemia: Bool01,
    familiarità: Bool01,
    fumo: Bool01,
    ipertensione: Bool01,
    obesità: Bool01,
    nessuno: Bool01,

    // SINTOMI
    angina: Bool01,
    astenia: Bool01,
    bruciore_retrosternale: Bool01,
    cardiopalmo: Bool01,
    costrizione_giugulare: Bool01,
    costrizione_mandibolare: Bool01,
    costrizione_retrosternale: Bool01,
    dispnea: Bool01,
    dolore_braccio_sinistro: Bool01,
    dolore_interscapolare: Bool01,
    epigastralgie: Bool01,
    lipotimia: Bool01,
    malessere: Bool01,
    no: Bool01,
    oppressione_epigastrica: Bool01,
    oppressione_retrosternale: Bool01,
    precordialgie: Bool01,
    scompenso_cardiaco: Bool01,
    sincope: Bool01,
    toracoalgie: Bool01,
    vertigini: Bool01,
}

enum StringOrUsize {
    String(String),
    Usize(u8),
}
trait GetValueUsize {
    fn get_value(self) -> u8;
}
trait GetValueString {
    fn get_value(self) -> String;
}
impl GetValueUsize for StringOrUsize {
    fn get_value(self) -> u8 {
        if let Self::Usize(risk_usize) = self {
            risk_usize
        } else {
            panic!("called get_value_usize on StringOrUsize::String which is not meant to be used")
        }
    }
}
impl GetValueString for StringOrUsize {
    fn get_value(self) -> String {
        if let Self::String(risk_string) = self {
            risk_string
        } else {
            panic!("called get_value_string on StringOrUsize::Usize which is not meant to be used")
        }
    }
}

impl OutputSchema {
    pub fn from_patient(paz: Patient, encoded_version: bool) -> Self {
        fn match_risk(risk: Rischio, encoded_version: bool) -> String {
            if encoded_version {
                match risk {
                    Rischio::Alto => "3".to_string(),
                    Rischio::Medio => "2".to_string(),
                    Rischio::Basso => "1".to_string(),
                }
            } else {
                risk.to_string()
            }
        }
        fn match_sex(sex: &Sex) -> String {
            match sex {
                Sex::Male => "Male".to_string(),
                Sex::Femmina => "Female".to_string(),
            }
        }
        fn contains_frc(paz: &Patient, frc: &FattoreRischio) -> Bool01 {
            let is_contained = paz.fattori_di_rischio.0.contains(frc);
            Bool01::from(is_contained)
        }

        fn contains_sintomo(paz: &Patient, sintomo: &Sintomo) -> Bool01 {
            let is_contained = paz.sintomi.inner().contains(sintomo);
            Bool01::from(is_contained)
        }

        OutputSchema {
            sex: match_sex(&paz.sex),
            id: paz.id.to_owned(),
            età: paz.età,
            kg: paz.kg,
            bmi: paz.bmi,
            sintomo: paz.sintomi.inner()[0].to_string(),
            fattori_di_rischio: paz.fattori_di_rischio.to_string(),

            // RISCHIO INFARTO
            pre_test: match_risk(paz.pre_test, encoded_version),
            post_test: match_risk(paz.post_test, encoded_version),

            // FATTORI DI RISCHIO
            diabete_insulino_dipendente: contains_frc(&paz, &DiabeteInsulinoDipendente),
            diabete_non_insulino_dipendente: contains_frc(&paz, &DiabeteNonInsulinoDipendente),
            dislipidemia: contains_frc(&paz, &Dislipidemia),
            familiarità: contains_frc(&paz, &Familiarità),
            fumo: contains_frc(&paz, &Fumo),
            ipertensione: contains_frc(&paz, &Ipertensione),
            obesità: contains_frc(&paz, &Obesità),
            nessuno: contains_frc(&paz, &Nessuno),

            // SINTOMI
            angina: contains_sintomo(&paz, &Angina),
            astenia: contains_sintomo(&paz, &Astenia),
            bruciore_retrosternale: contains_sintomo(&paz, &BrucioreRetrosternale),
            cardiopalmo: contains_sintomo(&paz, &Cardiopalmo),
            costrizione_giugulare: contains_sintomo(&paz, &CostrizioneGiugulare),
            costrizione_mandibolare: contains_sintomo(&paz, &CostrizioneMandibolare),
            costrizione_retrosternale: contains_sintomo(&paz, &CostrizioneRetrosternale),
            dispnea: contains_sintomo(&paz, &Dispnea),
            dolore_braccio_sinistro: contains_sintomo(&paz, &DoloreBraccioSinistro),
            dolore_interscapolare: contains_sintomo(&paz, &DoloreInterscapolare),
            epigastralgie: contains_sintomo(&paz, &Epigastralgie),
            lipotimia: contains_sintomo(&paz, &Lipotimia),
            malessere: contains_sintomo(&paz, &Malessere),
            no: contains_sintomo(&paz, &No),
            oppressione_epigastrica: contains_sintomo(&paz, &OppressionEpigastrica),
            oppressione_retrosternale: contains_sintomo(&paz, &OppressionRetrosternale),
            precordialgie: contains_sintomo(&paz, &Precordialgie),
            scompenso_cardiaco: contains_sintomo(&paz, &ScompensoCardiaco),
            sincope: contains_sintomo(&paz, &Sincope),
            toracoalgie: contains_sintomo(&paz, &Toracoalgie),
            vertigini: contains_sintomo(&paz, &Vertigini),
        }
    }
}

pub fn create_refined_csv(rusty_patients: Vec<Patient>, encoded_version: bool) {
    let file_path = DATA_PATH.to_string()
        + match encoded_version {
            true => "refined/refined_data.csv",
            false => "refined/refined_data_not_encoded.csv",
        };

    let mut writer = csv::Writer::from_path(&file_path).expect("creating the writer");

    for paz in rusty_patients.into_iter() {
        let paz = OutputSchema::from_patient(paz, encoded_version);
        writer.serialize(paz).expect("wrinting patient");
    }
    writer.flush().expect("flushing");

    println!("Succesfully written data on {}", file_path);
}
