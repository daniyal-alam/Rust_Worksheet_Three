/* Write a Rust function that reads a text file specified by its path and returns the
content as a String. Handle errors using the Result type.
 */
use std::error::Error;
use std::fs;

fn myFunc(file: &str) -> Result<String, Box<dyn Error>> {
    //error trait allow us to capture multiple errors
    let something = fs::read_to_string(file)?;
    Ok(something)
}

fn main() {
    let myfile = "text.txt";

    match myFunc(myfile) {
        Ok(something) => {
            println!("We got: {}", something);
        }
        Err(err) => {
            println!("We got: {}", err);
        }
    }
}
