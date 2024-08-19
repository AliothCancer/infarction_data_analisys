//! Rust data type to represent patient

use super::{
    fattori_di_rischio::FattoriDiRischio, patient_raw::PatientRaw, risk_level::Rischio, sex::Sex,
    sintomi::Sintomi,
};

#[derive(Debug)]
pub struct Patient {
    pub sex: Sex,
    pub id: String,
    pub età: f64,
    pub kg: f64,
    pub bmi: f64,
    pub fattori_di_rischio: FattoriDiRischio,
    pub sintomi: Sintomi,
    //pub ecg: String,
    //pub eco_basale: String,
    //pub eco_stress: String,
    //pub stress_test: String,
    pub pre_test: Rischio,  // rischio stimato pre
    pub post_test: Rischio, // rischio stimato post
}

impl Patient {
    pub fn count_frc(&self) -> u8 {
        self.fattori_di_rischio.count_frc()
    }
    pub fn from_patient_raw(patient_raw: PatientRaw) -> Self {
        let sex =
            Sex::from_string(patient_raw.sex.as_str()).expect("convertendo sex in Sex rust type");
        let id = patient_raw.id;
        let età = patient_raw
            .età
            .parse()
            .expect("provando a convertire età in float");
        let kg = patient_raw.kg.parse().unwrap_or(0.0);
        let bmi = patient_raw.bmi.parse().unwrap_or(0.0);
        let fattori_di_rischio = FattoriDiRischio::from_frc(patient_raw.frc.as_str());
        let sintomi = Sintomi::from_string(patient_raw.sintomi.as_str())
            .expect("provando a convertire sintomi in Sintomi rust type");

        let pre_test = Rischio::from_string(patient_raw.pre_test.as_str())
            .expect("provando a convertire pre_test in Rischio");
        let post_test = Rischio::from_string(patient_raw.post_test.as_str())
            .expect("provando a convertire post_test in Rischio");

        Patient {
            sex,
            id,
            età,
            kg,
            bmi,
            fattori_di_rischio,
            sintomi,
            pre_test,
            post_test,
        }
    }
}
