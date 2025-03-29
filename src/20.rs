fn main() {
    // Example of using vectors to store numbers
    let numbers: Vec<f64> = (0.1..=10.0).map(|x| x as f64).collect();
    println!("Vectors with floating-point values:");
    for num in &numbers {
        println!("{}", num);
    }
}
