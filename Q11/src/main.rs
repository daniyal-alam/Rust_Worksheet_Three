/* Write a Rust program that takes user input and appends it to a text file
specified by its path. Handle errors using the Result type and ensure the file is
properly closed. */
use std::fs::OpenOptions;
use std::io::{self, Write};

fn main() -> Result<(), io::Error> {
    let file_path = "output.txt";

    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)?;

    let stdin = io::stdin();
    let mut input = String::new();

    println!("Enter text:");
    loop {
        input.clear();
        stdin.read_line(&mut input)?;

        let mut file = file.lock();
        file.write_all(input.as_bytes())?;
    }
}
