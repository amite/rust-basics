
#![allow(dead_code)]

/*
 * Tasks to complete with strings
 * - print string
 * - print message passed into parameter
 * - get slices of a string
 * - 
 * - concatenate message from parameter
 * - count length of message from parameter
 * - iterate over words in a sentence
 * - iterate over letters in a word
 * - trim a string
 * - replace contents of a string
 * - replace partial contents of a string
 * - uppercase and lowercase a string without mutating it
 * - conditionally output the right string
**/


pub fn print_string() {
    println!("Hello from strings module")
}

pub fn print_msg(msg: &str) -> () {
    println!("{} called from main", msg);
}

pub fn print_msg_string(msg: &String) -> () {
    println!("{} called from main", msg);
}

pub fn calculate_length(s: &str) -> usize {
    s.len()
}

pub fn count_length(msg: &str){
    println!("{} called from main", msg.len());
}

// fn main() -> () {
//     
//     // strings are dynammic so their size is unknown at compile time
//     // String type is a struct with a vec field of u8 // vec: vec<u8>
//
//     let s1 = String::from("hello");
//
//     let len  = calculate_length(&s1);
//
//     println!("The length of '{}' is {}", s1, len);
//
//     let amit = Person { name: "amit", age: 24 };
//     println!("Name is {}", amit.name );
//     println!("{:?}", amit );
//     
//     let story = "Once upon a time...";
//     
//     let ptr = story.as_ptr();
//     let len = story.len();
//
//     println!("{:?}", ptr);
//     println!("{:?}", len);
//     //    display_help();
// }
//
