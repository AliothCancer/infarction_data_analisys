pub mod data_analysis {

    pub mod constants;

    pub mod graphs;

    pub mod dataset_cleaning;

    pub mod data_elaborations;

    pub mod patient_parsing {
        pub mod serialize_dataset;

        pub mod deserialize_dataset;

        pub mod dataset_datatype {
            pub mod fattori_di_rischio;
            pub mod patient;
            pub mod patient_raw;
            pub mod risk_level;
            pub mod sex;
            pub mod sintomi;
        }
    }
}
