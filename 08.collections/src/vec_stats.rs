// Module to consume a list of integers and return median and mode.
// In reference to Rust book 8.3:
//  "Given a list of integers, use a vector and return the median (when sorted, 
//   the value in the middle position) and mode (the value that occurs most 
//   often; a hash map will be helpful here) of the list."

use std::collections::HashMap;

fn summarize_list(list : &Vec<i32>) -> (i32, i32) {
    let mut counts : HashMap<u32, i32> = HashMap::new();
}