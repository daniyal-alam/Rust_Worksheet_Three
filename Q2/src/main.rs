/* Write a Rust program that takes a vector of integers as input, squares each
element, and then calculates the sum of the squared values. */
fn main() {
    let numbers = vec![2, 4, 6, 8, 10];

    let square: i32 = numbers.iter().map(|&i| i * i).sum();

    println!("{:?}", numbers.iter().map(|&i| i * i).collect::<Vec<_>>());
    println!("{}", square);
}
