#![allow(dead_code)]

use strings::{calculate_length, print_msg, print_msg_string};

mod strings;

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

// strings are all guaranteed to be valid utf-8
fn main() {
    let v: Vec<_> = "I am not not here to inform you...that"
        .match_indices("not")
        .collect();

    // This example demonstrates what happens when matches overlap.
    // The "aba" starting at index 2 is skipped due to the "aba" starting at index 0 being included.
    // The "aba" starting at index 6 is skipped due to the "aba" starting at index 4 being included.
    println!("{:?}", v);
    // println!("{:?}", assert_eq!(v, [(0, "aba"), (4, "aba")]));

    let x = "hello there";
    println!("{}", x);

    strings::print_string();
    let msg = "hello";
    print_msg(&msg);

    let msg = String::from("hey there");
    print_msg_string(&msg);

    let story = "Once upon a time...".to_string();
    let len = calculate_length(&story);
    println!("{}", len);

    display_help();
}

fn display_help() {
    println!("Usage: todo <command> <description> <args>");

    println!();
    println!("Commands: ");

    println!("  add <description> - adds a todo");
    println!("  list <description> - displays all todos");
}
