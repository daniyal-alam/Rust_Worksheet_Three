/* Create a generic function that filters elements of a vector based on a provided
predicate function. The function should work for vectors of different types.
 */
fn filter_elements<T, F>(input: Vec<T>, predicate: F) -> Vec<T>
where
    F: Fn(&T) -> bool,
{
    input
        .into_iter()
        .filter(|element| predicate(element))
        .collect()
}

fn main() {
    // Example usage with integers
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let filtered_numbers = filter_elements(numbers, |&x| x % 2 != 0);
    println!("Filtered numbers: {:?}", filtered_numbers);

    // Example usage with strings
    let words = vec!["rust", "programming", "dsa", "networking"];
    let filtered_words = filter_elements(words, |&word| word.len() > 5);
    println!("Filtered words: {:?}", filtered_words);
}
