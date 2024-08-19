use std::collections::HashMap;


use itertools::Itertools;

use crate::data_analysis::patient_parsing::{dataset_datatype::fattori_di_rischio::FattoreRischio, serialize_dataset::read_file};




pub fn  frc_numb_dist_with_subcat() -> Vec<(u8, Vec<(String, usize)>)>{

    let mut grouped_data = HashMap::new();
    let mut patients = read_file();

    patients.sort_by(|p1,p2| {
        let p1  = p1.fattori_di_rischio.0.len();
        let p2  = p2.fattori_di_rischio.0.len();

        p1.cmp(&p2)
    });

    let patients = patients
    .iter()
    .chunk_by(|p| p.fattori_di_rischio.0.len());

    let mut paz_tot = 0.0;
    let mut sub_cat = FattoreRischio::generate_hashmap();
    for (id,patient_group) in &patients{

        let id = id as u8;

        for patient in patient_group{

            // counting different frc for this group
            for frc in patient.fattori_di_rischio.0.as_slice(){
                let k = sub_cat.get_mut(&frc.to_string());
                match k {
                    Some(value) => *value += 1,
                    None => {
                        panic!("{frc:?}");
                    }
                }
            }

        }
        let sum = sub_cat.values().sum::<usize>() as f32;
        let n_paz = sum / (id as f32);
        paz_tot += n_paz;
        println!("Gruppo {id}:\n{:?}\n",sub_cat);
        println!("totale paz. = {} / {} = {}\n\n", sum, id, n_paz);
        
        // saving the group data in grouping data
        grouped_data.insert(id, sub_cat.clone());
        // resetting the subcategories counter
        sub_cat.values_mut().for_each(|x|*x=0);
    }
    println!("numero di pazienti contati dai frc: {paz_tot}");
    
    let grouped_data = grouped_data.into_iter().map(|(nfrc,frc_dict)|{
        
        let k = (nfrc,frc_dict.into_iter().collect_vec());

        k
    }).collect_vec();

    grouped_data



}




