/* Create a Rust function that takes a sentence as input, splits it into words, and
returns a vector of unique words in alphabetical order. */
use std::io;

fn unique_sorted_words(sentence: &str) -> Vec<String> {
    let words_split: Vec<&str> = sentence.split_whitespace().collect();

    let mut unique_words = std::collections::HashSet::new();

    for word in words_split {
        unique_words.insert(word.to_string());
    }

    let mut unique_words_vector: Vec<String> = unique_words.into_iter().collect();

    unique_words_vector.sort();

    unique_words_vector
}

fn main() {
    let mut sentence = String::new();
    println!("Enter sentence:");
    io::stdin()
        .read_line(&mut sentence)
        .expect("Unable to read input.");

    let result = unique_sorted_words(&sentence);

    for word in result {
        println!("{}", word);
    }
}
