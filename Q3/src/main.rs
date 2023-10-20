/* Implement a program that reads a text file and counts the frequency of each
word using a HashMap. Ignore punctuation and consider words in a
case-insensitive manner. */
use std::collections::HashMap;
use std::fs;

fn main() {
    let myfile = "text.txt";

    let contents = fs::read_to_string(myfile).expect("Unable to read file.");

    let mut word_count: HashMap<String, u32> = HashMap::new();

    for word in contents.split(|c: char| !c.is_alphanumeric()) {
        if !word.is_empty() {
            let word = word.to_lowercase();
            let word_count = word_count.entry(word).or_insert(0);
            *word_count += 1; // * dereference a mutable reference
        }
    }

    for (word, count) in &word_count {
        println!("{}, {}", word, count);
    }
}
