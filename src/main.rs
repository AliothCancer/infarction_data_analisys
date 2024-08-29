
use infarction_data_analisys::data_analysis::{data_elaborations::frc_frequency_classification, graphs::plot_frc_numb_dist_with_subcat};
use itertools::Itertools;
use prettytable::{cell, row};

fn main(){

}


fn _frc_classifica() {
    
    let datas = frc_frequency_classification();

    //dbg!(datas);

    use prettytable::Table;
    
    let mut table = Table::new();

    table.add_row(row![
        cell!("Fattore di rischio"),
        cell!("Percentuale"),
        cell!("n. pazienti")
    ]);

    for (frc, abs_freq) in datas.iter().sorted_by(|a,b| a.1.cmp(b.1))
        .rev(){
        
        let perc_freq = *abs_freq as f32 / 2599.0 * 100.0;
        let perc_freq_str = format!("{:.2}%",perc_freq);

        table.add_row(
            row![
                cell!(frc), cell!(&perc_freq_str), cell!(&format!("{abs_freq}"))
            ]
        );
        println!("{:.2}% :{}", *abs_freq as f32 / 2599.0 * 100.0, frc)
    }
    //table.print_tty(true).expect("printing to tty");
}

