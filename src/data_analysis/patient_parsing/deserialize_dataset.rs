use serde::Serialize;

use crate::data_analysis::{constants::DATA_PATH, patient_parsing::dataset_datatype::{
    fattori_di_rischio::FattoreRischio::{self, *},
    risk_level::Rischio,
    sex::Sex,
    sintomi::Sintomo::{self, *},
}};

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
    fattori_di_rischio: String,
    
    // RISCHIO
    pre_test: usize,  // rischio stimato pre
    post_test: usize, // rischio stimato post

    // FATTORI DI RISCHIO
    diabete_insulino_dipendente: usize,
    diabete_non_insulino_dipendente: usize,
    dislipidemia: usize,
    familiarità: usize,
    fumo: usize,
    ipertensione: usize,
    obesità: usize,
    nessuno: usize,

    // SINTOMI
    angina: usize,
    astenia: usize,
    bruciore_retrosternale: usize,
    cardiopalmo: usize,
    costrizione_giugulare: usize,
    costrizione_mandibolare: usize,
    costrizione_retrosternale: usize,
    dispnea: usize,
    dolore_braccio_sinistro: usize,
    dolore_interscapolare: usize,
    epigastralgie: usize,
    lipotimia: usize,
    malessere: usize,
    no: usize,
    oppressione_epigastrica: usize,
    oppressione_retrosternale: usize,
    precordialgie: usize,
    scompenso_cardiaco: usize,
    sincope: usize,
    toracoalgie: usize,
    vertigini: usize,
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
            match paz.fattori_di_rischio.0.contains(frc) {
                true => 1,
                false => 0,
            }
        }

        fn contains_sintomo(paz: &Patient, sintomo: &Sintomo) -> usize {
            match paz.sintomi.inner().contains(sintomo) {
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
            fattori_di_rischio: paz.fattori_di_rischio.to_string(),

            // RISCHIO INFARTO
            pre_test: match_risk(paz.pre_test),
            post_test: match_risk(paz.post_test),

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



pub fn create_refined_csv(rusty_patients: Vec<Patient>){

    let file_path = DATA_PATH.to_string() + "refined/refined_data.csv"; 
    let mut writer = csv::Writer::from_path(file_path)
    .expect("creating the writer");

    for paz in rusty_patients.into_iter(){
        let paz = OutputSchema::from_patient(paz);
        writer.serialize(paz).expect("wrinting patient");
    }
    writer.flush().expect("flushing");

}