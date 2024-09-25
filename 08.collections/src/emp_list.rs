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

    let mut emps : HashMap<&str, Vec<&str>> = HashMap::new();

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
                        match input.find("to") {
                            None => println!("Add command must be formatted as Add <X> to <Y>!"),
                            Some(pos) => {
                                match input[pos..].trim().len() {
                                    0 | 1 => println!("Department name must be at least 2 characters!"),
                                    _ => {
                                        let mut name = String::new();
                                        let mut dept = String::new();
                                        &input[4..pos].clone_into(&mut name);
                                        match emps.get_mut(&input[pos+3..]) {
                                            None => _ = emps.insert(&input[pos+3..], vec![&name])
                                                    .expect("Could not insert!"),
                                            Some(list) => list.push(&name),
                                        }
                                    },
                                }
                            }
                        }
                    },
                    "list" => {
                        match i_line.next() {
                            None => println!("\"List\" must be followed with 'all' or a department."),
                            Some(n) => {
                                match &n.to_ascii_lowercase()[..] {
                                    "all" => {
                                        for (key, value) in &emps {
                                            println!("{}:", key);
                                            for name in value {
                                                println!("\t{}", name);
                                            }
                                        }
                                    },
                                    _ => {
                                        match emps.contains_key(n) {
                                            true => {
                                                println!("{}", n);
                                                for name in emps.get(n).expect("Name vector failed!") {
                                                    println!("\t{}", name);
                                                }
                                            },
                                            false => print!("Department name \"{}\" not found!", n)
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