use super::dataset_datatype::{patient::Patient, patient_raw::PatientRaw};

pub fn read_file() -> Vec<Patient> {
    let csv_path_input = "../dati/ready_to_work_data.csv";
    let _csv_path_output = "../dati/data_refined.csv";

    println!("Reading data from {}", csv_path_input);

    let data = std::fs::read_to_string(csv_path_input);
    let data = match data {
        Ok(inner) => inner,
        Err(e) => panic!("Reading strings: {e}"),
    };

    let mut binding = csv::ReaderBuilder::new()
        .has_headers(true)
        .double_quote(false)
        .from_reader(data.as_bytes());
    let input = binding
        .byte_records()
        //.take(1)
        .flat_map(|x| x.ok())
        .map(|x| x.deserialize::<PatientRaw>(None).unwrap())
        .collect::<Vec<_>>();

    // 1. Trasforma i PatientRaw in Patient
    let rusty_patients = input
        .into_iter()
        .map(Patient::from_patient_raw)
        .collect::<Vec<_>>();

    rusty_patients
}
