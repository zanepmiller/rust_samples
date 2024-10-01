// Module to consume a list of integers and return median and mode.
// In reference to Rust book 8.3:
//  "Given a list of integers, use a vector and return the median (when sorted, 
//   the value in the middle position) and mode (the value that occurs most 
//   often; a hash map will be helpful here) of the list."

use std::collections::HashMap;

pub fn summarize_list(list : &Vec<i32>) -> (i32, Vec<i32>) {
    let mut ret_val : (i32, Vec<i32>) = (0, Vec::new());
    let mut counts : HashMap<i32, u32> = HashMap::new();
    let mut sorted = list.clone();
    sorted.sort();
    
    //  Calculate median
    let input_len = sorted.len();
    if input_len % 2 == 0 {
        ret_val.0 = (sorted[input_len / 2] + sorted[(input_len / 2) + 1]) / 2;
    } else {
        ret_val.0 = sorted[input_len / 2];
    }

    //  Calculate mode
    //  Setup to do a single pass for O(n)
    let mut max_count : u32 = 0;
    for entry in sorted {
        let count = counts.entry(entry).or_insert(0);
        *count +=1;

        if *count == max_count {
            ret_val.1.push(entry);
        } else if *count > max_count {
            max_count = *count;
            ret_val.1.clear();
            ret_val.1.push(entry);
        }
    }

    ret_val
}