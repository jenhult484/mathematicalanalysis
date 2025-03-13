// A function that takes a list of numbers as input and returns their sum
fn sum(numbers: &[f64]) -> f64 {
    let mut total = 0.0;
    for number in numbers {
        total += number;
    }
    total
}
