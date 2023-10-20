/* Create a function that takes a sentence as input and converts it to "Title Case"
(capitalize the first letter of each word) while ignoring common articles and
prepositions like "the," "in," "of," etc.
 */
use std::io;
fn to_title_case(sentence: &str) -> String {
    let list = vec!["the", "in", "of", "and", "a", "an", "to", "with"];

    let title_case_sentence = sentence
        .split_whitespace()
        .map(|word| {
            if list.contains(&word.to_lowercase().as_str()) {
                word.to_lowercase()
            } else {
                let mut chars = word.chars();
                match chars.next() {
                    None => String::new(),
                    Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
                }
            }
        })
        .collect::<Vec<String>>()
        .join(" ");

    title_case_sentence
}

fn main() {
    let mut input_sentence = String::new();
    println!("Enter sentence:");
    io::stdin()
        .read_line(&mut input_sentence)
        .expect("Invalid input.");
    let title_case_result = to_title_case(&input_sentence);
    println!("Title Case: {}", title_case_result);
}
