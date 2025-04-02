fn main() {
    // Define a function to calculate the factorial of a number
    fn factorial(n: isize) -> isize {
        if n == 0 || n == 1 {
            1
        } else {
            (n * factorial(n - 1))
        }
    }

    // Main code block where we call the factorial function and print the result
    println!("Factorial of 5 is: {}", factorial(5));
}
