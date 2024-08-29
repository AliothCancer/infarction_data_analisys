use std::collections::HashMap;

pub fn plot_frc_numb_dist_with_subcat() {
    use plotpy::{Barplot, Plot};

    // data
    let data: [(&str, [i32; 7]); 8] = [
        ("familiarità", [0, 214, 326, 302, 155, 52, 24]),
        ("dislipidemia", [0, 82, 300, 360, 219, 69, 24]),
        ("nessuno", [217, 0, 0, 0, 0, 0, 0]),
        ("ipertensione", [0, 309, 530, 485, 250, 71, 24]),
        ("obesità", [0, 9, 41, 85, 93, 49, 24]),
        ("diabete_nid", [0, 11, 75, 127, 108, 51, 22]),
        ("diabete_id", [0, 3, 14, 35, 23, 9, 2]),
        ("fumo", [0, 88, 235, 253, 188, 59, 24]),
    ];

    let mut data_normalized_to_patient_number = [("", [0.0; 7]); 8];
    data.iter().enumerate().for_each(|(n, (frc_name, values))| {
        let mut container = [0.0; 7];
        values
            .iter()
            .enumerate()
            .for_each(|(sub_n, x)| match sub_n {
                //0 | 1 => container[sub_n] = *x as f64, // se nfrc = 0 o 1 non toccare
                _ => container[sub_n] = *x as f64 / values.iter().sum::<i32>() as f64 * 100.0,
            });
        let new_tuple = (*frc_name, container);
        data_normalized_to_patient_number[n] = new_tuple;
    });
    //dbg!(data_normalized_to_patient_number);
    let species = [0, 1, 2, 3, 4, 5, 6];
    let mut sex_counts = HashMap::from(data_normalized_to_patient_number);

    let exclusion_list = [
        //"familiarità",
        //"dislipidemia",
        //"ipertensione",
        "diabete_id",
        "diabete_nid",
        "obesità",
        "fumo",
        //"nessuno"
    ];
    exclusion_list.iter().for_each(|x| {
        sex_counts.remove(x);
    });

    // barplot object and options
    let mut bar = Barplot::new();
    bar.set_with_text("center");

    // draw bars
    let mut bottom = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    for (sex, sex_count) in &sex_counts {
        bar.set_label(sex)
            .set_bottom(&bottom)
            .draw_with_str(&species, sex_count);
        for i in 0..sex_count.len() {
            bottom[i] += sex_count[i] as f64;
        }
    }

    // add barplot to plot and save figure
    let mut plot = Plot::new();
    plot.add(&bar)
        .set_title("Number of risk factor by patient number of risk factor")
        .legend()
        .show("/home/giulio/Scrivania/doc_barplot_2.svg")
        .unwrap();
}
