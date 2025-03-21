use std::collections::VecDeque;

fn main() {
    let mut numbers = VecDeque::new();

    // Add some numbers to the vector
    numbers.push_back(1);
    numbers.push_back(2);
    numbers.push_back(3);
    numbers.push_back(4);

    // Print all elements in the vector
    for number in &numbers {
        println!("{}", *number);
    }
}
