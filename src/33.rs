fn main() {
    // Example 1: Simple addition operation
    let num1 = 5;
    let num2 = 3;
    let sum = num1 + num2;
    println!("The sum of {} and {}", num1, num2);
    
    // Example 2: Addition with a function that takes two parameters
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }
    
    let result = add(5, 3);
    println!("The result of adding {} and {}", 5, 3);
}
