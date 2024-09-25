use std::env;
use std::fs;


mod vec_stats;
mod pig_latin;
mod emp_list;

fn main() {
    //  Vector stats
    let input_vec : Vec<i32> = vec![5, -2, 6, 8, 3, -2, 13, -30];
    let stats_out : (i32, Vec<i32>) = vec_stats::summarize_list(&input_vec);

    println!("Summary of list {:?} was: median {} mode {:?}", 
             input_vec,
             stats_out.0,
             stats_out.1);
    println!("{}", env::current_dir().expect("").into_os_string().into_string().expect(""));

    //  Pig latin conversion
    let contents = fs::read_to_string("src/address.txt").
        expect("Should have been able to read address.txt.");
    let pig_contents = pig_latin::pigify(&contents);
    fs::write("pig_address.txt", pig_contents).
        expect("Unable to write pig_context.txt.");

    //  Employee list interface
    emp_list::menu();
}
