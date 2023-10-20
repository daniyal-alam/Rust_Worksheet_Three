/* Create a function that parses a string as an integer. If the parsing fails, return
an Option with None; otherwise, return Some(parsed_integer) */
use std::io;
fn parse_string(input: &str) -> Option<i32> {
    match input.parse() {
        Ok(parsed_integer) => Some(parsed_integer),
        Err(_) => None,
    }
}

fn main() {
    let mut input = String::new();
    println!("Enter string you want to parse:");
    io::stdin().read_line(&mut input).expect("Invalid input.");
    let mut input = input.trim();

    match parse_string(&mut input) {
        Some(parsed_value) => println!("Parsed Integer: {}", parsed_value),
        None => println!("Failed"),
    }
}
