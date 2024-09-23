use std::io;

mod vec_stats;
mod pig_latin;
mod emp_list;

fn main() {
    let input_vec : Vec<i32> = vec![5, -2, 6, 8, 3, -2, 13, -30];
    let stats_out : (i32, Vec<i32>) = vec_stats::summarize_list(&input_vec);

    println!("Summary of list {:?} was: median {} mode {:?}", 
             input_vec,
             stats_out.0,
             stats_out.1);
}
