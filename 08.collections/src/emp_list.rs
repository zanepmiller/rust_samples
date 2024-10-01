// Module to manage a text interface using hash maps and vectors that allows
// a user to add individuals to departments, and produce an alphabetical list.
// In reference to Rust book 8.3:
//  "Using a hash map and vectors, create a text interface to allow a user to
//   add employee names to a department in a company; for example, “Add Sally to
//   Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of
//   all people in a department or all people in the company by department,
//   sorted alphabetically."

use std::io::{self, Write};
use std::collections::HashMap;

static USAGE : &str = "Add <X> to <Y>: Adds employee named X to department Y.\nList all:       Lists all employees by department.\nList <X>:       Lists all employees in department X.\nMenu:           Prints this menu.\nQuit:           Exits the program.";

//  Entry function to begin dummy interface
pub fn menu() -> () {

    let mut emps : HashMap<String, Vec<String>> = HashMap::new();

    println!("~~~Welcome to the Employee Tracker!~~~\n\n{}", USAGE);

    let mut input = String::new();
    loop {
        input.clear();
        print!("\n> ");
        io::stdout().flush().expect("Could not flush stdout!");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let mut i_line = input.trim().split_whitespace();
        match i_line.next() {
            None => (),
            Some(i) => {
                match &i.to_ascii_lowercase()[..] {
                    "add" => {
                        match parse_add_str(&input) {
                            None => println!("The input \"{}\" was invalid!", &input),
                            Some(a_str) => {
                                match emps.get_mut(&a_str.0) {
                                    None => {
                                        emps.insert(a_str.0, vec![a_str.1]);
                                    },
                                    Some(vec) => {
                                        vec.push(a_str.1);
                                        vec.sort();
                                    }
                                }
                            },
                        }
                    },
                    "list" => {
                        match i_line.next() {
                            None => println!("\"List\" must be followed with 'all' or a department."),
                            Some(n) => {
                                let n = n.to_string();
                                match &n.to_ascii_lowercase()[..] {
                                    "all" => {
                                        let mut emp_keys : Vec<&String> = emps.keys().collect::<Vec<&String>>();
                                        emp_keys.sort();
                                        if emp_keys.len() < 1 {
                                            println!("No employees in tracker; try adding some.");
                                            continue;
                                        }
                                        for key in emp_keys {
                                            println!("{}:", key);
                                            for name in emps.get(key).expect("Invalid key!") {
                                                println!("\t{}", name);
                                            }
                                        }
                                    },
                                    _ => {
                                        match emps.contains_key(&n) {
                                            true => {
                                                println!("{}:", n);
                                                for name in emps.get(&n).expect("Name vector failed!") {
                                                    println!("\t{}", name);
                                                }
                                            },
                                            false => print!("Department name \"{}\" not found!", n.trim())
                                        }
                                    }
                                }
                            }
                        }
                    },
                    "quit" => {
                        println!("Thank you for using Employee Tracker!");
                        break;
                    },
                    "menu" => println!("{}", USAGE),
                    _ => println!("I didn't understand that command."),
                }
            }
        }
    }
}

fn parse_add_str(i_str : &str) -> Option<(String, String)> {
    match i_str[..].find("to") {
        None => return Option::None,
        Some(pos) => {
            if pos < i_str.trim().len() - 4 {
                return Option::Some((i_str[pos+3..].trim().to_string(),
                                     i_str[4..pos].trim().to_string()));
            } else {
                return Option::None;
            }
        },
    }
}