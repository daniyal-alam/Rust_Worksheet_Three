/* Create a function that divides two integers and returns a Result with the result
of the division if the denominator is not zero. If the denominator is zero, return
an error indicating division by zero */
fn divide_integers(numerator: i32, denominator: i32) -> Result<i32, &'static str> {
    if denominator == 0 {
        Err("Division by zero is not allowed")
    } else {
        Ok(numerator / denominator)
    }
}

fn main() {
    let numerator = 10;
    let denominator = 2;

    match divide_integers(numerator, denominator) {
        Ok(result) => println!("Result: {}", result),
        Err(err) => println!("Error: {}", err),
    }

    let numerator = 8;
    let denominator = 0;

    match divide_integers(numerator, denominator) {
        Ok(result) => println!("Result: {}", result),
        Err(err) => println!("Error: {}", err),
    }
}
