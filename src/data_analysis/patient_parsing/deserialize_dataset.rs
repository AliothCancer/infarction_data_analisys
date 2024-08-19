use serde::Serialize;

use crate::data_analysis::patient_parsing::dataset_datatype::{
    fattori_di_rischio::FattoreRischio, risk_level::Rischio, sex::Sex,
};

use super::dataset_datatype::patient::Patient;

// Writing a new dataset with elaborated data

#[derive(Serialize)]
pub struct OutputSchema {
    sex: String,
    id: String,
    età: f64,
    kg: f64,
    bmi: f64,
    sintomo: String,
    pre_test: usize,  // rischio stimato pre
    post_test: usize, // rischio stimato post
    diabete_insulino_dipendente: usize,
    diabete_non_insulino_dipendente: usize,
    dislipidemia: usize,
    familiarità: usize,
    fumo: usize,
    ipertensione: usize,
    obesità: usize,
}
impl OutputSchema {
    pub fn from_patient(paz: Patient) -> Self {
        fn match_risk(risk: Rischio) -> usize {
            match risk {
                Rischio::Alto => 3,
                Rischio::Medio => 2,
                Rischio::Basso => 1,
            }
        }
        fn match_sex(sex: &Sex) -> String {
            match sex {
                Sex::Male => "Male".to_string(),
                Sex::Femmina => "Female".to_string(),
            }
        }
        fn contains_frc(paz: &Patient, frc: &FattoreRischio) -> usize {
            match paz.fattori_di_rischio.0.contains(&frc) {
                true => 1,
                false => 0,
            }
        }

        OutputSchema {
            sex: match_sex(&paz.sex),
            id: paz.id.to_owned(),
            età: paz.età,
            kg: paz.kg,
            bmi: paz.bmi,
            sintomo: paz.sintomi.inner()[0].to_string(),
            pre_test: match_risk(paz.pre_test),
            post_test: match_risk(paz.post_test),
            diabete_insulino_dipendente: contains_frc(
                &paz,
                &FattoreRischio::DiabeteInsulinoDipendente,
            ),
            diabete_non_insulino_dipendente: contains_frc(
                &paz,
                &FattoreRischio::DiabeteNonInsulinoDipendente,
            ),
            dislipidemia: contains_frc(&paz, &FattoreRischio::Dislipidemia),
            familiarità: contains_frc(&paz, &FattoreRischio::Familiarità),
            fumo: contains_frc(&paz, &FattoreRischio::Fumo),
            ipertensione: contains_frc(&paz, &FattoreRischio::Ipertensione),
            obesità: contains_frc(&paz, &FattoreRischio::Obesità),
        }
    }
}
